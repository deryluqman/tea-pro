#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This package contains common voting methods.

use rand::Rng;
use std::{collections::HashSet};
use std::{collections::HashMap};

/// This is a results object
pub struct Results<T>(pub Vec<T>);

/// This is a preference structure object
pub struct Preference<T>(pub Vec<Vec<T>>);

impl<T> Preference<T>
    where T: std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + std::clone::Clone
{

    /// This function makes it possible to audit the preference structure for accuracy
    pub fn audit(&self){
        let n: usize = self.0.len();
        let m: usize = self.0[0].len();

        // Generate a hash set for each item
        let mut hashes: Vec<HashSet<T>> = vec![];
        for i in 0..n {
            // Repack hash
            let mut new_hash = HashSet::new();
            for j in 0..self.0[i].len() {
                new_hash.insert(self.0[i][j].clone());
            }
            hashes.push(new_hash);
        }

        // Check that each contains the same set of items
        for i in 0..n {
            for j in i..n {
                let inter = hashes[i].intersection(&hashes[j]);
                if inter.count() != m {
                    // println!("{} {}", inter.count(), m);
                    panic!("Index {} of preference structure does not contain the same set of values as index {}", i, j);
                }
            }
        }
    }
}

/// This is a function for... things.
fn sort_hashmap<T>(results: HashMap<T, i64>) -> Results<T>
    where T: std::clone::Clone
{
    // Get a sorted (by field 0 ("count") in reversed order) list of the
    // most frequently used characters:
    let mut count_vec: Vec<(&T, &i64)> = results.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Unzip the count_vec
    let mut new_vec: Vec<T> = vec![];
    for elem in count_vec {
        let key = elem.0;
        new_vec.push((*key).clone());
    }

    Results(new_vec)
}

/// This is a helper function for other things
fn positional_voting<T>(pref: Preference<T>, weights: Vec<i64>) -> Results<T>
    where T: std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + std::clone::Clone
{
    // Audit it
    pref.audit();

    // Make a hashmap to save results
    let mut results: HashMap<T, i64> = HashMap::new();
    for elem in pref.0 {
        for i in 0..elem.len() {
            let current = results.entry(elem[i].clone()).or_insert(0);
            *current += weights[i] as i64;
        }
    }

    // Sort a vec for results
    sort_hashmap(results)
}

/// The Random Dictatorship voting method
pub fn random_dictator<T>(pref: Preference<T>) -> Results<T>
    where T: std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + std::clone::Clone
{
    // Audit it
    pref.audit();

    // Choose a random one
    let idx = rand::thread_rng().gen_range(0, pref.0.len());

    // Return it
    Results(pref.0[idx].clone())
}

/// The Plurality voting method
pub fn plurality<T>(pref: Preference<T>) -> Results<T>
    where T: std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + std::clone::Clone
{
    let mut weights = vec![0; pref.0[0].len()];
    weights[0] = 1;
    positional_voting(pref, weights)
}

/// The Borda voting method
pub fn borda<T>(pref: Preference<T>) -> Results<T>
    where T: std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + std::clone::Clone
{
    let mut weights = vec![];
    let n = pref.0[0].len();
    for i in 0..n {
        weights.push((n - i) as i64);
    }
    positional_voting(pref, weights)
}

#[cfg(test)]
mod type_tests {
    use crate::{Preference};

    #[test]
    fn try_some_ints() {
        let v = Preference(vec![vec![0, 1, 2, 3]; 4]);
        v.audit();
    }

    #[test]
    fn try_some_chars() {
        let v = Preference(vec![vec!['a', 'b', 'c', 'd']; 4]);
        v.audit();
    }

    #[test]
    fn try_some_enums() {
        #[derive(Clone, Eq, PartialEq, Hash)]
        enum Voters {
            A,
            B,
            C,
            D
        }
        let v = Preference(vec![vec![Voters::A, Voters::B, Voters::C, Voters::D]; 4]);
        v.audit();
    }

    #[test]
    #[should_panic]
    fn panic_audit_int() {
        let v = Preference(vec![
            vec![3, 1, 2, 9],
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3]]);
        v.audit();
    }

    #[test]
    #[should_panic]
    fn panic_audit_char() {
        let v = Preference(vec![
            vec!['3', '1', '2', '9'],
            vec!['0', '1', '2', '3'],
            vec!['0', '1', '2', '3']]);
        v.audit();
    }

    #[test]
    #[should_panic]
    fn panic_audit_enums() {
        #[derive(Clone, Eq, PartialEq, Hash)]
        enum Voters {
            A,
            B,
            C,
            D
        }
        let v = Preference(vec![vec![Voters::A, Voters::B, Voters::C, Voters::C, Voters::D]; 4]);
        v.audit();
    }
}


#[cfg(test)]
mod tests {
    use crate::{Preference, borda, plurality, random_dictator};

    #[test]
    fn dictatorship_functionality() {
        let v = vec![vec![0, 1, 2, 3]; 4];
        let p = Preference(v);

        assert_eq!(vec![0, 1, 2, 3], random_dictator(p).0);
    }

    #[test]
    fn plurality_functionality() {
        let v = vec![
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3],
            vec![1, 0, 2, 3],
            vec![1, 0, 2, 3],
            vec![2, 1, 0, 3]];
        let p = Preference(v);
        assert_eq!(vec![0, 1, 2, 3], plurality(p).0);
    }

    #[test]
    fn borda_functionality() {
        let v = vec![
            vec![3, 1, 2, 0],
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3],
            vec![3, 0, 1, 2]];
        let p = Preference(v);
        assert_eq!(vec![0, 1, 3, 2], borda(p).0);
    }
}