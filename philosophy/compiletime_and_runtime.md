# Separating Compiletime and Runtime

Requirements: 
- Code that describes plain hardware should be minimal to write. One shouldn't have to 'break out' of the generative environment to write plain hardware code. 
- It should be easy to write generative code mixed with plain hardware. 

## Differences
### Compile Time
Arrays need not be bounded. Integers need not be bounded. 

### Runtime
Arrays that have dynamic indices must have a fixed size. 
Integers must be bounded. 

## Multiplexer inference
There is quite a significant difference between an array access with a constant, and one which should infer a multiplexer, but in both cases the syntax in other languages is exactly the same: `my_arr[idx]`

The constant index should infer to just a wire connection which costs nothing. In this case the different wires that are part of an array don't have any relation to each other in hardware. This allows us to bestow other properties as well. For example constant indices don't conflict with each other if they don't point to the same element. Runtime indices do. Array wires with constant indices don't enforce any latency requirements upon each other. 'dynamically sized' arrays can only be indexed with compile time indices. Etc. 

With a runtime index (based on an integer wire in the design) should infer to a multiplexer. And then of course the array wires do have a relation. 

An initial thought to distinguish the two was to just check for constant-ness of the array argument, which can be done at flattening time. But that wouldn't make the distinction clear enough. 

Proposal: Require `mux` keyword for any runtime array index which should infer to a multiplexer. 

Examples: 
- `a[5] = 1` constant index write
- `a[mux b] = 1` multiplexed write
- `x = a[5]` constant index read
- `x = a[mux b]` multiplexed index write
