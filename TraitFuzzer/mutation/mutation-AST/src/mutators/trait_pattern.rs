use rand::{Rng, seq::SliceRandom};
use syn::parse_quote;
use syn::{GenericParam, Ident, ImplItem, ItemTrait, TraitItem, TraitItemFn, TraitItemConst, Type};

use crate::ttcg::TtcgInfo;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraitPattern {
	Basic,
	AssocType,
	Gat,
	AssocTypeAndConst,
    ConstGeneric,
}

pub fn choose_pattern() -> TraitPattern {
	let mut rng = rand::thread_rng();
	match rng.gen_range(0..5) {
		0 => TraitPattern::Basic,
		1 => TraitPattern::AssocType,
		2 => TraitPattern::Gat,
		3 => TraitPattern::AssocTypeAndConst,
        _ => TraitPattern::ConstGeneric,
	}
}

pub fn build_trait(ident: &Ident, pattern: TraitPattern) -> ItemTrait {
	match pattern {
		TraitPattern::Basic => parse_quote!(trait #ident {}),
		TraitPattern::AssocType => parse_quote!(trait #ident { type Assoc; }),
		TraitPattern::Gat => parse_quote!(trait #ident { type Item<'a>; }),
		TraitPattern::AssocTypeAndConst => parse_quote! {
			trait #ident {
				type Assoc;
				const C: Self::Assoc;
				fn f(&self) -> Self::Assoc;
			}
		},
        TraitPattern::ConstGeneric => parse_quote!(trait #ident <const N: usize> {}),
	}
}

pub fn infer_pattern(tr: &ItemTrait) -> TraitPattern {
	let mut has_assoc = false;
	let mut has_gat = false;
	let mut has_const = false;

	for it in &tr.items {
		match it {
			TraitItem::Type(t) => {
				if !t.generics.params.is_empty() {
					has_gat = true;
				} else {
					has_assoc = true;
				}
			}
			TraitItem::Const(_) => {
				has_const = true;
			}
			_ => {}
		}
	}

	if has_const {
		TraitPattern::AssocTypeAndConst
	} else if has_gat {
		TraitPattern::Gat
	} else if has_assoc {
		TraitPattern::AssocType
	} else {
		TraitPattern::Basic
	}
}

pub fn make_impl_items_for_trait(tr: &ItemTrait, ttcg: &TtcgInfo) -> Vec<ImplItem> {
	let mut out: Vec<ImplItem> = Vec::new();

	// Check if we have const items. If so, we restrict associated types to primitives
	// so we can easily generate valid const values.
	let has_const = tr.items.iter().any(|it| matches!(it, TraitItem::Const(_)));

	// If we need to pick a primitive type for the associated type, decide it once here
	// so that if the trait has multiple items relating to it, they stay consistent?
	// Actually, the current template `AssocTypeAndConst` only has one assoc type `Assoc`.
	// But `build_trait` is hardcoded. If we had multiple types, we might need a map.
	// For now, let's just pick a random primitive for *each* associated type if `has_const` is true.
	// This ensures that `type Assoc = i32; const C: i32 = ...` works.
	// WAIT: The trait definition is `type Assoc; const C: Self::Assoc;`.
	// So `C`'s type depends on `Assoc`.
	// When we implement `type Assoc = T;`, `C` becomes `const C: T = val;`.
    // We need to remember what we chose for `Assoc` so we can generate the right value for `C`.
    
    // Let's store the chosen type for the associated type simply by name if possible,
    // or just rely on the fact that we process them in order?
    // In `make_impl_items_for_trait`, we iterate over items. `Assoc` usually comes first in our templates.
    // Let's keep a small map of `Ident -> String` (name of the primitive type) to valid value generator.

    let mut assoc_type_choice: Option<String> = None;

    // Primitives we support for consts:
    let primitives = ["u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "bool", "char", "usize", "isize"];

	for it in &tr.items {
		match it {
			TraitItem::Type(assoc) if assoc.default.is_none() => {
				let assoc_ident = &assoc.ident;
				let base_ty_str: String;
				
				let base_ty: Type = if has_const {
                    // Pick a random primitive
					let chosen = primitives.choose(&mut rand::thread_rng()).unwrap();
                    base_ty_str = chosen.to_string();
                    assoc_type_choice = Some(base_ty_str.clone());
                    // We need to construct Ident from string.
                    let id = Ident::new(&base_ty_str, proc_macro2::Span::call_site());
                    parse_quote!(#id)
				} else {
					let mut candidates: Vec<syn::Ident> = ttcg.types.clone();
					candidates.push(syn::Ident::new("i32", proc_macro2::Span::call_site()));
					let chosen = candidates
						.choose(&mut rand::thread_rng())
						.cloned()
						.unwrap();
                    base_ty_str = chosen.to_string();
					parse_quote!(#chosen)
				};

				let lt = assoc
					.generics
					.params
					.iter()
					.find_map(|p| match p {
						GenericParam::Lifetime(lt) => Some(lt.lifetime.clone()),
						_ => None,
					});

				let rhs_ty: Type = if let Some(lifetime) = lt {
					parse_quote!(&#lifetime #base_ty)
				} else {
					base_ty
				};

				if assoc.generics.params.is_empty() {
					out.push(parse_quote!(type #assoc_ident = #rhs_ty;));
				} else {
					let gen = &assoc.generics;
					out.push(parse_quote!(type #assoc_ident #gen = #rhs_ty;));
				}
			}
			TraitItem::Const(TraitItemConst { ident, ty, default, .. }) if default.is_none() => {
                // Generate a value based on the type we chose previously.
                // If we didn't choose one (maybe the trait doesn't have an assoc type but has a const?),
                // then we look at `ty`.
                // In our specific template `const C: Self::Assoc`, the type is opaque to us here locally 
                // unless we tracked `Assoc`.
                
                let val_expr: syn::Expr = if let Some(ty_name) = &assoc_type_choice {
                    generate_const_value(ty_name)
                } else {
                     // Fallback if we couldn't track it or complex case.
                     // Just try 0 and hope it's an integer.
                     parse_quote!(0)
                };
                
				out.push(parse_quote!(const #ident: #ty = #val_expr;));
			}
			TraitItem::Fn(TraitItemFn { sig, default, .. }) if default.is_none() => {
				let sig = sig.clone();
				out.push(parse_quote!(#sig { unimplemented!() }));
			}
			_ => {}
		}
	}

	out
}

fn generate_const_value(type_name: &str) -> syn::Expr {
    let mut rng = rand::thread_rng();
    match type_name {
        "bool" => {
            let b = rng.gen::<bool>();
            parse_quote!(#b)
        },
        "char" => {
             // Random printable char? Or just 'a'
             // Keeping it simple to avoid escaping issues
             let chars = ['a', 'b', 'c', 'X', 'Y', 'Z', '0', '9'];
             let c = chars.choose(&mut rng).unwrap();
             parse_quote!(#c)
        },
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => {
             // For simplicity, generate a u16 and cast, or just use a small number.
             // We can generate a literal.
             let v: u32 = rng.gen_range(0..1000);
             // We don't strictly need type suffix if the const type is defined, but it helps.
             // Actually, `const C: u8 = 100;` works fine.
             let lit = syn::LitInt::new(&v.to_string(), proc_macro2::Span::call_site());
             parse_quote!(#lit)
        },
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
             let v: i32 = rng.gen_range(-1000..1000);
             let lit = syn::LitInt::new(&v.to_string(), proc_macro2::Span::call_site());
             parse_quote!(#lit)
        },
        _ => parse_quote!(0) // Fallback
    }
}
