# About
This crate provides functionality for several common voting methods.

# Example Usage
Using this crate is easy! Simply add this crate as a dependency and then `use` it:
``` 
use vote::{audit, Method};

fn main() {
    // Make a preference profile
    let v = vec![vec![0, 1, 2, 3]; 4];

    // Make sure everything is hunkydory
    audit(&v);

    // Make a voting method
    let x = vote::RandomDictator(v);

    // Get the result
    println!("{:?}", x.get_ranking())
}
```