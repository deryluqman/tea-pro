# About
This crate provides functionality for several functions that are commonly
used to benchmark new optimization algorithms. More specifically, function is part of a struct
that contains the objective function as well as other important information (bounds of the
canonical problem, the known minimum value, and a function that returns the global minimizer.

This crate provides access to several single- and multi-objective funtions. For exhaustive lists, check [here](single/index.html) and [here](multi/index.html), respectively.


# Example Usage
Using this crate is easy! Simply add this crate as a dependency and then `use` it:
``` 
use benchfun::*;

let fx = benchfun::single::Ackley::f(vec![0.0; 3]);
println!("{}", fx);

```
You can also use a `use` statement that looks more like `use benchfun::{SingleObjective}` but that's just messy!