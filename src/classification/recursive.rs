use bit_matrix::BitMatrix;

use analysis::{self, RhsClosure};
use ContextFree;
use ContextFreeRef;

/// Calculation of parts of grammar that participate in recursion,
/// be it left-recursion, right-recursion or middle-recursion.
pub struct Recursion<'a, G: 'a> {
    grammar: &'a G,
    derivation: BitMatrix,
    distances: Vec<Vec<Option<u32>>>,
    prediction_distances: Vec<Option<u32>>,
    completion_distances: Vec<Option<u32>>,
    min_of: Vec<Option<u32>>,
}

struct RecursiveRules<'a, G, R>
    where G: 'a
{
    rules: R,
    recursion: &'a Recursion<&'a mut G>,
}

impl<'a, G> Recursion<'a, G>
    where G: ContextFree + 'a,
          &'a G: ContextFreeRef<'a, Target = G>
{
    /// Returns a new `MinimalDistance` for a grammar.
    pub fn new(grammar: &'a G) -> Self {
        let reachability = analysis::reachability_matrix(grammar);
        let distances = grammar.rules().map(|rule| vec![None; rule.rhs().len() + 1]).collect();
        Recursion {
            grammar: grammar,
            reachability,
            prediction_distances: vec![None; grammar.num_syms()],
            completion_distances: vec![None; grammar.num_syms()],
            min_of: vec![None; grammar.num_syms()],
        }
    }

    fn rules(&self) -> RecursiveRules<'a, G, <&'a G as ContextFreeRef<'a>>::Rules> {
        RecursiveRules {
            rules: self.grammar.rules(),
            recursion: self,
        }
    }
}
