use clap::Parser;
use quote::quote;
use std::fs;
use std::path::PathBuf;
use syn::{parse_file, File};

mod mutators;
mod ttcg;
use mutators::Mutation_1::*;
use mutators::Mutation_2::*;
use mutators::Mutation_3::*;
use mutators::framework::Mutator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long)]
    mode: String,

    /// Force a particular mutation-candidate index (0-based) within the selected mutator.
    /// If out of range, the mutator will fall back to random selection.
    #[arg(long)]
    index: Option<usize>,

    /// Emit mutation-choice metadata to stderr as a single parseable line.
    /// Format: "MUTATION_CHOICE mode=<mode> count=<count> index=<index> mutated=<0|1>"
    #[arg(long, default_value_t = false)]
    emit_choice: bool,

    /// Force a particular choice index (0-based) for trait-constraint mutation (constraint_injection).
    #[arg(long)]
    constraint_index: Option<usize>,

    /// Force a particular replacement-candidate index (0-based) within projection_rewrite.
    #[arg(long)]
    choice_index: Option<usize>,

    /// Force a particular trait pattern (0=Basic, 1=Assoc, 2=GAT, 3=AssocConst).
    #[arg(long)]
    pattern_index: Option<usize>,
}

fn main() {
    let args = Args::parse();
    
    let content = fs::read_to_string(&args.input).expect("Failed to read input file");
    let mut syntax_tree: File = match parse_file(&content) {
        Ok(f) => f,
        Err(e) => {
            // Many rustc tests intentionally use syntax that newer compilers accept
            // but `syn` may not yet parse. Don't panic; let the driver skip.
            eprintln!("Parse failed: {}", e);
            eprintln!("No mutation performed.");
            fs::write(&args.output, content).expect("Failed to write output file");
            return;
        }
    };

    // Unified TTCG/TCS metrics mode: emit JSON for the Python driver (seed selection/stagnation).
    // Keeps the same CLI contract (input/output/mode) to avoid changing callers.
    if args.mode.as_str() == "ttcg_metrics" {
        let info = crate::ttcg::TtcgInfo::from_file(&syntax_tree);
        let c = crate::ttcg::ConstraintChoiceMetrics::from_file(&syntax_tree);
        let p = ProjectionRewriteMutator::projection_choice_metrics(&syntax_tree);
        let payload = serde_json::json!({
            "constraint_sites": c.constraint_sites,
            "constraint_choice_sum": c.constraint_choice_sum,
            "lifetime_sites": c.lifetime_sites,
            "rewrite_sites": p.rewrite_sites,
            "rewrite_choice_sum": p.rewrite_choice_sum,
            "traits": info.traits.len(),
            "types": info.types.len(),
            "impl_edges": info.impl_edges.len(),
            "supertrait_edges": info.supertrait_edges.len(),
            "trait_assoc_types": info.trait_assoc_types.len(),
            "impl_assoc_bindings": info.impl_assoc_bindings.len(),
        });
        println!("{}", payload.to_string());
        fs::write(&args.output, content).expect("Failed to write output file");
        return;
    }

    if args.mode.as_str() == "ttcg_entities" {
        let info = crate::ttcg::TtcgInfo::from_file(&syntax_tree);
        let payload = serde_json::json!({
            "types": info.types.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            "traits": info.traits.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            "impl_edges": info
                .impl_edges
                .iter()
                .map(|(ty, tr)| serde_json::json!({"type": ty.to_string(), "trait": tr.to_string()}))
                .collect::<Vec<_>>(),
            "supertrait_edges": info
                .supertrait_edges
                .iter()
                .map(|(tr, sup)| serde_json::json!({"trait": tr.to_string(), "supertrait": sup.to_string()}))
                .collect::<Vec<_>>(),
            "trait_assoc_types": info
                .trait_assoc_types
                .iter()
                .map(|(tr, assoc)| serde_json::json!({"trait": tr.to_string(), "assoc": assoc.to_string()}))
                .collect::<Vec<_>>(),
        });
        println!("{}", payload.to_string());
        fs::write(&args.output, content).expect("Failed to write output file");
        return;
    }

    if args.mode.as_str() == "constraint_debug" {
        let sites = ConstraintInjectionMutator::collect_sites_with_candidates(&syntax_tree);
        println!("{}", serde_json::to_string(&sites).unwrap_or("[]".to_string()));
        fs::write(&args.output, content).expect("Failed to write output file");
        return;
    }

    if args.mode.as_str() == "constraint_debug_pretty" {
        let sites = ConstraintInjectionMutator::collect_sites_with_candidates(&syntax_tree);
        for s in sites {
            println!("#{} [{}] {}", s.index, s.kind, s.label);
            println!("  candidates: {}", s.candidates.len());
            for c in s.candidates {
                println!("    - {}", c);
            }
            println!();
        }
        fs::write(&args.output, content).expect("Failed to write output file");
        return;
    }

    if args.mode.as_str() == "projection_debug_pretty" {
        let sites = ProjectionRewriteMutator::collect_sites_with_candidates(&syntax_tree);
        for s in sites {
            println!("#{} [rewrite] {}", s.index, s.label);
            println!("  candidates: {}", s.candidates.len());
            for c in s.candidates {
                println!("    - {}", c);
            }
            println!();
        }
        fs::write(&args.output, content).expect("Failed to write output file");
        return;
    }

    let (mutated, chosen_index, candidate_count, constraint_count, chosen_constraint_index) = match args.mode.as_str() {
        // Structural
        "add_trait" => {
            let mut mutator = AddTraitMutator { force_pattern: args.pattern_index };
            let (m, i, c) = mutator.run_with_meta(&mut syntax_tree, args.index);
            (m, i, c, 0, 0)
        }
        "add_impl" => {
            let (m, i, c) = AddImplMutator.run_with_meta(&mut syntax_tree, args.index);
            (m, i, c, 0, 0)
        }
        
        // Injection
        "constraint_injection" => ConstraintInjectionMutator::run_with_meta_and_constraint(
            &mut syntax_tree,
            args.index,
            args.constraint_index,
        ),
        "supertrait_injection" => ConstraintInjectionMutator::run_with_meta_supertrait(
            &mut syntax_tree,
            args.index,
            args.constraint_index,
        ),
        "projection_rewrite" => ProjectionRewriteMutator::run_with_meta_and_choice(
            &mut syntax_tree,
            args.index,
            args.choice_index,
        ),
        
        _ => {
            eprintln!("Unknown mode: {}", args.mode);
            (false, 0, 0, 0, 0)
        }
    };

    if args.emit_choice {
        let m = if mutated { 1 } else { 0 };
        if args.mode.as_str() == "constraint_injection" || args.mode.as_str() == "supertrait_injection" {
            eprintln!(
                "MUTATION_CHOICE mode={} count={} index={} mutated={} choice_count={} choice_index={}",
                args.mode,
                candidate_count,
                chosen_index,
                m,
                constraint_count,
                chosen_constraint_index,
            );
        } else if args.mode.as_str() == "projection_rewrite" {
            eprintln!(
                "MUTATION_CHOICE mode={} count={} index={} mutated={} choice_count={} choice_index={}",
                args.mode,
                candidate_count,
                chosen_index,
                m,
                constraint_count,
                chosen_constraint_index,
            );
        } else {
            eprintln!(
                "MUTATION_CHOICE mode={} count={} index={} mutated={}",
                args.mode,
                candidate_count,
                chosen_index,
                m
            );
        }
    }

    if mutated {
        eprintln!("Mutation successful.");
    } else {
        eprintln!("No mutation performed.");
    }

    // prettyplease can panic on newer/unsupported `syn` nodes (e.g. TypeParamBound::Verbatim).
    // Don't let formatting crash the whole mutation tool; fall back to token-based printing.
    // Note: even if we catch_unwind, the default panic hook prints to stderr; temporarily silence it.
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mutated_content = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        prettyplease::unparse(&syntax_tree)
    })) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("prettyplease panicked; falling back to token-based output");
            quote!(#syntax_tree).to_string()
        }
    };
    std::panic::set_hook(prev_hook);
    fs::write(&args.output, mutated_content).expect("Failed to write output file");
}
