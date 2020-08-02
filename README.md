# About
This crate provides functionality for several common voting methods.

# Example Usage
Using this crate is easy! Simply add this crate as a dependency and then `use` it:
``` 
use vote::*;

// Make a preference profile
let v = vec![vec![1, 2, 3]; 4];

// Load it into a voting method
let m = RandomDictator(v.clone());

// Get the ranking
println!("{:?}", m.get_ranking());

```