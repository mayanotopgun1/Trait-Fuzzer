use syn::File;
use rand::Rng;

// Standard Mutator Interface
pub trait Mutator {
    // Step 1 & 2: Traverse and Collect candidates
    fn collect(&mut self, ast: &File) -> usize;

    // Step 4 & 5: Apply mutation to selected candidate
    fn mutate(&mut self, ast: &mut File, index: usize) -> bool;

    // Step 1-6 Driver (Standardized logic)
    fn run(&mut self, ast: &mut File) -> bool {
        self.run_with_meta(ast, None).0
    }

    /// Like `run()`, but optionally forces a specific candidate `index`.
    /// Returns (mutated, chosen_index, candidate_count).
    ///
    /// This is used by the Python driver to avoid repeatedly sampling the same
    /// mutation point for a given seed+strategy.
    fn run_with_meta(&mut self, ast: &mut File, forced_index: Option<usize>) -> (bool, usize, usize) {
        // Step 1 & 2
        let count = self.collect(ast);
        if count == 0 {
            return (false, 0, 0);
        }

        // Step 3: Select Mutation Instance
        let index = match forced_index {
            Some(i) if i < count => i,
            _ => {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..count)
            }
        };

        // Step 4 & 5 & 6
        let mutated = self.mutate(ast, index);
        (mutated, index, count)
    }
}
