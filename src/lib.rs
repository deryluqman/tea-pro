#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This package contains common voting methods.

use rand::Rng;

/// This function makes it possible to audit a preference structure to make sures its ok
pub fn audit(pref: &[Vec<i32>]){
    let n: usize = pref[0].len();
    for (i, x) in pref.iter().enumerate() {
        if x.len() != n {
            panic!("Index {} of preference structure is not equal in length to the first", i);
        }
        for j in 0..n {
            if !x.contains(&(j as i32)) {
                panic!("Index {} of preference structure does not contain the value {}", i, j);
            }
        }
    }
}

/// This trait provides an implementation for returning ranks from a
pub trait Rank {
    /// Returns a ranked version of the preference structure
    fn rank(&self) -> Vec<i32>;

    /// Returns the maximum value in the vector
    fn rank_max(&self) -> i32;

    /// Returns the argmax of the vector
    fn rank_argmax(&self) -> Vec<usize>;
}

impl Rank for Vec<i32> {
    fn rank(&self) -> Vec<i32> {
        // Get a copy of self that we can work on
        let mut v = self.clone();

        // Get a vector to save rankings in
        let mut ranked: Vec<i32> = vec![0; self.len()];

        // Get argmax, assign to rank, and set value to -1
        for i in 0..v.len() {
            let results = v.rank_argmax();
            ranked[i] = results[0] as i32;
            v[results[0]] = -1;
        }
        ranked
    }

    fn rank_max(&self) -> i32 {
        let mut current_max: i32 = i32::MIN;
        for x in self.iter() {
            if *x > current_max {
                current_max = *x;
            }
        }
        current_max
    }

    fn rank_argmax(&self) -> Vec<usize> {
        // Get a version of the vector to work on
        let v = self.clone();

        // Get the current max
        let m: i32 = self.rank_max();

        // Find the index where that max occurs
        let mut index: Vec<usize> = vec![];
        for (i, vi) in v.iter().enumerate() {
            if *vi == m {
                index.push(i as usize);
            }
        }

        // Return as usize
        index
    }
}

/// This trait ensures consistent implementation of all voting methods
pub trait Method {
    /// This method returns a ranking
    fn get_ranking(&self) -> Vec<i32>;
}

/// The Random Dictatorship voting method
pub struct RandomDictator (pub Vec<Vec<i32>>);

impl Method for RandomDictator {
    fn get_ranking(&self) -> Vec<i32> {
        let idx = rand::thread_rng().gen_range(0, self.0.len());
        let v = self.0.clone();
        v[idx].clone()
    }
}


/// The Plurality voting method
pub struct Plurality (pub Vec<Vec<i32>>);

impl Method for Plurality {
    fn get_ranking(&self) -> Vec<i32> {
        let mut counts = vec![0; self.0[0].len()];
        for pref in &self.0 {
            counts[pref[0] as usize] += 1;
        }
        counts.rank()
    }
}


/// The Plurality voting method
pub struct Borda (pub Vec<Vec<i32>>);

impl Method for Borda {
    fn get_ranking(&self) -> Vec<i32> {
        let n = self.0[0].len();
        let mut scores: Vec<i32> = vec![0; self.0[0].len()];
        for pref in &self.0 {
            for (i, elem) in pref.iter().enumerate() {
                scores[*elem as usize] += (n - i) as i32;
            }
        }
        scores.rank()
    }
}


#[cfg(test)]
mod tests {
    use crate::{audit, RandomDictator, Method, Plurality};

    #[test]
    fn dictatorship() {
        let v = vec![vec![0, 1, 2, 3]; 4];
        audit(&v);
        let m = RandomDictator(v);
        assert_eq!(vec![0, 1, 2, 3], m.get_ranking());
    }

    #[test]
    fn plurality() {
        let v = vec![
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3],
            vec![1, 0, 2, 3],
            vec![1, 0, 2, 3],
            vec![2, 1, 0, 3]];
        audit(&v);
        let m = Plurality(v);
        assert_eq!(vec![0, 1, 2, 3], m.get_ranking());
    }

    #[test]
    fn borda() {
        let v = vec![
            vec![3, 1, 2, 0],
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3]];
        audit(&v);
        let m = Plurality(v);
        assert_eq!(vec![0, 3, 1, 2], m.get_ranking());
    }

    #[test]
    #[should_panic]
    fn panic_on_audit() {
        let v = vec![
            vec![3, 1, 2, 57],
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3]];
        audit(&v);
    }
}
