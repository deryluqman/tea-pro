[![Build Status](https://travis-ci.org/cmccomb/vote.svg?branch=master)](https://travis-ci.org/cmccomb/vote)
[![Crates.io](https://img.shields.io/crates/v/vote.svg)](https://crates.io/crates/vote)


# About
This crate provides functionality for several common ranked choice voting methods.

# Example Usage
Using this crate is easy! Simply add this crate as a dependency and then `use` it:
``` 
use vote::{random_dictator, Preference};

fn main() {
    // Make a preference profile
    let v = Preference(vec![vec![0, 1, 2, 3]; 4]);

    // Make a voting method
    let x = random_dictator(v);

    // Get the result
    println!("{:?}", x.0)
}
```
The preferences profile is a unit struct which contains something of type `vec<vec<T>>` which represents. 
The first level of indexes represents the voters, each of whom has a `vec<T>` to encode their votes. This vec contains
unique items such that if an item is at index `i`, its rank in the vote is `i+1`.