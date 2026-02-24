use puzzle_solver::{*, permutation_group::*};

fn main() {
    // let root = SquarePuzzle{pos: (0, 0), tar: (3, 5)};
    // let action = solver::solve_with_a_star(root, test_heuristic);
    // println!("{:#?}", action);

    let p = circle_puzzle::CirclePuzzle::from_vec(
        vec![0, 6, 2, 11, 13, 4, 12, 1, 5, 14, 1, 12, 0]
    );
    let actions = solver::solve_with_a_star(p, circle_puzzle::heuristic);

    println!("{:#?}", actions);

    return;

    let ops = vec![
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
    ];

    let tar = ops[0].clone() * ops[1].clone();

    let mut all_ops = Vec::new();

    for op in ops.iter(){
        let mut set_ops = Vec::new();
        for i in 1..6{
            set_ops.push(op.powi(i));
        }
        all_ops.push(set_ops);
    }

    println!("{}", tar.to_string());

    let e = Permute::e();

    // search_communicator_recurse(&all_ops, &e, &e, 7, 3, String::new());
}

pub fn search_communicator_recurse(ops: &Vec<Vec<Permute>>, src: &Permute, left_src: &Permute, depth: usize, pre_idx: usize, name: String){
    let mut next_src = Vec::new();
    for (i, op_set) in ops.iter().enumerate(){
        if i == pre_idx{
            continue
        }
        for (j, op) in op_set.iter().enumerate(){
            let tar = op * src;
            let left_tar = &op.inverse() * left_src;
            next_src.push((i, j, tar, left_tar));
        }
    }
    for (i, j, next, next_left) in next_src.iter(){
        let n = next * next_left;
        if n.size() <= 4 && n.size() > 0{
            let next_name = format!("{}[{}_{}]", name, i, j+1);
            println!("depth:{}, order:{}, name:{}, \npermute:\n{}", depth, n.order(), next_name, n.to_string());
        }
    }
    if depth == 0{
        return;
    }
    for (i, j, next, next_left) in next_src.iter(){
        let next_name = format!("{}[{}_{}]", name, i, j+1);
        search_communicator_recurse(ops, next, next_left, depth - 1, *i, next_name);
    }
}

pub fn search_communicator(ops: Vec<Permute>){
    use std::collections::HashMap;
    let mut orig = Vec::new();
    for op in ops.iter(){
        orig.push(op.clone());
        orig.push(op.inverse());
    }

    let mut set = HashMap::new();
    set.insert(Permute::e().to_string(), Permute::e());

    for i in 0..20{
        let mut new_set = set.clone();
        for s in set.iter(){
            for o in orig.iter(){
                let next = o.clone() * s.1.clone();
                if next.size() < 4{
                    println!("{}", next.to_string());
                }
                if new_set.get(&next.to_string()).is_some(){
                    continue
                }
                if new_set.get(&next.inverse().to_string()).is_some(){
                    continue
                }
                new_set.entry(next.to_string()).or_insert(next);
            }
        }
        println!("i:{i}, size:{}", new_set.len());
        let mut counter = HashMap::new();
        for v in new_set.values(){
            let num = v.size();
            counter.entry(num).and_modify(|f|{
                *f += 1;
            }).or_insert(1);
        }
        println!("{:#?}", counter);
        set = new_set;
    }
}

pub fn test_heuristic(p: &SquarePuzzle) -> f32{
    return ((p.tar.0 - p.pos.0).abs() + (p.tar.1 - p.pos.1).abs()) as f32
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct SquarePuzzle{
    pos: (isize, isize),
    tar: (isize, isize)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MoveAction{
    Up,
    Down,
    Left,
    Right
}

impl Next<MoveAction> for SquarePuzzle{
    fn next(&self, action: &MoveAction) -> Self {
        use MoveAction::*;
        match action{
            Up => Self { pos: (self.pos.0, self.pos.1 + 1), tar: self.tar },
            Down => Self { pos: (self.pos.0, self.pos.1 - 1), tar: self.tar },
            Left => Self { pos: (self.pos.0 - 1, self.pos.1), tar: self.tar },
            Right => Self { pos: (self.pos.0 + 1, self.pos.1), tar: self.tar}
        }
    }
}

impl Solved for SquarePuzzle{
    fn is_solved(&self) -> bool {
        return self.pos.0 == self.tar.0 && self.pos.1 == self.tar.1;
    }
}

impl Failed for SquarePuzzle{}

impl GetDiscreteActions<MoveAction> for SquarePuzzle{
    fn get_discrete_actions(&self) -> Vec<MoveAction> {
        use MoveAction::*;
        return vec![Up, Down, Left, Right];
    }
}

impl SequentialMovementPuzzle<MoveAction> for SquarePuzzle{

}

