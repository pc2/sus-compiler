# SUS Language
A Hardware Description Language focussed on strong type and temporal safety features

Main Inspirations: TL-Verilog, Filament, Rust

## Core philosophy
This project is an attempt to create a safety-first, correct-by-default HDL. It must make programming easier and safer without sacrificing on low level control. Much akin to what Rust is for the software industry. 

Current HDLs mostly build on top of existing Software languages such as Chisel and SpinalHDL. This allows for great software integration, but throws away a lot of the terseness and extended type safety that HDLs could benefit from. 

An interesting new innovation is TL-Verilog. In this language they built a higher level abstraction for designing hardware, by moving away from the Register-Transfer level to a pipeline-focussed design. This makes TL-Verilog well-suited for the development of multi-stage pipelines (a critical tenet of performance-oriented hardware). While TL-Verilog does this one thing far better than other languages, it lacks proper support for more complex pipelines, forcing the user to drop down to Verilog. This makes it not a replacement, but an extention of Verilog. 

The main goals of the language are roughly listed below:
- Strong and extensible Typing
- Data loss and duplication safety
- Easy to create and fine-tune processing pipelines
- Easy to test
- Testing Software Integration
- Better visualization of data flow --> Eliminate Wave plots
- Integrate timing constraints into source files. 

### Basic constructs (Similar to many current HDLs, such as Chisel)
- Bundles
- Interfaces
- Handle control signals with streams
- Clocks are handled with dedicated syntax
- Syntactic sugar for Resets
- Lambda Modules

## Tasks
### Major Milestones
- [ ] Arbitrary forward pipelines full flow
- [ ] Arbitrary FPGA hardware full flow
- [x] Generative Code
- [ ] Templates
### Parsing
- [x] Basic Tokenizer
- [x] Basic Syntax Error Reporting
- [x] Syntax error reporting with infos
- [x] Basic Token Highlighting in Terminal
- [x] Local Variable and Type Name highlighting
- [x] Array Syntax
- [x] Function Call Syntax
- [x] Unary and Binary Operators
- [x] Can Parse Multiply-Add pipeline
- [x] Can Parse Blur2 filter
- [x] If Statements
- [ ] Structs
- [x] For Loops
- [ ] Multi-Interface Syntax
- [ ] Native Module integration syntax
- [ ] Can Parse FIFO implementation
- [ ] Clock Domain Crossings
- [ ] Rhythm Syntax
- [ ] Generator Syntax

### Linking and Name Resolution
- [x] Single File Name Resolution
- [x] Multi File Name Resolution
- [x] Incremental Linking
- [ ] Incremental Compilation
- [ ] Multi-Threaded Compilation

### Type and Bound Checking
- [x] Basic Type Checking (bools, ints, arrays, etc)
- [ ] Types for Interfaces
- [ ] Integer and Array Bounds Checking
- [ ] Latency Checking

### LSP
- [x] Basic LSP for VSCode integration
- [x] Syntax Highlighting
- [x] Error and Warning Reporting
- [ ] Per-Line Resource Utilization Reporting

### Code Generation
- [x] Expression Flattening
- [ ] State Machine Generation
- [ ] Can Generate Verilog for Multiply-Add pipeline
- [ ] Can Generate Verilog for Blur2 filter
- [ ] Can Generate Verilog for FIFO
- [ ] Timing Failure extraction from vendor tools

### Simulation
- [ ] Basic testbench
- [ ] "Visualization"

## Features

### Streams
Streams are the main abstraction used in this language. The main stream type is the 'pipe' stream. Data traveling along a 'pipe' stream has an extra bit which denotes if the data is valid. 

### Time slicing

Streams data going through a pipe expects operations to only be performed on data of the same 'time slice'. This is data that has departed at the same time. Performing operations on data of different time slices is an error, unless cast explicitly (for things like FIR filters or fixed size convolutions). 

A big benefit of 'time slicing' is greater ability for debugging. Instead of staring at wave plots, the whole trajectory of a data packet can be followed throughout the pipeline, making spotting errors far easier. 

### Easy Pipelining
Critical for achieving high frequencies. Computation is split up over multiple stages split by registers, such that multiple operations can be 'coming down the pipe' at the same time. This is one area where the mainstream HDLs like (System)Verilog and VHDL really suffer, as it is a lot of work to define the registers manually. Two languages have already made important strides in this regard. TL-Verilog and Filament.    
[**TL-Verilog**](https://arxiv.org/abs/1811.01780) greatly simplifies the notation for pipeline creation. Instead of explicitly having to add registers on each wire, they divide the logic into pipeline stages notationally. Additionally they add several basic control flow structures, such as FIFOs and ring queues. Its notational simplicity could be considered the gold standard for 'simple' 1-clock-per-stage pipelines.   
[**Filament**](https://rachitnigam.com/files/pubs/filament.pdf) has made incredible strides in improving safety for more complex pipelines, which involve processing steps taking multiple cycles. In their paper they describe a syntax of adding Delay and hold time annotations to every signal, adding module instantiations and preventing multiple uses of the same module at the same time. They were able to create a comprehensive semantic type system that captured the full timing information for statically scheduled pipelining. 

I consider 'static pipelining' to be a solved problem. The one thing we can still innovate on in this area is combining these ideas. To encode the full semantic richness of Filament while keeping that terse notation that makes TL-Verilog shine. 

An example of such static pipeline can be shown as follows: 
```
pipeline multiply_add : i32 a, i32 b, i32 c -> i32 result {
  reg i32 tmp = a * b;
  i32 tmp2 = tmp + c;
  reg result = tmp2 + a;
}
```
Pipeline stages are denoted by adding the 'reg' keyword to statements. Either at the statement level, or to add registers within expressions. This example would then compile to the following Verilog code:
```Verilog
module multiply_add(
  input[31:0] a,
  input[31:0] b,
  input[31:0] c,
  output reg[31:0] result_DD // Note 'DD' means twice delayed signal
) {
  reg[31:0] tmp_D;
  wire[31:0] tmp2_D;
  reg[31:0] a_D; // Also need to delay a and c, to be in sync with tmp_D
  reg[31:0] c_D;

  always @(posedge clk) begin 
    tmp_D <= a * b;
    c_D <= c;
    a_D <= a;
  end
  
  assign tmp2_D = tmp_D + c_D;

  always @(posedge clk) begin 
    result_DD <= tmp_D + a_D;
  end
}
```

### Regex-Like Timeline descriptions
Often we will build modules that process a stream of data, where the operations are dependent on the order of the data. But where separate runs are still independent. IE there is no latent state between runs, as opposed to modules such as memory modules, or FIFOs, which do carry latent state. 

If we have a proper description of the timeline of our outputs, we can match our output pattern to this stream, and throw a compiler error if it doesn't. 

Making this distinction allows us to express timeline-bound operations, such as accumulators and stream processors with sufficient safety features. 

For fixed-length timelines, this was already explored in Filament. We extend that to dynamic runtime timeline length. 

Below is an example of a 2-wide blur filter. Its interface is described in the first part, and its run timeline shown on the timeline section. It takes a stream of indeterminate length. The first element is eaten without producing a result, and for all subsequent elements it outputs a result. (Note the difference between the 'state' registers that deal with the persistent data across cycles, and the 'reg' "pipeline step" operator.) This module takes a stream of length N, and outputs the first element of a stream of length (N-1) 2 clock cycles later. 
```
timeline (a, true -> /) | (a, false -> /) .. (a, false -> r)* .. (a, true -> r)
module blur : int a, bool done -> int result {
	state bool working = false; // Initial value, not a real assignment
	state int prev;

	if working {
		reg result = prev + a; // Add a pipeline stage for shits and giggles
	}
	prev = a;
	working = !done;
}
```

This could[^1] compile to the following Verilog
```Verilog
module blur(
  input valid_in,
  input[31:0] a,
  output reg valid_out,
  output reg[31:0] result
) {
  // Data path
  reg[31:0] prev;

  always @(posedge clk) begin
    prev <= a;
    result <= a + prev_a;
  end
  
  // Control path
  reg prev_valid;
  always @(posedge clk) begin
    prev_valid <= valid_in;
    valid_out <= valid_in && prev_valid;
  end
}
```

[^1]: Control signals should be fully managed by the compiler. The compiler may decide not to output a certain control signal if the target module for example doesn't require it. 

### Stricter integer types
I propose to add one generic integer type: *int<low, high>*. Instead of specifying the bitwidth of this integer, we specify its absolute range. It is not necessary to specify this range for every integer, as in most cases it can be inferred by the compiler. This inference allows the compiler to use the minimum bitwidth necessary to represent the integer. Signed integers are just integers with a negative lower bound. 

Integers come in two flavors: theorethically infinite integers, and modular arithmetic integers. 

Integer bounds should rarely have to be specified. The compiler should be able to infer them most of the time. 

Provide easy naming syntax for commonly-used integers: u8, u16, i8, i64, etc. 
Predefined integer sizes should also include things like udsp "Preferred DSP size" or something

We can add functions such as `int -> to2cpl -> bool[]` and `bool[] -> from2cpl -> int`

This also allows us to more strictly define our interfaces. Instead of requesting an int of so many bits, we request a specific range. 

Casting integers to smaller ranges is again a place where explicitly casting is required. The simulator can then check this at runtime. 

### Modules
There are no functions, every function is a module. 

Modules come in three flavors: Pipelines, multi-cycle pipelines and Modules
Basic pipeline:
```
pipeline <name>: <typ> <name>, <typ> <name>, ... -> <result_typ> <result_name>, ... {
  // Code...
}
```
multi-cycle pipelines have an additional field that describes the timeline. 
```
pipeline <name>: <typ> <name>, <typ> <name>, ... -> <result_typ> <result_name>, ... : timeline (a -> /)*..(/ -> r) {
  // Code that may utilize results across clock cycles...
}
```
Finally, true modules may have multiple interfaces, and may contain state that is kept across calls
```
module <name>: <typ> <name>, <typ> <name>, ... -> <result_typ> <result_name>, ... : timeline (a -> /)*..(/ -> r) {
  // Code that may utilize results across clock cycles...
}
```

A module is instantiated as follows: `output1, output2 = myModule(input1, input2)`

This can still change

### Clocks as language constructs
The oldest design languages such as Verilog and VHDL keep their RTL code and Timing Constraints separate. This is nice in one part, because the clock speed doesn't actually affect the theorethical functioning of the hardware. But on the other hand, once you have multiple clocks, their relative clock speed does have an effect on the actual functioning of the hardware. Clocks are just passed in as regular wires, and therefore can also be used as regular signals. 

Timing information itself should not be part of the RTL. So the clocks' absolute frequency, rise and fall times etc, those still belong in the regular constraints file. But Clocks' relative frequency, wether they're synchronous, and other constraints that directly affect the hardware such as false paths and multicycle paths should certainly be in the RTL specification itself. 

As an added benefit, hardware modules can then alter their construction based on this information, so for example, a FIFO can use a standard synchronous implementation for a single clock, but then switch to different CDC approaches for (un-)synchronized clocks. 

By including clocks in the language itself, we can then start making statements about data rates. For example a stream may be outputting on clock A, with full bandwidth, and then be transported onto clock A*2 at half its bandwidth. One neat way of expressing the signal throughput is done by [Aetherling](https://aetherling.org/). Signals are expressed as sequences of valid and invalid elements. This can then again filter out bad designs, where the bandwidth from one clock may not be carryable by another clock. 

### Integrate Timing Constraints into language text itself
- False Paths
- Multicycle Paths

Often, false paths are used to denote semi-constants that should be disseminated throughout the FPGA, or bits of hardware that won't affect each other, because only one will be active. Adding false paths relaxes the placement problem, leading to more optimal hardware implementations for the paths that matter. 

Constants specifically require that the modules the constant affect aren't being used when the constant changes. This should be representible in some way. 

### Strong Standard Library
- Avoids repeating common structures
- Refuse to rely on "inference" for hard logic blocks, instead start from the constraints inherent in these hard logic blocks to adapt the hardware around these blocks. For example hard logic registers around multiply blocks and BRAM blocks. This integrates well with streams for example. 

## Constraints

### Data Loss
- No data loss
- No new invalid data
- every read must correspond to data destruction
- data destruction must happen together with a read
- Stream Splits and merges may not lose or duplicate date

### Temporal safety
- Operations may only happen on data of the same 'time slice' within a stream
- "Happens-before" relations -> proving FIFOs

### Strong Typing
- Actual data types
- sized integers   (Min-max), not necessarily on power of 2 boundary
- representation independent integers
- Structs
- Include Rust-style enum types?
- operator overloading?

## Goals
### Formal proofs for correctness of common constructs
- Multiply-Add circuit
- Skid Buffer
- Safe Stream Split over multiple work units
- Safe Stream Merge of multiple work blocks
- FIFO
- Ready/Acknowledge Clock domain Crossing
- Ring pipeline

## Long Term Strategy
[https://www.youtube.com/watch?v=XZ3w_jec1v8]("The Economics of Programming Languages" by Evan Czaplicki (Strange Loop 2023))
