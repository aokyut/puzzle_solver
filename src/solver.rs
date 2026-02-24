use crate::SequentialMovementPuzzle;
use std::{
    cmp::{Eq, Ord, PartialOrd}, collections::{BinaryHeap, hash_map::Entry, HashMap, HashSet, hash_map::VacantEntry}
};

pub struct ScoredQueue<P>{
    puzzle: P,
    actual_cost: f32,
    expected_cost: f32,
}

impl<P> ScoredQueue<P>{
    fn score(&self) -> f32{
        return self.actual_cost + self.expected_cost;
    }
}

impl<P> PartialOrd for ScoredQueue<P>{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.score().total_cmp(&other.score()).reverse());
    }
}

impl<P> PartialEq for ScoredQueue<P>{
    fn eq(&self, other: &Self) -> bool {
        return self.score() == other.score();
    }
}

impl<P> Eq for ScoredQueue<P>{
}

impl<P> Ord for ScoredQueue<P>{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

type Cost = f32;
type ActionId = usize;

// TODO: reached: HashMap内部で経路復元のためにparentをそのまま保持しているがもっと良い方法がありそう
pub fn solve_with_a_star<Action: Ord, P: SequentialMovementPuzzle<Action>>(root: P, heuristic: impl Fn(&P) -> f32) -> Option<Vec<Action>>{
    let mut queue: BinaryHeap<ScoredQueue<P>> = BinaryHeap::new();
    let root_node = ScoredQueue{ 
        puzzle: root.clone(), 
        actual_cost: 0.0, 
        expected_cost: heuristic(&root),
    };
    queue.push(root_node);
    let mut reached: HashMap<P, (ActionId, P, Cost, bool)> = HashMap::new();
    reached.insert(root.clone(), (0, root.clone(), 0.0, false));
    let mut result = None;
    
    while let Some(tar_node) = queue.pop(){
        println!("queue:{}, {}, {}", queue.len(), tar_node.actual_cost, tar_node.expected_cost);
        if tar_node.puzzle.is_solved(){
            result = Some(tar_node.puzzle.clone());
            break
        }
        if tar_node.puzzle.is_failed(){
            reached.entry(tar_node.puzzle)
                .and_modify(|(_, _, _, flag)|{
                    *flag = true;
                });
            continue
        }
        if let Some((_, _, _, reached)) = reached.get(&tar_node.puzzle){
            if *reached{
                continue
            }
        }
        let tar_cost = tar_node.actual_cost;

        for (action_id, child) in tar_node.puzzle.get_children().into_iter().enumerate(){
            let h = heuristic(&child);

            reached.entry(child.clone())
                .and_modify(|(_id, _parent, _cost, _reached)|{
                    if *_cost > tar_cost + 1.0{
                        *_cost = tar_cost + 1.0;
                        *_id = action_id;
                        *_parent = tar_node.puzzle.clone();
                        *_reached = false;
                    }
                })
                .or_insert((action_id, tar_node.puzzle.clone(), tar_cost + 1.0, false));
            let child_node = ScoredQueue{
                puzzle: child,
                actual_cost: tar_cost + 1.0,
                expected_cost: h
            };
            
            queue.push(child_node);
        }

        reached.entry(tar_node.puzzle)
            .and_modify(|(_, _, _, flag)|{
                *flag = true;
            });
    }

    // 経路復元
    if result.is_none(){
        return None;
    }
    let mut last = result.unwrap();
    let mut action_path = Vec::new();
    loop{
        let val = reached.get(&last).unwrap();
        if val.1 == last{
            action_path.reverse();
            return Some(action_path);
        }
        let mut actions = val.1.sorted_action();
        let action = actions.remove(val.0);
        action_path.push(action);
        last = val.1.clone();
    }
}