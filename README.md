# SUS Language
A Hardware Description Language focussed on strong type and temporal safety features

Main Inspirations: TL-Verilog, Rust

## Core philosophy
This project is an attempt to create a safety-first, correct-by-default yet still with low level control HDL much like Rust is for the software industry. 

Current HDLs mostly build on top of existing Software languages such as Chisel and SpinalHDL. This allows for great software integration, but throws away a lot of the terseness and extended type safety that HDLs could benefit from. 

A great and interesting new innovation is TL-Verilog. In this language they built a higher level abstraction for designing hardware, moving away from the Register-Transfer level to a pipeline-focussed design. This makes TL-Verilog incredibly well-suited for the development of processing pipelines. What holds TL-Verilog back from being the language that accomplishes the goals of , and 

- Strong Typing
- Eliminate common issues
- Channels with info. Free flowing, Slowdown, Stall channels
- Temporal safety
- Ease of creating and fine-tuning processing pipelines
- Easy to test with software integration
- Better visualization of data flow --> Eliminate Wave plots
- Integrate most timing analyzer constraints into source files themselves. False paths, Clocks, multicycle paths, etc. All belong in 

### Terseness (Similar to many current HDLs, such as Chisel)
- Bundles
- Interfaces
- Handle control signals with channels
- Clocks are handled with dedicated syntax
- Syntactic sugar for Resets

### Integrate Timing Analizer constraints into language
- False/multicycle paths

## Features

### Channels
Channels are the main abstraction used in this language. The main channel type is the 'pipe' channel. Data traveling along a 'pipe' channel has an extra bit which denotes if the data is valid. 

### Time slicing

Channel data going through a pipe expects operations to only be performed on data of the same 'time slice'. This is data that has departed at the same time. Performing operations on data of different time slices is an error, unless cast explicitly (for things like FIR filters or fixed size convolutions). 

A big benefit of 'time slicing' is greater ability for debugging. Instead of staring at wave plots, the whole trajectory of a data packet can be followed throughout the pipeline, making spotting errors far easier. 

### Easy pipelining syntax borrowed from TL-Verilog
TL-Verilog's main feature, the line break pipelining syntax allows for tivial manual pipelining of operations, which I am a big fan of! This integrates very nicely with the time slicing feature. 

### Stricter integer types
I propose to add one generic integer type: *int<low, high>*. Instead of specifying the bitwidth of this integer, we specify its absolute range. It is not necessary to specify this range for every integer, as in most cases it can be inferred by the compiler. This inference allows the compiler to use the minimum bitwidth necessary to represent the integer. Signed integers are just integers with a negative lower bound. 

We can add functions such as `int -> to2cpl -> bool[]` and `bool[] -> from2cpl -> int`

This also allows us to more strictly define our interfaces. Instead of requesting an int of so many bits, we request a specific range. 

Casting integers to smaller sizes is again a place where explicitly casting is required. The simulator can then check this at runtime. 

### Module instantiation syntax
There are no functions, every function is a module. 

A module is instantiated as follows: `input1, input2 -> myModule -> output1, output2`

This can still change

###


### Strong Standard Library
- Avoids repeating common structures
- Refuse to rely on "inference" for hard logic blocks, instead start from the constraints inherent in these hard logic blocks to adapt the hardware around these blocks. For example hard logic registers around multiply blocks and BRAM blocks. This integrates well with Channels for example

## Constraints

### Data Loss
- No data loss
- No new invalid data
- every read must correspond to data destruction
- data destruction must happen together with a read
- Channel Splits and merges may not lose or duplicate date

### Temporal safety
- Operations may only happen on data of the same 'time slice' within a stream
- "Happens-before" relations -> proving FIFOs

### Strong Typing
- Actual data types
- sized integers
- representation independent integers

## Goals
### Formal proofs for correctness of common constructs
- Multiply-Add circuit
- Skid Buffer
- Safe Stream Split over multiple work units
- Safe Stream Merge of multiple work blocks
- FIFO
- Ready/Acknowledge Clock domain Crossing
- Ring pipeline
- 