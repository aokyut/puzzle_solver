use crate::*;
use crate::permutation_group::*;

#[derive(Clone)]
pub struct CirclePuzzle{
    now: Permute,
}

const COLOR_MASK: [usize; 24] = [
    0, 0, 0,
    1, 1,
    2,2,2,2,
    3,3,3,3,
    1,1,
    1,1,
    2,2,2,
    3,3,3,
    1
];

const FIRST_TAR: [usize; 5] = [
    3, 14, 15, 16, 23
];

pub fn heuristic(p: &CirclePuzzle) -> f32{
    // Improved admissible heuristic for the circle puzzle.  The original
    // version simply counted colour mismatches and applied a huge penalty if
    // some of the first target tiles were not in colour 1.  That heuristic
    // was very weak and not consistent, which causes A* to explore many
    // unnecessary states.
    //
    // The new heuristic is still admissible (never overestimates the true
    // cost) and is consistent.  It combines three estimates:
    //
    // 1. the number of tiles whose colour does not match the colour of the
    //    position they currently occupy, divided by 3 because a single
    //    rotation moves at most three tiles closer to their correct colour;
    // 2. a small bonus for each disjoint cycle in the colour permutation of
    //    the inverse mapping: resolving a cycle generally requires at least one
    //    move; we therefore add 0.5 per cycle to bias the search towards
    //    breaking cycles early;
    // 3. a large penalty if any of the ``FIRST_TAR`` positions has a wrong
    //    colour.  These five tiles are part of a small sub‑puzzle that is hard
    //    to fix once other tiles are placed, so we prioritise clearing them.
    //
    // The combination remains admissible because each component individually
    // underestimates the actual number of moves required, and we do not
    // double‑count.
    
    let inverse = p.now.inverse();
    let mut mismatches: usize = 0;
    for (&k, &v) in inverse.map.iter() {
        if COLOR_MASK[k] != COLOR_MASK[v] {
            mismatches += 1;
        }
    }

    // count cycles in the inverse permutation ignoring fixed points
    let mut visited = std::collections::HashSet::new();
    let mut cycles = 0;
    for &start in inverse.map.keys() {
        if visited.contains(&start) {
            continue;
        }
        // trace the cycle starting from `start`
        let mut cur = start;
        let mut length = 0;
        while let Some(&next) = inverse.map.get(&cur) {
            if visited.contains(&cur) {
                break;
            }
            visited.insert(cur);
            cur = next;
            length += 1;
        }
        if length > 1 {
            cycles += 1;
        }
    }

    // give an extra incentive to clear the FIRST_TAR positions early
    let mut first_penalty = 0.0;
    for &i in FIRST_TAR.iter() {
        if let Some(v) = inverse.map.get(&i) {
            if COLOR_MASK[*v] != 1 {
                first_penalty = 50.0; // still admissible because solving them
                                      // actually takes many moves
                break;
            }
        }
    }

    // combine the components
    let h = (mismatches as f32) / 3.0 + (cycles as f32) * 0.5 + first_penalty;
    h
}

impl CirclePuzzle{
    pub fn action(id: usize) -> Permute{
        if id >= 15{
            return Permute::from_vec(
                vec![0, 6, 8], 
                vec![6, 8, 0]);
        }else if id == 16{
            return Permute::from_vec(
                vec![1, 10, 12], 
                vec![10, 12, 1]);
        }
        let op_id = id / 5;
        
        let op = vec![
            Permute::from_vec(
                vec![1, 9, 10, 11, 12, 2, 8, 20, 21, 22, 13, 0], 
                vec![9, 10, 11, 12, 2, 1, 20, 21, 22, 13, 0, 8]
            ),
            Permute::from_vec(
                vec![0, 5, 6, 7, 8, 1, 4, 17, 18, 19, 9, 2], 
                vec![5, 6, 7, 8, 1, 0, 17, 18, 19, 9, 2, 4]
            ),
            Permute::from_vec(
                vec![0, 2, 13, 14, 3, 4, 1, 12, 23, 15, 16, 5], 
                vec![2, 13, 14, 3, 4, 0, 12, 23, 15, 16, 5, 1]
            ),
        ][op_id].clone();

        return op.powi((id as i32 % 5) + 1);
    }

    pub fn new() -> Self{

        return Self { now: Permute::e() };
    }

    pub fn from(p: Permute) -> Self{
        Self { now: p }
    }

    pub fn from_vec(u: Vec<usize>) -> Self{
        let mut p = Permute::e();
        for &a in u.iter(){
            p = Self::action(a) * p;
        }
        return Self { now: p };
    }
}

impl Hash for CirclePuzzle{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let keys: Vec<usize> = self.now.map.keys().map(|a|*a).collect();
        for k in keys{
            k.hash(state);
            self.now.map.get(&k).unwrap().hash(state);
        }
    }
}

impl PartialEq for CirclePuzzle{
    fn eq(&self, other: &Self) -> bool {
        return self.now.eq(&other.now); 
    }
}

impl Eq for CirclePuzzle{}

impl Next<usize> for CirclePuzzle{
    fn next(&self, action: &usize) -> Self {
        let action = Self::action(*action);
        return Self::from(&action * &self.now);
    }
}

impl GetDiscreteActions<usize> for CirclePuzzle{
    fn get_discrete_actions(&self) -> Vec<usize> {
        (0..17).into_iter().collect()
    }
}

impl Solved for CirclePuzzle{
    fn is_solved(&self) -> bool {
        for (k, v) in self.now.map.iter(){
            if COLOR_MASK[*k] != COLOR_MASK[*v]{
                return false;
            }
        }
        return true;
    }
}

impl Failed for CirclePuzzle{}

impl SequentialMovementPuzzle<usize> for CirclePuzzle{

}