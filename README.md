# About
This crate provides functionality for several common voting methods.

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