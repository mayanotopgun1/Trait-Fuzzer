use quote::ToTokens;
use rand::seq::SliceRandom;
use syn::visit::{self, Visit};
use syn::{GenericParam, Ident, ItemEnum, ItemImpl, ItemStruct, ItemTrait, TraitItem, Type, TypeParamBound, WherePredicate};

use std::collections::{HashMap, HashSet};

// =============================================================================
// TTCG/TCS vocabulary (paper-aligned)
// =============================================================================
/// A constraint *site* is a syntactic location where a new constraint can be injected.
/// Examples: supertrait list, impl where-clause, generic param bounds, assoc type bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstraintSiteKind {
    Supertrait,
    ImplWhere,
    GenericBound,
    AssocBound,
}

/// A constraint site with a stable index (0-based in traversal order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstraintSite {
    pub index: u32,
    pub kind: ConstraintSiteKind,
}

/// The size of a constraint *set* at a site (number of candidate constraints).
///
/// IMPORTANT: constraint sets are *site-local* and *scope-aware*.
/// For example, constraints that mention generic params are only available
/// at sites whose scope contains those params.
pub type ConstraintSetSize = u32;

/// TCS (Trait Constraint Space) across all sites: sum over each site's constraint set size.
/// This is the number of (site x constraint) combinations *after* scope filtering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstraintChoiceSpace {
    pub site_count: u32,
    pub total_choices: u32,
}

#[derive(Debug, Default, Clone)]
pub struct TtcgInfo {
    pub traits: Vec<Ident>,
    pub types: Vec<Ident>,

    // (Type implements Trait)
    pub impl_edges: Vec<(Ident, Ident)>,
    // impl edges where Self is a generic parameter (blanket impls)
    pub impl_edges_blanket: Vec<(Ident, Ident)>,

    // (Trait has supertrait)
    pub supertrait_edges: Vec<(Ident, Ident)>,

    // (Trait declares associated type)
    pub trait_assoc_types: Vec<(Ident, Ident)>,

    // impl Type: assoc bindings like `type Assoc = Ty;` inside `impl Trait for Type`.
    pub impl_assoc_bindings: Vec<ImplAssocBinding>,

    // blanket impl templates like `impl<T> Trait for T` or `impl<T> Trait for Option<T>`.
    pub impl_blanket_templates: Vec<BlanketImplTemplate>,
}

#[derive(Debug, Clone)]
pub struct ImplAssocBinding {
    pub self_ty: Ident,
    pub trait_ident: Ident,
    pub assoc_ident: Ident,
    pub rhs_ty: syn::Type,
}

#[derive(Debug, Clone)]
pub struct BlanketImplTemplate {
    pub self_ty: syn::Type,
    pub trait_ident: Ident,
    pub generic_params: Vec<Ident>,
}

#[derive(Debug, Default, Clone)]
pub struct ConstraintChoiceMetrics {
    /// Count of constraint *sites* in the AST (see ConstraintSiteKind).
    pub constraint_sites: u32,
    /// Sum of constraint-set sizes across all sites.
    /// This is the TCS size: total (site x constraint) combinations after scope filtering.
    pub constraint_choice_sum: u32,
    /// Total number of function arguments available for lifetime mutation (Mutation IV).
    pub lifetime_sites: u32,
    /// Total number of functions available for outlive constraint mutation (Mutation V).
    pub outlive_sites: u32,
}

impl ConstraintChoiceMetrics {
    pub fn choice_space(&self) -> ConstraintChoiceSpace {
        ConstraintChoiceSpace {
            site_count: self.constraint_sites,
            total_choices: self.constraint_choice_sum,
        }
    }
}

impl ConstraintChoiceMetrics {
    fn trait_idents_from_bounds(bounds: &syn::punctuated::Punctuated<TypeParamBound, syn::token::Plus>) -> HashSet<String> {
        bounds
            .iter()
            .filter_map(|b| match b {
                TypeParamBound::Trait(tb) => tb.path.get_ident().map(|id| id.to_string()),
                _ => None,
            })
            .collect()
    }

    fn where_bound_map(generics: &syn::Generics) -> HashMap<String, HashSet<String>> {
        let mut out: HashMap<String, HashSet<String>> = HashMap::new();
        let Some(where_clause) = &generics.where_clause else {
            return out;
        };

        for pred in where_clause.predicates.iter() {
            let WherePredicate::Type(tp) = pred else {
                continue;
            };
            let syn::Type::Path(pty) = &tp.bounded_ty else {
                continue;
            };
            let Some(ty_ident) = pty.path.get_ident() else {
                continue;
            };

            let entry = out.entry(ty_ident.to_string()).or_default();
            for b in tp.bounds.iter() {
                if let TypeParamBound::Trait(tb) = b {
                    if let Some(tr) = tb.path.get_ident() {
                        entry.insert(tr.to_string());
                    }
                }
            }
        }

        out
    }

    fn count_trait_choices(pool: &HashSet<String>, already: &HashSet<String>) -> u32 {
        let n = pool.iter().filter(|t| !already.contains(*t)).count();
        if n == 0 { 1 } else { n as u32 }
    }

    pub fn from_file(ast: &syn::File) -> Self {
        let ttcg = TtcgInfo::from_file(ast);

        // Trait pool used by choose_trait_prefer_custom (custom traits in the file).
        let custom_traits: HashSet<String> = ttcg.traits.iter().map(|t| t.to_string()).collect();

        // Helper for impl-self-type-specific pool: union(custom_traits, matching impl-edge traits).
        let mut impl_edge_traits_by_self: HashMap<String, HashSet<String>> = HashMap::new();
        for (ty, tr) in &ttcg.impl_edges {
            impl_edge_traits_by_self
                .entry(ty.to_string())
                .or_default()
                .insert(tr.to_string());
        }

        struct V<'a> {
            custom_traits: &'a HashSet<String>,
            impl_edge_traits_by_self: &'a HashMap<String, HashSet<String>>,
            out: ConstraintChoiceMetrics,
            in_trait_impl: bool,
        }

        impl<'ast> Visit<'ast> for V<'_> {
            fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
                // Supertrait injection site: pick a trait (prefer custom), excluding self + already-present supertraits.
                self.out.constraint_sites += 1;
                let mut already = ConstraintChoiceMetrics::trait_idents_from_bounds(&i.supertraits);
                already.insert(i.ident.to_string());
                let choices = ConstraintChoiceMetrics::count_trait_choices(self.custom_traits, &already);
                self.out.constraint_choice_sum += choices;

                visit::visit_item_trait(self, i);
            }

            fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
                let old_in_trait = self.in_trait_impl;
                if i.trait_.is_some() {
                    self.in_trait_impl = true;
                }

                // Where-clause injection site: we approximate selectable constraints as the sum of
                // available (TypeParam: Trait) choices over local type params (+ Self when identifiable).
                self.out.constraint_sites += 1;

                let mut total_choices: u32 = 0;
                let where_bounds = ConstraintChoiceMetrics::where_bound_map(&i.generics);

                // Local type parameters in impl generics.
                for p in i.generics.params.iter() {
                    let GenericParam::Type(tp) = p else { continue; };
                    let already = where_bounds.get(&tp.ident.to_string()).cloned().unwrap_or_default();
                    total_choices += ConstraintChoiceMetrics::count_trait_choices(self.custom_traits, &already);
                }

                // Self type (if it is a simple ident path).
                if let Type::Path(tp) = &*i.self_ty {
                    if let Some(self_ident) = tp.path.get_ident() {
                        let mut pool: HashSet<String> = self.custom_traits.clone();
                        if let Some(extra) = self.impl_edge_traits_by_self.get(&self_ident.to_string()) {
                            for t in extra.iter() { pool.insert(t.clone()); }
                        }
                        let already = where_bounds.get(&self_ident.to_string()).cloned().unwrap_or_default();
                        total_choices += ConstraintChoiceMetrics::count_trait_choices(&pool, &already);
                    }
                }

                // If nothing was countable (no type params and no identifiable Self), fall back to 1.
                if total_choices == 0 {
                    total_choices = 1;
                }
                self.out.constraint_choice_sum += total_choices;

                visit::visit_item_impl(self, i);
                self.in_trait_impl = old_in_trait;
            }

            fn visit_generic_param(&mut self, i: &'ast GenericParam) {
                // Generic bound injection site (type params only): count how many traits we could add.
                if let GenericParam::Type(tp) = i {
                    self.out.constraint_sites += 1;
                    let already = ConstraintChoiceMetrics::trait_idents_from_bounds(&tp.bounds);
                    let choices = ConstraintChoiceMetrics::count_trait_choices(self.custom_traits, &already);
                    self.out.constraint_choice_sum += choices;
                }
                visit::visit_generic_param(self, i);
            }

            fn visit_trait_item_type(&mut self, i: &'ast syn::TraitItemType) {
                // Associated type bound injection site.
                self.out.constraint_sites += 1;
                let already = ConstraintChoiceMetrics::trait_idents_from_bounds(&i.bounds);
                let choices = ConstraintChoiceMetrics::count_trait_choices(self.custom_traits, &already);
                self.out.constraint_choice_sum += choices;
            }

            // Mutation IV & V: Count lifetime and outlive sites
            fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
                // Top-level functions: count arguments for lifetime mutations
                self.out.lifetime_sites += i.sig.inputs.len() as u32;
                // Also count the function itself for outlive mutations
                self.out.outlive_sites += 1;
                visit::visit_item_fn(self, i);
            }

            fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
                if !self.in_trait_impl {
                    // Inherent impl methods: count arguments for lifetime mutations
                    self.out.lifetime_sites += i.sig.inputs.len() as u32;
                    // Also count the method itself for outlive mutations
                    self.out.outlive_sites += 1;
                }
                visit::visit_impl_item_fn(self, i);
            }
        }

        let mut v = V {
            custom_traits: &custom_traits,
            impl_edge_traits_by_self: &impl_edge_traits_by_self,
            out: ConstraintChoiceMetrics::default(),
            in_trait_impl: false,
        };
        v.visit_file(ast);
        v.out
    }
}

impl TtcgInfo {
    pub fn from_file(ast: &syn::File) -> Self {
        struct Collector {
            info: TtcgInfo,
        }

        fn impl_self_has_generics(self_ty: &Type, generics: &syn::Generics) -> bool {
            let generic_idents: HashSet<Ident> = generics
                .params
                .iter()
                .filter_map(|p| match p {
                    GenericParam::Type(tp) => Some(tp.ident.clone()),
                    _ => None,
                })
                .collect();
            if generic_idents.is_empty() {
                return false;
            }

            struct V {
                generic_idents: HashSet<Ident>,
                found: bool,
            }

            impl<'ast> Visit<'ast> for V {
                fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
                    if let Some(id) = i.path.get_ident() {
                        if self.generic_idents.contains(id) {
                            self.found = true;
                            return;
                        }
                    }
                    visit::visit_type_path(self, i);
                }
            }

            let mut v = V {
                generic_idents,
                found: false,
            };
            v.visit_type(self_ty);
            v.found
        }

        impl<'ast> Visit<'ast> for Collector {
            fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
                self.info.traits.push(i.ident.clone());

                for item in &i.items {
                    if let TraitItem::Type(assoc) = item {
                        self.info
                            .trait_assoc_types
                            .push((i.ident.clone(), assoc.ident.clone()));
                    }
                }

                for bound in &i.supertraits {
                    if let TypeParamBound::Trait(tb) = bound {
                        if let Some(super_ident) = tb.path.get_ident() {
                            self.info
                                .supertrait_edges
                                .push((i.ident.clone(), super_ident.clone()));
                        }
                    }
                }

                visit::visit_item_trait(self, i);
            }

            fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
                self.info.types.push(i.ident.clone());
                visit::visit_item_struct(self, i);
            }

            fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
                self.info.types.push(i.ident.clone());
                visit::visit_item_enum(self, i);
            }

            fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
                if let Some((_, trait_path, _)) = &i.trait_ {
                    if let Some(trait_ident) = trait_path.get_ident() {
                        let generic_params: Vec<Ident> = i
                            .generics
                            .params
                            .iter()
                            .filter_map(|p| match p {
                                GenericParam::Type(tp) => Some(tp.ident.clone()),
                                _ => None,
                            })
                            .collect();
                        let is_blanket = impl_self_has_generics(&i.self_ty, &i.generics);
                        if is_blanket {
                            self.info.impl_blanket_templates.push(BlanketImplTemplate {
                                self_ty: (*i.self_ty).clone(),
                                trait_ident: trait_ident.clone(),
                                generic_params,
                            });
                        }
                        if let Type::Path(self_ty) = &*i.self_ty {
                            if let Some(type_ident) = self_ty.path.get_ident() {
                                let is_generic_self = is_blanket;
                                self.info
                                    .impl_edges
                                    .push((type_ident.clone(), trait_ident.clone()));
                                if is_generic_self {
                                    self.info
                                        .impl_edges_blanket
                                        .push((type_ident.clone(), trait_ident.clone()));
                                }

                                for impl_item in &i.items {
                                    if let syn::ImplItem::Type(assoc) = impl_item {
                                        self.info.impl_assoc_bindings.push(ImplAssocBinding {
                                            self_ty: type_ident.clone(),
                                            trait_ident: trait_ident.clone(),
                                            assoc_ident: assoc.ident.clone(),
                                            rhs_ty: assoc.ty.clone(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }

                visit::visit_item_impl(self, i);
            }
        }

        let mut c = Collector {
            info: TtcgInfo::default(),
        };
        c.visit_file(ast);

        // Keep lists stable and reasonably unique.
        c.info
            .traits
            .sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        c.info.traits.dedup_by(|a, b| a == b);
        c.info
            .types
            .sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        c.info.types.dedup_by(|a, b| a == b);
        c.info.impl_edges.sort_by(|(ta, tr_a), (tb, tr_b)| {
            (ta.to_string(), tr_a.to_string()).cmp(&(tb.to_string(), tr_b.to_string()))
        });
        c.info.impl_edges.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
        c.info.impl_edges_blanket.sort_by(|(ta, tr_a), (tb, tr_b)| {
            (ta.to_string(), tr_a.to_string()).cmp(&(tb.to_string(), tr_b.to_string()))
        });
        c.info.impl_edges_blanket.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
        c.info.impl_blanket_templates.sort_by(|a, b| {
            (
                a.self_ty.to_token_stream().to_string(),
                a.trait_ident.to_string(),
            )
                .cmp(&(b.self_ty.to_token_stream().to_string(), b.trait_ident.to_string()))
        });
        c.info.impl_blanket_templates.dedup_by(|a, b| {
            a.self_ty.to_token_stream().to_string() == b.self_ty.to_token_stream().to_string()
                && a.trait_ident == b.trait_ident
        });
        c.info.supertrait_edges.sort_by(|(t1, s1), (t2, s2)| {
            (t1.to_string(), s1.to_string()).cmp(&(t2.to_string(), s2.to_string()))
        });
        c.info.supertrait_edges.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

        c.info.trait_assoc_types.sort_by(|(t1, a1), (t2, a2)| {
            (t1.to_string(), a1.to_string()).cmp(&(t2.to_string(), a2.to_string()))
        });
        c.info
            .trait_assoc_types
            .dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

        c.info.impl_assoc_bindings.sort_by(|a, b| {
            (
                a.self_ty.to_string(),
                a.trait_ident.to_string(),
                a.assoc_ident.to_string(),
                a.rhs_ty.to_token_stream().to_string(),
            )
                .cmp(&(
                    b.self_ty.to_string(),
                    b.trait_ident.to_string(),
                    b.assoc_ident.to_string(),
                    b.rhs_ty.to_token_stream().to_string(),
                ))
        });
        c.info.impl_assoc_bindings.dedup_by(|a, b| {
            a.self_ty == b.self_ty
                && a.trait_ident == b.trait_ident
                && a.assoc_ident == b.assoc_ident
                && a.rhs_ty.to_token_stream().to_string() == b.rhs_ty.to_token_stream().to_string()
        });

        c.info
    }

    pub fn any_constraint_pair(&self) -> Option<(Ident, Ident)> {
        let mut rng = rand::thread_rng();
        self.impl_edges.choose(&mut rng).cloned()
    }
}
