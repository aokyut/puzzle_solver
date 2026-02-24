use std::ops::Mul;
use std::collections::{HashMap, HashSet};
pub mod circle_puzzle;

#[derive(Debug)]
pub struct PermuteGroup{
    size: usize,
    ops: Vec<Permute>,
}

impl PermuteGroup{
    pub fn from(size: usize, ops: Vec<Permute>) -> Self{
        return Self { size, ops };
    }
}



#[derive(Clone, Debug)]
pub struct Permute{
    map: HashMap<usize, usize>
}

impl Mul for &Permute{
    type Output = Permute;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut new: HashMap<usize, usize> = HashMap::new();
        let relm_a: HashSet<usize> = self.map.iter().map(|a|*a.0).collect();
        let relm_b: HashSet<usize> = rhs.map.iter().map(|a|*a.0).collect();
        let input = relm_a.union(&relm_b);
        
        for &key in input.into_iter(){
            match rhs.map.get(&key){
                None => {
                    new.insert(key, *self.map.get(&key).unwrap());
                },
                Some(v) => {
                    let last = self.map.get(v);
                    match self.map.get(v){
                        None => {
                            new.insert(key, *v);
                        },
                        Some(vv) => {
                            if *vv == key{
                                continue
                            }else{
                                new.insert(key, *vv);
                            }
                        }
                    }
                }
            }
        }

        return Permute{map: new}
    }
}

impl Mul for Permute{
    type Output = Permute;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut new: HashMap<usize, usize> = HashMap::new();
        let relm_a: HashSet<usize> = self.map.iter().map(|a|*a.0).collect();
        let relm_b: HashSet<usize> = rhs.map.iter().map(|a|*a.0).collect();
        let input = relm_a.union(&relm_b);
        
        for &key in input.into_iter(){
            match rhs.map.get(&key){
                None => {
                    new.insert(key, *self.map.get(&key).unwrap());
                },
                Some(v) => {
                    let last = self.map.get(v);
                    match self.map.get(v){
                        None => {
                            new.insert(key, *v);
                        },
                        Some(vv) => {
                            if *vv == key{
                                continue
                            }else{
                                new.insert(key, *vv);
                            }
                        }
                    }
                }
            }
        }

        return Permute{map: new}
    }
}

impl Permute{
    pub fn from_vec(from: Vec<usize>, to: Vec<usize>) -> Self{
        assert_eq!(from.len(), to.len());
        
        let mut new = HashMap::new();
        for (f, t) in from.iter().zip(to.iter()){
            new.insert(*f, *t);
        }

        return Self { map: new };
    }
    pub fn e() -> Self{
        return Permute { map: HashMap::new() };
    }
    pub fn inverse(&self) -> Self{
        let mut new = HashMap::new();
        for (k, v) in self.map.iter(){
            new.insert(*v, *k);
        }
        return Self { map: new };
    }
    pub fn square(&self) -> Self{
        return self * self;
    }
    pub fn powi(&self, index: i32) -> Self{
        if index == 0{
            return Self::e();
        }else{
            let mut i = index.abs();
            let mut perm = self.clone();
            let mut ans = Self::e();
            while i != 0{
                if i & 1 == 1{
                    ans = perm.clone() * ans;
                }
                perm = perm.square();
                i >>= 1;
            }
            return ans;
        }
    }

    pub fn eq(&self, other: &Permute) -> bool{
        if self.size() != other.size(){
            return false;
        }
        for (k, v) in self.map.iter(){
            match other.map.get(k){
                None => {
                    return false;
                },
                Some(ov) => {
                    if ov != v{
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn to_string(&self) -> String{
        if self.map.len() == 0{
            return format!("|E|")
        }
        let mut s = String::new();
        let mut v = Vec::new();
        s += "|";
        let mut keys: Vec<usize> = self.map.keys().map(|a|*a).collect();
        keys.sort();
        for (i, key) in keys.iter().enumerate(){
            if i != 0{
                s += " ";
            }
            s += format!("{:>2}", *key).as_str();
            v.push(self.map.get(key).unwrap());
        }
        s += "|\n|";
        for (i, val) in v.into_iter().enumerate(){
            if i != 0{
                s += " ";
            }
            s += format!("{:>2}", *val).as_str();
        }
        s += "|";

        return s;
    }

    pub fn order(&self) -> usize{
        let mut num = 2;
        let mut now = self.square();
        loop {
            if now.size() == 0{
                return num;
            }
            now = self * &now;
            num += 1;
        }
    }

    pub fn size(&self) -> usize{
        return self.map.len();
    }
}
