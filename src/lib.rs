use std::{hash::Hash, cmp::Ord};
pub mod solver;
pub mod permutation_group;

pub trait Solved{
    fn is_solved(&self) -> bool;
}

pub trait Failed{
    fn is_failed(&self) -> bool{
        return false;
    }
}

pub trait Next<Action>{
    fn next(&self, action: &Action) -> Self;
}

pub trait GetDiscreteActions<Action>{
    fn get_discrete_actions(&self) -> Vec<Action>;
}


pub trait SequentialMovementPuzzle<Action: Ord>: Solved + Failed + GetDiscreteActions<Action> + Next<Action> + Eq + Hash + Clone{
    fn get_children(&self) -> Vec<Self>{
        let actions = self.sorted_action();
        let next_states: Vec<Self> = actions.iter().map(|a|self.next(a)).collect();

        return next_states;
    }

    fn sorted_action(&self) -> Vec<Action>{
        let mut actions = self.get_discrete_actions();
        actions.sort_by(|a, b| a.cmp(b));
        return actions;
    }
}





















