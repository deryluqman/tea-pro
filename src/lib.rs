#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This package contains common voting methods.

use rand::Rng;

/// This trait ensures consistent implementation of all voting methods
pub trait Method {
    /// This method returns a ranking
    fn get_ranking(&self) -> Vec<i32>;
}

/// The random dictatorship voting method
pub struct RandomDictator (Vec<Vec<i32>>);

impl Method for RandomDictator {
    fn get_ranking(&self) -> Vec<i32> {
        let idx = rand::thread_rng().gen_range(0, self.0.len());
        let v = self.0.clone();
        v[idx].clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{RandomDictator, Method};

    #[test]
    fn it_works() {
        let v = vec![vec![1, 2, 3]; 4];
        let m = RandomDictator(v.clone());
        assert_eq!(v[0], m.get_ranking());
    }
}
