// =========================================================================
// PROJECTION REWRITE MUTATION (MutationⅢ)
// =========================================================================
use super::framework::Mutator;
use quote::ToTokens;
use rand::Rng;
use std::collections::HashMap;
use syn::visit::{self, Visit};
use syn::visit_mut::{self, VisitMut};
use syn::{parse_quote, Ident, Type};

use crate::ttcg::{ImplAssocBinding, TtcgInfo};
use serde::Serialize;

pub struct ProjectionRewriteMutator;

#[derive(Debug, Default, Clone, Copy)]
pub struct ProjectionChoiceMetrics {
    pub rewrite_sites: u32,
    pub rewrite_choice_sum: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectionSiteDebug {
    pub index: usize,
    pub label: String,
    pub candidates: Vec<String>,
}

#[derive(Clone)]
struct ProjectionCandidate {
    ty: Type,
    self_ty: Ident,
    trait_ident: Ident,
    assoc_ident: Ident,
}

fn normalize_tokens<T: ToTokens>(node: &T) -> String {
    node.to_token_stream().to_string().split_whitespace().collect()
}

fn build_replacement_map(ttcg: &TtcgInfo) -> HashMap<String, Vec<ProjectionCandidate>> {
    let mut map: HashMap<String, Vec<ProjectionCandidate>> = HashMap::new();
    for ImplAssocBinding { self_ty, trait_ident, assoc_ident, rhs_ty } in &ttcg.impl_assoc_bindings {
        let key = normalize_tokens(rhs_ty);
        let proj: Type = parse_quote!(<#self_ty as #trait_ident>::#assoc_ident);
        map.entry(key).or_default().push(ProjectionCandidate {
            ty: proj,
            self_ty: self_ty.clone(),
            trait_ident: trait_ident.clone(),
            assoc_ident: assoc_ident.clone(),
        });
    }
    map
}

struct ProjectionRewriteCollector {
    count: usize,
    map: HashMap<String, Vec<ProjectionCandidate>>,
    current_impl_self: Option<Ident>,
    current_impl_trait: Option<Ident>,
    current_impl_assoc: Option<Ident>,
}

struct ProjectionSiteCollector {
    map: HashMap<String, Vec<ProjectionCandidate>>,
    current_impl_self: Option<Ident>,
    current_impl_trait: Option<Ident>,
    current_impl_assoc: Option<Ident>,
    out: Vec<ProjectionSiteDebug>,
    index: usize,
}

struct ProjectionChoiceCollector {
    metrics: ProjectionChoiceMetrics,
    map: HashMap<String, Vec<ProjectionCandidate>>,
    current_impl_self: Option<Ident>,
    current_impl_trait: Option<Ident>,
    current_impl_assoc: Option<Ident>,
}

#[derive(Clone, Copy)]
struct RewriteChoiceEntry {
    site_index: usize,
    local_index: usize,
    local_count: usize,
}

impl<'ast> Visit<'ast> for ProjectionRewriteCollector {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let prev_self = self.current_impl_self.clone();
        let prev_trait = self.current_impl_trait.clone();
        if let Some((_, trait_path, _)) = &i.trait_ {
            if let Some(trait_ident) = trait_path.get_ident() {
                if let Type::Path(self_ty) = &*i.self_ty {
                    if let Some(type_ident) = self_ty.path.get_ident() {
                        self.current_impl_self = Some(type_ident.clone());
                        self.current_impl_trait = Some(trait_ident.clone());
                    }
                }
            }
        }

        visit::visit_item_impl(self, i);
        self.current_impl_self = prev_self;
        self.current_impl_trait = prev_trait;
    }

    fn visit_impl_item_type(&mut self, i: &'ast syn::ImplItemType) {
        let prev_assoc = self.current_impl_assoc.clone();
        self.current_impl_assoc = Some(i.ident.clone());
        visit::visit_impl_item_type(self, i);
        self.current_impl_assoc = prev_assoc;
    }

    fn visit_type(&mut self, i: &'ast Type) {
        if let Type::Path(tp) = i {
            if tp.qself.is_none() {
                let key = normalize_tokens(i);
                if self.map.contains_key(&key) {
                    let has_non_self = self
                        .map
                        .get(&key)
                        .map(|cands| {
                            cands.iter().any(|c| {
                                let same_self = self.current_impl_self.as_ref().is_some_and(|s| s == &c.self_ty);
                                let same_trait = self.current_impl_trait.as_ref().is_some_and(|t| t == &c.trait_ident);
                                let same_assoc = self.current_impl_assoc.as_ref().is_some_and(|a| a == &c.assoc_ident);
                                !(same_self && same_trait && same_assoc)
                            })
                        })
                        .unwrap_or(false);
                    if has_non_self {
                        self.count += 1;
                    }
                }
            }
        }
        visit::visit_type(self, i);
    }
}

impl<'ast> Visit<'ast> for ProjectionSiteCollector {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let prev_self = self.current_impl_self.clone();
        let prev_trait = self.current_impl_trait.clone();
        if let Some((_, trait_path, _)) = &i.trait_ {
            if let Some(trait_ident) = trait_path.get_ident() {
                if let Type::Path(self_ty) = &*i.self_ty {
                    if let Some(type_ident) = self_ty.path.get_ident() {
                        self.current_impl_self = Some(type_ident.clone());
                        self.current_impl_trait = Some(trait_ident.clone());
                    }
                }
            }
        }

        visit::visit_item_impl(self, i);
        self.current_impl_self = prev_self;
        self.current_impl_trait = prev_trait;
    }

    fn visit_impl_item_type(&mut self, i: &'ast syn::ImplItemType) {
        let prev_assoc = self.current_impl_assoc.clone();
        self.current_impl_assoc = Some(i.ident.clone());
        visit::visit_impl_item_type(self, i);
        self.current_impl_assoc = prev_assoc;
    }

    fn visit_type(&mut self, i: &'ast Type) {
        if let Type::Path(tp) = i {
            if tp.qself.is_none() {
                let key = normalize_tokens(i);
                if let Some(cands) = self.map.get(&key) {
                    let filtered: Vec<&ProjectionCandidate> = cands
                        .iter()
                        .filter(|c| {
                            let same_self = self.current_impl_self.as_ref().is_some_and(|s| s == &c.self_ty);
                            let same_trait = self.current_impl_trait.as_ref().is_some_and(|t| t == &c.trait_ident);
                            let same_assoc = self.current_impl_assoc.as_ref().is_some_and(|a| a == &c.assoc_ident);
                            !(same_self && same_trait && same_assoc)
                        })
                        .collect();
                    if !filtered.is_empty() {
                        let label = format!(
                            "type {} in impl {} for {}",
                            normalize_tokens(i),
                            self.current_impl_trait
                                .as_ref()
                                .map(|t| t.to_string())
                                .unwrap_or_else(|| "?".to_string()),
                            self.current_impl_self
                                .as_ref()
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "?".to_string()),
                        );
                        let candidates = filtered
                            .into_iter()
                            .map(|c| normalize_tokens(&c.ty))
                            .collect();
                        self.out.push(ProjectionSiteDebug {
                            index: self.index,
                            label,
                            candidates,
                        });
                        self.index += 1;
                    }
                }
            }
        }
        visit::visit_type(self, i);
    }
}

impl<'ast> Visit<'ast> for ProjectionChoiceCollector {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let prev_self = self.current_impl_self.clone();
        let prev_trait = self.current_impl_trait.clone();
        if let Some((_, trait_path, _)) = &i.trait_ {
            if let Some(trait_ident) = trait_path.get_ident() {
                if let Type::Path(self_ty) = &*i.self_ty {
                    if let Some(type_ident) = self_ty.path.get_ident() {
                        self.current_impl_self = Some(type_ident.clone());
                        self.current_impl_trait = Some(trait_ident.clone());
                    }
                }
            }
        }

        visit::visit_item_impl(self, i);
        self.current_impl_self = prev_self;
        self.current_impl_trait = prev_trait;
    }

    fn visit_impl_item_type(&mut self, i: &'ast syn::ImplItemType) {
        let prev_assoc = self.current_impl_assoc.clone();
        self.current_impl_assoc = Some(i.ident.clone());
        visit::visit_impl_item_type(self, i);
        self.current_impl_assoc = prev_assoc;
    }

    fn visit_type(&mut self, i: &'ast Type) {
        if let Type::Path(tp) = i {
            if tp.qself.is_none() {
                let key = normalize_tokens(i);
                if let Some(cands) = self.map.get(&key) {
                    let filtered: Vec<&ProjectionCandidate> = cands
                        .iter()
                        .filter(|c| {
                            let same_self = self.current_impl_self.as_ref().is_some_and(|s| s == &c.self_ty);
                            let same_trait = self.current_impl_trait.as_ref().is_some_and(|t| t == &c.trait_ident);
                            let same_assoc = self.current_impl_assoc.as_ref().is_some_and(|a| a == &c.assoc_ident);
                            !(same_self && same_trait && same_assoc)
                        })
                        .collect();
                    let n = filtered.len() as u32;
                    if n > 0 {
                        self.metrics.rewrite_sites += 1;
                        self.metrics.rewrite_choice_sum += n;
                    }
                }
            }
        }
        visit::visit_type(self, i);
    }
}

struct ProjectionRewriteApplier {
    target: usize,
    current: usize,
    mutated: bool,
    map: HashMap<String, Vec<ProjectionCandidate>>,
    current_impl_self: Option<Ident>,
    current_impl_trait: Option<Ident>,
    current_impl_assoc: Option<Ident>,
    choice_index: Option<usize>,
    choice_count: usize,
    chosen_choice_index: usize,
}

impl VisitMut for ProjectionRewriteApplier {
    fn visit_item_impl_mut(&mut self, i: &mut syn::ItemImpl) {
        let prev_self = self.current_impl_self.clone();
        let prev_trait = self.current_impl_trait.clone();
        if let Some((_, trait_path, _)) = &i.trait_ {
            if let Some(trait_ident) = trait_path.get_ident() {
                if let Type::Path(self_ty) = &*i.self_ty {
                    if let Some(type_ident) = self_ty.path.get_ident() {
                        self.current_impl_self = Some(type_ident.clone());
                        self.current_impl_trait = Some(trait_ident.clone());
                    }
                }
            }
        }

        visit_mut::visit_item_impl_mut(self, i);
        self.current_impl_self = prev_self;
        self.current_impl_trait = prev_trait;
    }

    fn visit_impl_item_type_mut(&mut self, i: &mut syn::ImplItemType) {
        let prev_assoc = self.current_impl_assoc.clone();
        self.current_impl_assoc = Some(i.ident.clone());
        visit_mut::visit_impl_item_type_mut(self, i);
        self.current_impl_assoc = prev_assoc;
    }

    fn visit_type_mut(&mut self, i: &mut Type) {
        if self.mutated {
            return;
        }
        if let Type::Path(tp) = i {
            if tp.qself.is_none() {
                let key = normalize_tokens(i);
                if let Some(candidates) = self.map.get(&key) {
                    if self.current == self.target {
                        let mut rng = rand::thread_rng();
                        let filtered: Vec<&ProjectionCandidate> = candidates
                            .iter()
                            .filter(|c| {
                                let same_self = self.current_impl_self.as_ref().is_some_and(|s| s == &c.self_ty);
                                let same_trait = self.current_impl_trait.as_ref().is_some_and(|t| t == &c.trait_ident);
                                let same_assoc = self.current_impl_assoc.as_ref().is_some_and(|a| a == &c.assoc_ident);
                                !(same_self && same_trait && same_assoc)
                            })
                            .collect();
                        self.choice_count = filtered.len();
                        if !filtered.is_empty() {
                            let idx = self.choice_index.unwrap_or_else(|| rng.gen_range(0..filtered.len())) % filtered.len();
                            self.chosen_choice_index = idx;
                            if let Some(replacement) = filtered.get(idx) {
                                *i = replacement.ty.clone();
                                self.mutated = true;
                            }
                        }
                    }
                    self.current += 1;
                }
            }
        }
        visit_mut::visit_type_mut(self, i);
    }
}

impl Mutator for ProjectionRewriteMutator {
    fn collect(&mut self, ast: &syn::File) -> usize {
        let ttcg = TtcgInfo::from_file(ast);
        let map = build_replacement_map(&ttcg);
        if map.is_empty() {
            return 0;
        }
        let mut c = ProjectionRewriteCollector {
            count: 0,
            map,
            current_impl_self: None,
            current_impl_trait: None,
            current_impl_assoc: None,
        };
        c.visit_file(ast);
        c.count
    }

    fn mutate(&mut self, ast: &mut syn::File, index: usize) -> bool {
        let ttcg = TtcgInfo::from_file(ast);
        let map = build_replacement_map(&ttcg);
        if map.is_empty() {
            return false;
        }
        let mut a = ProjectionRewriteApplier {
            target: index,
            current: 0,
            mutated: false,
            map,
            current_impl_self: None,
            current_impl_trait: None,
            current_impl_assoc: None,
            choice_index: None,
            choice_count: 0,
            chosen_choice_index: 0,
        };
        a.visit_file_mut(ast);
        a.mutated
    }
}

impl ProjectionRewriteMutator {
    pub fn projection_choice_metrics(ast: &syn::File) -> ProjectionChoiceMetrics {
        let ttcg = TtcgInfo::from_file(ast);
        let map = build_replacement_map(&ttcg);
        if map.is_empty() {
            return ProjectionChoiceMetrics::default();
        }
        let mut c = ProjectionChoiceCollector {
            metrics: ProjectionChoiceMetrics::default(),
            map,
            current_impl_self: None,
            current_impl_trait: None,
            current_impl_assoc: None,
        };
        c.visit_file(ast);
        c.metrics
    }

    pub fn collect_sites_with_candidates(ast: &syn::File) -> Vec<ProjectionSiteDebug> {
        let ttcg = TtcgInfo::from_file(ast);
        let map = build_replacement_map(&ttcg);
        if map.is_empty() {
            return Vec::new();
        }
        let mut c = ProjectionSiteCollector {
            map,
            current_impl_self: None,
            current_impl_trait: None,
            current_impl_assoc: None,
            out: Vec::new(),
            index: 0,
        };
        c.visit_file(ast);
        c.out
    }

    fn collect_choice_entries(ast: &syn::File) -> Vec<RewriteChoiceEntry> {
        let ttcg = TtcgInfo::from_file(ast);
        let map = build_replacement_map(&ttcg);
        if map.is_empty() {
            return Vec::new();
        }

        struct V {
            map: HashMap<String, Vec<ProjectionCandidate>>,
            current_impl_self: Option<Ident>,
            current_impl_trait: Option<Ident>,
            current_impl_assoc: Option<Ident>,
            out: Vec<RewriteChoiceEntry>,
            site_index: usize,
        }

        impl<'ast> Visit<'ast> for V {
            fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
                let prev_self = self.current_impl_self.clone();
                let prev_trait = self.current_impl_trait.clone();
                if let Some((_, trait_path, _)) = &i.trait_ {
                    if let Some(trait_ident) = trait_path.get_ident() {
                        if let Type::Path(self_ty) = &*i.self_ty {
                            if let Some(type_ident) = self_ty.path.get_ident() {
                                self.current_impl_self = Some(type_ident.clone());
                                self.current_impl_trait = Some(trait_ident.clone());
                            }
                        }
                    }
                }

                visit::visit_item_impl(self, i);
                self.current_impl_self = prev_self;
                self.current_impl_trait = prev_trait;
            }

            fn visit_impl_item_type(&mut self, i: &'ast syn::ImplItemType) {
                let prev_assoc = self.current_impl_assoc.clone();
                self.current_impl_assoc = Some(i.ident.clone());
                visit::visit_impl_item_type(self, i);
                self.current_impl_assoc = prev_assoc;
            }

            fn visit_type(&mut self, i: &'ast Type) {
                if let Type::Path(tp) = i {
                    if tp.qself.is_none() {
                        let key = normalize_tokens(i);
                        if let Some(cands) = self.map.get(&key) {
                            let filtered: Vec<&ProjectionCandidate> = cands
                                .iter()
                                .filter(|c| {
                                    let same_self = self.current_impl_self.as_ref().is_some_and(|s| s == &c.self_ty);
                                    let same_trait = self.current_impl_trait.as_ref().is_some_and(|t| t == &c.trait_ident);
                                    let same_assoc = self.current_impl_assoc.as_ref().is_some_and(|a| a == &c.assoc_ident);
                                    !(same_self && same_trait && same_assoc)
                                })
                                .collect();
                            let local_count = filtered.len();
                            if local_count > 0 {
                                for idx in 0..local_count {
                                    self.out.push(RewriteChoiceEntry {
                                        site_index: self.site_index,
                                        local_index: idx,
                                        local_count,
                                    });
                                }
                                self.site_index += 1;
                            }
                        }
                    }
                }
                visit::visit_type(self, i);
            }
        }

        let mut v = V {
            map,
            current_impl_self: None,
            current_impl_trait: None,
            current_impl_assoc: None,
            out: Vec::new(),
            site_index: 0,
        };
        v.visit_file(ast);
        v.out
    }

    pub fn run_with_meta_and_choice(
        ast: &mut syn::File,
        forced_index: Option<usize>,
        choice_index: Option<usize>,
    ) -> (bool, usize, usize, usize, usize) {
        let mut m = ProjectionRewriteMutator;
        let site_count = m.collect(ast);
        if site_count == 0 {
            return (false, 0, 0, 0, 0);
        }

        let flat = Self::collect_choice_entries(ast);
        let choice_count = flat.len();
        if choice_count == 0 {
            return (false, 0, site_count, 0, 0);
        }

        let idx = match choice_index.or(forced_index) {
            Some(i) if i < choice_count => i,
            _ => {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..choice_count)
            }
        };

        let entry = flat[idx];
        let ttcg = TtcgInfo::from_file(&*ast);
        let map = build_replacement_map(&ttcg);
        if map.is_empty() {
            return (false, entry.site_index, site_count, choice_count, idx);
        }

        let mut a = ProjectionRewriteApplier {
            target: entry.site_index,
            current: 0,
            mutated: false,
            map,
            current_impl_self: None,
            current_impl_trait: None,
            current_impl_assoc: None,
            choice_index: Some(entry.local_index),
            choice_count: entry.local_count,
            chosen_choice_index: entry.local_index,
        };
        a.visit_file_mut(ast);
        (a.mutated, entry.site_index, site_count, choice_count, idx)
    }
}