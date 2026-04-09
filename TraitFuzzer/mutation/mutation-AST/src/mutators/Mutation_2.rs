// =========================================================================
// TRAIT-CONSTRAINT MUTATION (Mutation II)
// =========================================================================
use super::framework::Mutator;
use quote::ToTokens;
use rand::seq::SliceRandom;
use rand::Rng;
use syn::visit::{self, Visit};
use syn::visit_mut::{self, VisitMut};
use syn::{parse_quote, GenericParam, Ident, ItemImpl, ItemTrait, TraitItemType, Type, TypeParamBound, WherePredicate};
use crate::ttcg::{BlanketImplTemplate, ImplAssocBinding, TtcgInfo};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
// Guard: avoid traversing pathological ASTs that cause stack overflow.
const MAX_CONSTRAINT_SITES: usize = 5000;

// =========================================================================
// Trait-Constraint Mutation (TTCG/TCS-guided)
// =========================================================================
//
// We extract a TTCG (Trait-Type Constraint Graph) from the current seed:
// - Nodes: Type and Trait symbols discovered in the AST.
// - Edges: impl edges (Type -> Trait) and projection edges
//   (<Type as Trait>::Assoc -> Trait), which represent possible constraints.
//
// Extraction source: TTCG (crate::ttcg) gives us:
// - traits/types
// - impl_edges: (Type, Trait)
// - trait_assoc_types: (Trait, Assoc)
// - impl_assoc_bindings: (Type, Trait, Assoc)
//
// Injection sites ("constraint sites"):
// - where clauses (impl generics)
// - generic type parameter bounds
// - trait supertraits (inheritance bounds)
// - associated type bounds
//
// Constraint set:
// - For each site, we compute a *constraint set* = the list of candidate constraints
//   applicable at that site *under the current scope* (e.g., only in-scope generics).
// - The total (site x constraint) combinations define the TCS
//   (Trait Constraint Space) used by this mutator.
//
// Scope rules (important for correctness):
// - If we inject a constraint on a generic parameter, it must be in scope.
// - If we inject a constraint on a concrete type, scope is not restrictive.
// - For projection bounds, prefer concrete impl assoc bindings when available.
pub struct ConstraintInjectionMutator;

#[derive(Debug, Clone, Serialize)]
pub struct ConstraintSiteDebug {
    pub index: usize,
    pub kind: String,
    pub label: String,
    pub candidates: Vec<String>,
}

struct ConstraintInjectionCollector {
    count: usize,
}

impl<'ast> Visit<'ast> for ConstraintInjectionCollector {
    fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
        self.count += 1; // supertrait injection point
        if self.count >= MAX_CONSTRAINT_SITES {
            return; // stop traversing deeper to avoid stack blowup
        }
        visit::visit_item_trait(self, i);
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        self.count += 1; // where injection point
        if self.count >= MAX_CONSTRAINT_SITES {
            return;
        }
        visit::visit_item_impl(self, i);
    }

    fn visit_generic_param(&mut self, i: &'ast GenericParam) {
        if matches!(i, GenericParam::Type(_)) {
            self.count += 1; // generic bound injection point
            if self.count >= MAX_CONSTRAINT_SITES {
                return;
            }
        }
        visit::visit_generic_param(self, i);
    }

    fn visit_trait_item_type(&mut self, _: &'ast TraitItemType) {
        self.count += 1; // associated type bound injection point
        if self.count >= MAX_CONSTRAINT_SITES {
            return;
        }
    }
}

struct ConstraintInjectionApplier {
    target: usize,
    current: usize,
    mutated: bool,
    ttcg: TtcgInfo,
    constraint_index: Option<usize>,
    constraint_count: usize,
    chosen_constraint_index: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SiteKind {
    Supertrait,
    Where,
    GenericBound,
    AssocBound,
}

#[derive(Clone)]
struct ChoiceEntry {
    site_index: usize,
    local_index: usize,
    kind: SiteKind,
    local_count: usize,
}

impl ConstraintInjectionApplier {
    fn fallback_trait() -> Ident {
        syn::Ident::new("Copy", proc_macro2::Span::call_site())
    }

    fn choose_trait_prefer_custom(&self, excluded: &[Ident]) -> Ident {
        // Prefer traits defined in the current file (TTCG traits), avoid obvious duplicates.
        let mut rng = rand::thread_rng();
        let candidates: Vec<Ident> = self
            .ttcg
            .traits
            .iter()
            .filter(|t| !excluded.iter().any(|e| e == *t))
            .cloned()
            .collect();

        candidates
            .choose(&mut rng)
            .cloned()
            .unwrap_or_else(Self::fallback_trait)
    }

    fn local_type_params(generics: &syn::Generics) -> Vec<Ident> {
        generics
            .params
            .iter()
            .filter_map(|p| match p {
                GenericParam::Type(tp) => Some(tp.ident.clone()),
                _ => None,
            })
            .collect()
    }

    fn pick_local_param_predicate(
        &self,
        generics: &syn::Generics,
        excluded_traits: &[Ident],
    ) -> Option<WherePredicate> {
        let mut rng = rand::thread_rng();
        let local_params = Self::local_type_params(generics);
        let tp = local_params.choose(&mut rng)?;
        let trait_ident = self.choose_trait_prefer_custom(excluded_traits);
        Some(parse_quote!(#tp: #trait_ident))
    }

    fn pick_self_predicate(&self, self_ty: &Ident, excluded_traits: &[Ident]) -> Option<WherePredicate> {
        let mut rng = rand::thread_rng();
        let matching: Vec<Ident> = self
            .ttcg
            .impl_edges
            .iter()
            .filter(|(ty, _)| ty == self_ty)
            .map(|(_, tr)| tr.clone())
            .collect();

        if let Some(tr) = matching.choose(&mut rng) {
            return Some(parse_quote!(#self_ty: #tr));
        }

        let trait_ident = self.choose_trait_prefer_custom(excluded_traits);
        Some(parse_quote!(#self_ty: #trait_ident))
    }

    fn pick_any_impl_edge_predicate(&self) -> Option<WherePredicate> {
        let (type_ident, trait_ident) = self.ttcg.any_constraint_pair()?;
        Some(parse_quote!(#type_ident: #trait_ident))
    }

    fn normalize_tokens<T: ToTokens>(node: &T) -> String {
        node.to_token_stream().to_string().split_whitespace().collect()
    }

    fn type_key(ty: &Type) -> String {
        Self::normalize_tokens(ty)
    }

    fn substitute_type(ty: &Type, subst: &HashMap<Ident, Type>) -> Type {
        let mut out = ty.clone();
        struct V<'a> {
            subst: &'a HashMap<Ident, Type>,
        }
        impl<'a> VisitMut for V<'a> {
            fn visit_type_mut(&mut self, i: &mut Type) {
                if let Type::Path(tp) = i {
                    if tp.qself.is_none() {
                        if let Some(id) = tp.path.get_ident() {
                            if let Some(rep) = self.subst.get(id) {
                                *i = rep.clone();
                                return;
                            }
                        }
                    }
                }
                visit_mut::visit_type_mut(self, i);
            }
        }
        let mut v = V { subst };
        v.visit_type_mut(&mut out);
        out
    }

    fn expand_blanket_template(
        &self,
        template: &BlanketImplTemplate,
        base_types: &[Type],
    ) -> Vec<Type> {
        if template.generic_params.is_empty() {
            return vec![template.self_ty.clone()];
        }

        let mut out: Vec<Type> = Vec::new();
        let mut subst: HashMap<Ident, Type> = HashMap::new();

        fn rec(
            idx: usize,
            params: &[Ident],
            base: &[Type],
            template: &BlanketImplTemplate,
            subst: &mut HashMap<Ident, Type>,
            out: &mut Vec<Type>,
        ) {
            if idx >= params.len() {
                out.push(ConstraintInjectionApplier::substitute_type(&template.self_ty, subst));
                return;
            }
            for ty in base {
                subst.insert(params[idx].clone(), ty.clone());
                rec(idx + 1, params, base, template, subst, out);
            }
            subst.remove(&params[idx]);
        }

        rec(0, &template.generic_params, base_types, template, &mut subst, &mut out);
        out
    }

    fn dedup_bounds(bounds: Vec<TypeParamBound>) -> Vec<TypeParamBound> {
        let mut out: Vec<TypeParamBound> = Vec::new();
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        for b in bounds {
            let key = Self::normalize_tokens(&b);
            if seen.insert(key) {
                out.push(b);
            }
        }
        out
    }

    fn dedup_preds(preds: Vec<WherePredicate>) -> Vec<WherePredicate> {
        let mut out: Vec<WherePredicate> = Vec::new();
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        for p in preds {
            let key = Self::normalize_tokens(&p);
            if seen.insert(key) {
                out.push(p);
            }
        }
        out
    }

    fn has_where_predicate(generics: &syn::Generics, predicate: &WherePredicate) -> bool {
        let Some(where_clause) = &generics.where_clause else {
            return false;
        };

        let needle = Self::normalize_tokens(predicate);
        where_clause
            .predicates
            .iter()
            .any(|p| Self::normalize_tokens(p) == needle)
    }

    fn choose_where_predicate_for_generics(
        &self,
        generics: &syn::Generics,
        self_ty_ident: Option<&Ident>,
        excluded_traits: &[Ident],
    ) -> Option<WherePredicate> {
        // TTCG rule: prefer constraints that are valid in the *current scope*.
        // 1) Prefer binding a local generic parameter.
        if let Some(pred) = self.pick_local_param_predicate(generics, excluded_traits) {
            return Some(pred);
        }

        // 2) In impl context, prefer constraining the impl self type.
        if let Some(self_ty) = self_ty_ident {
            return self.pick_self_predicate(self_ty, excluded_traits);
        }

        // 3) Otherwise, fall back to any existing (Type implements Trait) edge.
        if let Some(pred) = self.pick_any_impl_edge_predicate() {
            return Some(pred);
        }

        // 4) Final fallback: concrete type + a (preferably custom) trait.
        let trait_ident = self.choose_trait_prefer_custom(excluded_traits);
        let type_ident = self.ttcg.types.first().cloned()?;
        Some(parse_quote!(#type_ident: #trait_ident))
    }

    fn choose_projection_where_predicate(&self, prefer_self_ty: Option<&Ident>) -> Option<WherePredicate> {
        // TTCG rule: a projection constraint exists if we have
        //   - an impl binding (concrete), or
        //   - a trait assoc declaration + an impl edge to pick a concrete type.
        let mut rng = rand::thread_rng();
        // 1) Prefer concrete assoc bindings from impl blocks.
        let candidates: Vec<&ImplAssocBinding> = match prefer_self_ty {
            Some(self_ty) => self
                .ttcg
                .impl_assoc_bindings
                .iter()
                .filter(|b| &b.self_ty == self_ty)
                .collect(),
            None => Vec::new(),
        };

        let pool: Vec<&ImplAssocBinding> = if !candidates.is_empty() {
            candidates
        } else {
            self.ttcg.impl_assoc_bindings.iter().collect()
        };

        if let Some(binding) = pool.choose(&mut rng).copied() {
            let bound_trait = self.choose_trait_prefer_custom(&[]);
            let self_ty = &binding.self_ty;
            let tr = &binding.trait_ident;
            let assoc = &binding.assoc_ident;
            return Some(parse_quote!(<#self_ty as #tr>::#assoc: #bound_trait));
        }

        // 2) If there is no concrete binding, still allow a projection *bound* guided by:
        //    - trait-associated-type declarations
        //    - an existing (Type implements Trait) edge
        // This keeps the mutation useful even when assoc types are only defaulted.
        let (tr, assoc) = self.ttcg.trait_assoc_types.choose(&mut rng).cloned()?;
        let impl_types_for_trait: Vec<Ident> = self
            .ttcg
            .impl_edges
            .iter()
            .filter(|(_, tr2)| *tr2 == tr)
            .map(|(ty, _)| ty.clone())
            .collect();

        let self_ty = impl_types_for_trait.choose(&mut rng)?.clone();
        let bound_trait = self.choose_trait_prefer_custom(&[tr.clone()]);
        Some(parse_quote!(<#self_ty as #tr>::#assoc: #bound_trait))
    }


    fn choose_impl_where_predicate(&self, i: &ItemImpl) -> Option<WherePredicate> {
        // TTCG rule: in impl scope, prefer local generic params or Self.
        // Avoid injecting constraints that mention out-of-scope type params.
        // In real-world Rust, impl where-clauses mostly constrain local generic params and/or Self.
        // But predicates about other in-scope concrete types also exist (less common). We'll inject
        // those occasionally to increase diversity.
        const PROJECTION_PRED_PROB: f64 = 0.35;
        const UNRELATED_IMPL_EDGE_PROB: f64 = 0.25;

        let mut rng = rand::thread_rng();

        let self_ty_ident = match &*i.self_ty {
            Type::Path(tp) => tp.path.get_ident(),
            _ => None,
        };

        // 0) Occasionally inject a projection constraint to exercise normalization:
        //    where <Type as Trait>::Assoc: Bound
        if rng.gen_bool(PROJECTION_PRED_PROB) {
            if let Some(pred) = self.choose_projection_where_predicate(self_ty_ident) {
                return Some(pred);
            }
        }

        // 1) Prefer binding a local generic parameter.
        if let Some(pred) = self.pick_local_param_predicate(&i.generics, &[]) {
            return Some(pred);
        }

        // 2) Occasionally inject an "unrelated" predicate: OtherType: OtherTrait (from TTCG impl edges).
        if rng.gen_bool(UNRELATED_IMPL_EDGE_PROB) {
            let unrelated_edges: Vec<(Ident, Ident)> = self
                .ttcg
                .impl_edges
                .iter()
                .filter(|(ty, _)| self_ty_ident.map(|s| s != ty).unwrap_or(true))
                .cloned()
                .collect();

            if let Some((ty, tr)) = unrelated_edges.choose(&mut rng) {
                return Some(parse_quote!(#ty: #tr));
            }
        }

        // 3) Otherwise, constrain Self if possible (usually the most meaningful).
        self.choose_where_predicate_for_generics(&i.generics, self_ty_ident, &[])
    }

    fn has_bound(bounds: &syn::punctuated::Punctuated<TypeParamBound, syn::token::Plus>, trait_ident: &Ident) -> bool {
        bounds.iter().any(|b| match b {
            TypeParamBound::Trait(tb) => tb.path.get_ident().is_some_and(|id| id == trait_ident),
            _ => false,
        })
    }

    fn has_supertrait(i: &ItemTrait, trait_ident: &Ident) -> bool {
        i.supertraits.iter().any(|b| match b {
            TypeParamBound::Trait(tb) => tb.path.get_ident().is_some_and(|id| id == trait_ident),
            _ => false,
        })
    }

    fn supertrait_candidates(&self, i: &ItemTrait) -> Vec<TypeParamBound> {
        let mut out: Vec<TypeParamBound> = Vec::new();
        for tr in &self.ttcg.traits {
            if tr == &i.ident {
                continue;
            }
            if !Self::has_supertrait(i, tr) {
                out.push(parse_quote!(#tr));
            }
        }
        Self::dedup_bounds(out)
    }

    fn generic_bound_candidates(&self, tp: &syn::TypeParam) -> Vec<TypeParamBound> {
        let mut out: Vec<TypeParamBound> = Vec::new();
        for tr in &self.ttcg.traits {
            if !Self::has_bound(&tp.bounds, tr) {
                out.push(parse_quote!(#tr));
            }
        }
        Self::dedup_bounds(out)
    }

    fn assoc_bound_candidates(&self, i: &TraitItemType) -> Vec<TypeParamBound> {
        let mut out: Vec<TypeParamBound> = Vec::new();
        for tr in &self.ttcg.traits {
            if !Self::has_bound(&i.bounds, tr) {
                out.push(parse_quote!(#tr));
            }
        }
        Self::dedup_bounds(out)
    }

    fn projection_bound_candidates(
        &self,
        in_scope: &[Ident],
        concrete_types: &[Type],
        impl_traits_by_type: &HashMap<String, HashSet<Ident>>,
        blanket_traits_on_generic_self: &HashSet<Ident>,
    ) -> Vec<WherePredicate> {
        let mut out: Vec<WherePredicate> = Vec::new();
        // 1) Concrete assoc bindings always allowed (self_ty is concrete).
        for b in &self.ttcg.impl_assoc_bindings {
            let self_ty = &b.self_ty;
            let tr = &b.trait_ident;
            let assoc = &b.assoc_ident;
            for bound_tr in &self.ttcg.traits {
                out.push(parse_quote!(<#self_ty as #tr>::#assoc: #bound_tr));
            }
        }

        // 2) Full-type expansion for assoc types declared on traits:
        for (tr, assoc) in &self.ttcg.trait_assoc_types {
            // Concrete types: allow if trait is implemented (direct or blanket).
            for ty in concrete_types {
                let key = Self::type_key(ty);
                let mut ok = false;
                if let Some(pool) = impl_traits_by_type.get(&key) {
                    ok = pool.contains(tr);
                }
                if ok {
                    for bound_tr in &self.ttcg.traits {
                        out.push(parse_quote!(<#ty as #tr>::#assoc: #bound_tr));
                    }
                }
            }

            // In-scope generics: only allow if trait is blanket-implemented.
            if blanket_traits_on_generic_self.contains(tr) {
                for ty in in_scope {
                    for bound_tr in &self.ttcg.traits {
                        out.push(parse_quote!(<#ty as #tr>::#assoc: #bound_tr));
                    }
                }
            }
        }

        Self::dedup_preds(out)
    }

    fn where_predicate_candidates(&self, i: &ItemImpl) -> Vec<WherePredicate> {
        let mut out: Vec<WherePredicate> = Vec::new();

        let mut in_scope: Vec<Ident> = Vec::new();
        let mut base_concrete_types: Vec<Type> = Vec::new();
        for ty in &self.ttcg.types {
            base_concrete_types.push(parse_quote!(#ty));
        }
        for (ty, _) in &self.ttcg.impl_edges {
            base_concrete_types.push(parse_quote!(#ty));
        }
        for b in &self.ttcg.impl_assoc_bindings {
            let ty = &b.self_ty;
            base_concrete_types.push(parse_quote!(#ty));
        }
        // Dedup base concrete types
        base_concrete_types.sort_by(|a, b| Self::type_key(a).cmp(&Self::type_key(b)));
        base_concrete_types.dedup_by(|a, b| Self::type_key(a) == Self::type_key(b));

        let mut impl_traits_by_type: HashMap<String, HashSet<Ident>> = HashMap::new();
        let mut type_by_key: HashMap<String, Type> = HashMap::new();
        let mut blanket_traits_on_generic_self: HashSet<Ident> = HashSet::new();

        let mut add_type_trait = |ty: &Type, tr: &Ident| {
            let key = Self::type_key(ty);
            type_by_key.entry(key.clone()).or_insert_with(|| ty.clone());
            impl_traits_by_type.entry(key).or_default().insert(tr.clone());
        };

        for (ty, tr) in &self.ttcg.impl_edges {
            let t: Type = parse_quote!(#ty);
            add_type_trait(&t, tr);
        }

        for tpl in &self.ttcg.impl_blanket_templates {
            // If self_ty is exactly a generic param, allow T: Trait in-scope.
            if let Type::Path(tp) = &tpl.self_ty {
                if tp.qself.is_none() {
                    if let Some(id) = tp.path.get_ident() {
                        if tpl.generic_params.iter().any(|p| p == id) {
                            blanket_traits_on_generic_self.insert(tpl.trait_ident.clone());
                        }
                    }
                }
            }
            let expanded = self.expand_blanket_template(tpl, &base_concrete_types);
            for ty in expanded {
                add_type_trait(&ty, &tpl.trait_ident);
            }
        }

        let mut concrete_types: Vec<Type> = type_by_key.values().cloned().collect();
        concrete_types.sort_by(|a, b| Self::type_key(a).cmp(&Self::type_key(b)));

        // Local generic params: T: Trait
        for p in i.generics.params.iter() {
            let GenericParam::Type(tp) = p else { continue; };
            in_scope.push(tp.ident.clone());
            let mut trait_pool: HashSet<Ident> = HashSet::new();
            let tp_ident = &tp.ident;
            let tp_ty: Type = parse_quote!(#tp_ident);
            if let Some(trs) = impl_traits_by_type.get(&Self::type_key(&tp_ty)) {
                for tr in trs {
                    trait_pool.insert(tr.clone());
                }
            }
            for tr in &blanket_traits_on_generic_self {
                trait_pool.insert(tr.clone());
            }
            for tr in trait_pool {
                if !Self::has_bound(&tp.bounds, &tr) {
                    let tp_ident = &tp.ident;
                    out.push(parse_quote!(#tp_ident: #tr));
                }
            }
        }

        // Self type constraints
        let self_ty_ident = match &*i.self_ty {
            Type::Path(tp) => tp.path.get_ident(),
            _ => None,
        };
        if let Some(self_ty) = self_ty_ident {
            in_scope.push(self_ty.clone());
            let mut pool: HashSet<Ident> = HashSet::new();
            let self_ty_type: Type = parse_quote!(#self_ty);
            if let Some(trs) = impl_traits_by_type.get(&Self::type_key(&self_ty_type)) {
                for tr in trs {
                    pool.insert(tr.clone());
                }
            }
            for tr in pool {
                out.push(parse_quote!(#self_ty: #tr));
            }
        }

        // Concrete type constraints from impl edges + blanket expansion
        for ty in &concrete_types {
            let mut pool: HashSet<Ident> = HashSet::new();
            if let Some(trs) = impl_traits_by_type.get(&Self::type_key(ty)) {
                for tr in trs {
                    pool.insert(tr.clone());
                }
            }
            for tr in pool {
                out.push(parse_quote!(#ty: #tr));
            }
        }

        // Projection bounds (full-type expansion + scope-aware generics)
        out.extend(self.projection_bound_candidates(
            &in_scope,
            &concrete_types,
            &impl_traits_by_type,
            &blanket_traits_on_generic_self,
        ));

        let dedup = Self::dedup_preds(out);
        // Filter out predicates already present in the where-clause to avoid no-op picks.
        dedup
            .into_iter()
            .filter(|p| !Self::has_where_predicate(&i.generics, p))
            .collect()
    }

    fn inject_supertrait(&mut self, i: &mut ItemTrait) -> bool {
        let candidates = self.supertrait_candidates(i);
        if candidates.is_empty() {
            self.constraint_count = 0;
            return false;
        }
        self.constraint_count = candidates.len();
        let idx = self.constraint_index.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..candidates.len())
        }) % candidates.len();
        self.chosen_constraint_index = idx;
        i.supertraits.push(candidates[idx].clone());
        true
    }

    fn inject_where_predicate(&mut self, i: &mut ItemImpl) -> bool {
        let candidates = self.where_predicate_candidates(i);
        if candidates.is_empty() {
            self.constraint_count = 0;
            return false;
        }
        self.constraint_count = candidates.len();
        let idx = self.constraint_index.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..candidates.len())
        }) % candidates.len();
        self.chosen_constraint_index = idx;
        let pred = candidates[idx].clone();
        if !Self::has_where_predicate(&i.generics, &pred) {
            i.generics.make_where_clause().predicates.push(pred);
            return true;
        }
        false
    }

    fn inject_generic_bound(&mut self, tp: &mut syn::TypeParam) -> bool {
        let candidates = self.generic_bound_candidates(tp);
        if candidates.is_empty() {
            self.constraint_count = 0;
            return false;
        }
        self.constraint_count = candidates.len();
        let idx = self.constraint_index.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..candidates.len())
        }) % candidates.len();
        self.chosen_constraint_index = idx;
        tp.bounds.push(candidates[idx].clone());
        true
    }

    fn inject_assoc_bound(&mut self, i: &mut TraitItemType) -> bool {
        let candidates = self.assoc_bound_candidates(i);
        if candidates.is_empty() {
            self.constraint_count = 0;
            return false;
        }
        self.constraint_count = candidates.len();
        let idx = self.constraint_index.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..candidates.len())
        }) % candidates.len();
        self.chosen_constraint_index = idx;
        i.bounds.push(candidates[idx].clone());
        true
    }
}

fn collect_sites_with_candidates_for_choice(applier: &ConstraintInjectionApplier, ast: &syn::File) -> Vec<Vec<ChoiceEntry>> {
    struct V<'a> {
        applier: &'a ConstraintInjectionApplier,
        out: Vec<Vec<ChoiceEntry>>,
        site_index: usize,
    }

    impl<'ast> Visit<'ast> for V<'_> {
        fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
            let cands = self.applier.supertrait_candidates(i);
            let local_count = cands.len();
            if self.site_index >= MAX_CONSTRAINT_SITES {
                return;
            }
            let mut entries = Vec::new();
            for idx in 0..local_count {
                entries.push(ChoiceEntry {
                    site_index: self.site_index,
                    local_index: idx,
                    kind: SiteKind::Supertrait,
                    local_count,
                });
            }
            self.out.push(entries);
            self.site_index += 1;
            visit::visit_item_trait(self, i);
        }

        fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
            let cands = self.applier.where_predicate_candidates(i);
            let local_count = cands.len();
            if self.site_index >= MAX_CONSTRAINT_SITES {
                return;
            }
            let mut entries = Vec::new();
            for idx in 0..local_count {
                entries.push(ChoiceEntry {
                    site_index: self.site_index,
                    local_index: idx,
                    kind: SiteKind::Where,
                    local_count,
                });
            }
            self.out.push(entries);
            self.site_index += 1;
            visit::visit_item_impl(self, i);
        }

        fn visit_generic_param(&mut self, i: &'ast GenericParam) {
            if let GenericParam::Type(tp) = i {
                let cands = self.applier.generic_bound_candidates(tp);
                let local_count = cands.len();
                if self.site_index >= MAX_CONSTRAINT_SITES {
                    return;
                }
                let mut entries = Vec::new();
                for idx in 0..local_count {
                    entries.push(ChoiceEntry {
                        site_index: self.site_index,
                        local_index: idx,
                        kind: SiteKind::GenericBound,
                        local_count,
                    });
                }
                self.out.push(entries);
                self.site_index += 1;
            }
            visit::visit_generic_param(self, i);
        }

        fn visit_trait_item_type(&mut self, i: &'ast TraitItemType) {
            let cands = self.applier.assoc_bound_candidates(i);
            let local_count = cands.len();
            if self.site_index >= MAX_CONSTRAINT_SITES {
                return;
            }
            let mut entries = Vec::new();
            for idx in 0..local_count {
                entries.push(ChoiceEntry {
                    site_index: self.site_index,
                    local_index: idx,
                    kind: SiteKind::AssocBound,
                    local_count,
                });
            }
            self.out.push(entries);
            self.site_index += 1;
        }
    }

    let mut v = V {
        applier,
        out: Vec::new(),
        site_index: 0,
    };
    v.visit_file(ast);
    v.out
}

impl ConstraintInjectionMutator {
    pub fn collect_sites_with_candidates(ast: &syn::File) -> Vec<ConstraintSiteDebug> {
        let ttcg = TtcgInfo::from_file(ast);
        let a = ConstraintInjectionApplier {
            target: 0,
            current: 0,
            mutated: false,
            ttcg,
            constraint_index: None,
            constraint_count: 0,
            chosen_constraint_index: 0,
        };

        struct V {
            applier: ConstraintInjectionApplier,
            out: Vec<ConstraintSiteDebug>,
            index: usize,
            current_trait: Option<Ident>,
            current_impl_label: Option<String>,
        }

        impl<'ast> Visit<'ast> for V {
            fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
                let prev_trait = self.current_trait.clone();
                self.current_trait = Some(i.ident.clone());
                let cands = self.applier.supertrait_candidates(i);
                let candidates = cands
                    .into_iter()
                    .map(|b| ConstraintInjectionApplier::normalize_tokens(&b))
                    .collect();
                self.out.push(ConstraintSiteDebug {
                    index: self.index,
                    kind: "supertrait".to_string(),
                    label: format!("trait {}", i.ident),
                    candidates,
                });
                self.index += 1;
                visit::visit_item_trait(self, i);
                self.current_trait = prev_trait;
            }

            fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
                let prev_impl = self.current_impl_label.clone();
                let self_ty = i.self_ty.to_token_stream().to_string();
                let label = if let Some((_, trait_path, _)) = &i.trait_ {
                    format!("impl {} for {}", trait_path.to_token_stream(), self_ty)
                } else {
                    format!("impl {}", self_ty)
                };
                self.current_impl_label = Some(label.clone());
                let cands = self.applier.where_predicate_candidates(i);
                let candidates = cands
                    .into_iter()
                    .map(|p| ConstraintInjectionApplier::normalize_tokens(&p))
                    .collect();
                self.out.push(ConstraintSiteDebug {
                    index: self.index,
                    kind: "where".to_string(),
                    label,
                    candidates,
                });
                self.index += 1;
                visit::visit_item_impl(self, i);
                self.current_impl_label = prev_impl;
            }

            fn visit_generic_param(&mut self, i: &'ast GenericParam) {
                if let GenericParam::Type(tp) = i {
                    let cands = self.applier.generic_bound_candidates(tp);
                    let candidates = cands
                        .into_iter()
                        .map(|b| ConstraintInjectionApplier::normalize_tokens(&b))
                        .collect();
                    let mut label = format!("param {}", tp.ident);
                    if let Some(ctx) = &self.current_impl_label {
                        label = format!("{} in {}", label, ctx);
                    } else if let Some(tr) = &self.current_trait {
                        label = format!("{} in trait {}", label, tr);
                    }
                    self.out.push(ConstraintSiteDebug {
                        index: self.index,
                        kind: "generic_bound".to_string(),
                        label,
                        candidates,
                    });
                    self.index += 1;
                }
                visit::visit_generic_param(self, i);
            }

            fn visit_trait_item_type(&mut self, i: &'ast TraitItemType) {
                let cands = self.applier.assoc_bound_candidates(i);
                let candidates = cands
                    .into_iter()
                    .map(|b| ConstraintInjectionApplier::normalize_tokens(&b))
                    .collect();
                let label = if let Some(tr) = &self.current_trait {
                    format!("{}::{}", tr, i.ident)
                } else {
                    format!("assoc {}", i.ident)
                };
                self.out.push(ConstraintSiteDebug {
                    index: self.index,
                    kind: "assoc_bound".to_string(),
                    label,
                    candidates,
                });
                self.index += 1;
            }
        }

        let mut v = V {
            applier: a,
            out: Vec::new(),
            index: 0,
            current_trait: None,
            current_impl_label: None,
        };
        v.visit_file(ast);
        v.out
    }
}

impl VisitMut for ConstraintInjectionApplier {
    fn visit_item_trait_mut(&mut self, i: &mut ItemTrait) {
        // supertrait (inheritance) site
        if self.current == self.target {
            self.mutated = self.inject_supertrait(i) || self.mutated;
        }
        self.current += 1;

        visit_mut::visit_item_trait_mut(self, i);
    }

    fn visit_item_impl_mut(&mut self, i: &mut ItemImpl) {
        if self.current == self.target {
            self.mutated = self.inject_where_predicate(i) || self.mutated;
        }
        self.current += 1;
        visit_mut::visit_item_impl_mut(self, i);
    }

    fn visit_generic_param_mut(&mut self, i: &mut GenericParam) {
        if let GenericParam::Type(tp) = i {
            if self.current == self.target {
                self.mutated = self.inject_generic_bound(tp) || self.mutated;
            }
            self.current += 1;
        }
        visit_mut::visit_generic_param_mut(self, i);
    }

    fn visit_trait_item_type_mut(&mut self, i: &mut TraitItemType) {
        if self.current == self.target {
            self.mutated = self.inject_assoc_bound(i) || self.mutated;
        }
        self.current += 1;
        visit_mut::visit_trait_item_type_mut(self, i);
    }
}

impl Mutator for ConstraintInjectionMutator {
    fn collect(&mut self, ast: &syn::File) -> usize {
        let mut c = ConstraintInjectionCollector { count: 0 };
        c.visit_file(ast);
        c.count
    }

    fn mutate(&mut self, ast: &mut syn::File, index: usize) -> bool {
        let ttcg = TtcgInfo::from_file(&*ast);
        let mut a = ConstraintInjectionApplier {
            target: index,
            current: 0,
            mutated: false,
            ttcg,
            constraint_index: None,
            constraint_count: 0,
            chosen_constraint_index: 0,
        };
        a.visit_file_mut(ast);
        a.mutated
    }
}

impl ConstraintInjectionMutator {
    pub fn run_with_meta_and_constraint(
        ast: &mut syn::File,
        forced_index: Option<usize>,
        constraint_index: Option<usize>,
    ) -> (bool, usize, usize, usize, usize) {
        let mut m = ConstraintInjectionMutator;
        let site_count = m.collect(ast);
        if site_count == 0 {
            return (false, 0, 0, 0, 0);
        }

        let ttcg = TtcgInfo::from_file(&*ast);
        let a = ConstraintInjectionApplier {
            target: 0,
            current: 0,
            mutated: false,
            ttcg,
            constraint_index: None,
            constraint_count: 0,
            chosen_constraint_index: 0,
        };

        let sites = collect_sites_with_candidates_for_choice(&a, ast);
        let flat: Vec<ChoiceEntry> = sites.into_iter().flatten().collect();
        let choice_count = flat.len();
        // Safety: avoid attempting to index into an enormous choice space which
        // may be produced by pathological or generated inputs and cause deep recursion
        // or extreme memory/stack usage. Bail out early in that case.
        if choice_count > MAX_CONSTRAINT_SITES * 100 {
            return (false, 0, site_count, 0, 0);
        }
        if choice_count == 0 {
            return (false, 0, site_count, 0, 0);
        }

        let choice_index = match constraint_index.or(forced_index) {
            Some(i) if i < choice_count => i,
            _ => {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..choice_count)
            }
        };

        let entry = flat[choice_index].clone();
        let mut applier = ConstraintInjectionApplier {
            target: entry.site_index,
            current: 0,
            mutated: false,
            ttcg: a.ttcg,
            constraint_index: Some(entry.local_index),
            constraint_count: entry.local_count,
            chosen_constraint_index: entry.local_index,
        };
        applier.visit_file_mut(ast);
        (
            applier.mutated,
            entry.site_index,
            site_count,
            choice_count,
            choice_index,
        )
    }

    pub fn run_with_meta_supertrait(
        ast: &mut syn::File,
        forced_index: Option<usize>,
        constraint_index: Option<usize>,
    ) -> (bool, usize, usize, usize, usize) {
        let mut m = ConstraintInjectionMutator;
        let site_count = m.collect(ast);
        if site_count == 0 {
            return (false, 0, 0, 0, 0);
        }

        let ttcg = TtcgInfo::from_file(&*ast);
        let a = ConstraintInjectionApplier {
            target: 0,
            current: 0,
            mutated: false,
            ttcg,
            constraint_index: None,
            constraint_count: 0,
            chosen_constraint_index: 0,
        };

        let sites = collect_sites_with_candidates_for_choice(&a, ast);
        let flat: Vec<ChoiceEntry> = sites
            .into_iter()
            .flatten()
            .filter(|e| e.kind == SiteKind::Supertrait)
            .collect();
        let choice_count = flat.len();
        if choice_count > MAX_CONSTRAINT_SITES * 100 {
            return (false, 0, site_count, 0, 0);
        }
        if choice_count == 0 {
            return (false, 0, site_count, 0, 0);
        }

        let choice_index = match constraint_index.or(forced_index) {
            Some(i) if i < choice_count => i,
            _ => {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..choice_count)
            }
        };

        let entry = flat[choice_index].clone();
        let mut applier = ConstraintInjectionApplier {
            target: entry.site_index,
            current: 0,
            mutated: false,
            ttcg: a.ttcg,
            constraint_index: Some(entry.local_index),
            constraint_count: entry.local_count,
            chosen_constraint_index: entry.local_index,
        };
        applier.visit_file_mut(ast);
        (
            applier.mutated,
            entry.site_index,
            site_count,
            choice_count,
            choice_index,
        )
    }
}