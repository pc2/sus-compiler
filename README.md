# SUS Language
A Hardware Description Language focussed on strong type and temporal safety features

Main Inspirations: TL-Verilog, Rust

## Core philosophy
This project is an attempt to create a safety-first, correct-by-default yet still with low level control HDL much like Rust is for the software industry. 

Current HDLs mostly build on top of existing Software languages such as Chisel and SpinalHDL. This allows for great software integration, but throws away a lot of the terseness and extended type safety that HDLs could benefit from. 

A great and interesting new innovation is TL-Verilog. In this language they built a higher level abstraction for designing hardware, moving away from the Register-Transfer level to a pipeline-focussed design. This makes TL-Verilog incredibly well-suited for the development of processing pipelines. What holds TL-Verilog back from being the language that accomplishes the goals of safety, and terseness. 

- Strong Typing
- Eliminate common issues
- Streams with info. Free flowing, Slowdown, stall streams
- Temporal safety
- Ease of creating and fine-tuning processing pipelines
- Easy to test with software integration
- Better visualization of data flow --> Eliminate Wave plots
- Integrate most timing analyzer constraints into source files themselves. False paths, Clocks, multicycle paths, etc. All belong in the RTL specification itself, not in the timing constraints. 

### Terseness (Similar to many current HDLs, such as Chisel)
- Bundles
- Interfaces
- Handle control signals with streams
- Clocks are handled with dedicated syntax
- Syntactic sugar for Resets

### Integrate Timing Analizer constraints into language
- False/multicycle paths

## Features

### Easy Pipelining
Critical for achieving high frequencies. Computation is split up over multiple stages split by registers, such that multiple operations can be 'coming down the pipe' at the same time. This is one area where the mainstream HDLs like (System)Verilog and VHDL really suffer, as it is a lot of work to define the registers manually. Two languages have already made important strides in this regard. TL-Verilog and Filament.    
[**TL-Verilog**](https://arxiv.org/abs/1811.01780) greatly simplifies the notation for pipeline creation. Instead of explicitly having to add registers on each wire, they divide the logic into pipeline stages notationally. Additionally they add several basic control flow structures, such as FIFOs and ring queues. Its notational simplicity could be considered the gold standard for 'simple' 1-clock-per-stage pipelines.   
[**Filament**](https://rachitnigam.com/files/pubs/filament.pdf) has made incredible strides in improving safety for more complex pipelines, which involve processing steps taking multiple cycles. In their paper they describe a syntax of adding Delay and hold time annotations to every signal, adding module instantiations and preventing multiple uses of the same module at the same time. They were able to create a comprehensive semantic type system that captured the full timing information for statically scheduled pipelining. 

I consider 'static pipelining' to be a solved problem. The one thing we can still innovate on in this area is combining these ideas. To encode the full semantic richness of Filament while keeping that terse notation that makes TL-Verilog shine. 

Perhaps one thing that could still be useful here is 

### Clocks as language constructs
The oldest design languages such as Verilog and VHDL keep their RTL code and Timing Constraints separate. This is nice in one part, because the clock speed doesn't actually affect the theorethical functioning of the hardware. But on the other hand, once you have multiple clocks, their relative clock speed does have an effect on the actual functioning of the hardware. Clocks are just passed in as regular wires, and therefore can also be used as regular signals. 

Timing information itself should not be part of the RTL. So the clocks' absolute frequency, rise and fall times etc, those still belong in the regular constraints file. But Clocks' relative frequency, wether they're synchronous, and other constraints that directly affect the hardware such as false paths and multicycle paths should certainly be in the RTL specification itself. 

As an added benefit, hardware modules can then alter their construction based on this information, so for example, a FIFO can use a standard synchronous implementation for a single clock, but then switch to different CDC approaches for (un-)synchronized clocks. 

By including clocks in the language itself, we can then start making statements about data rates. For example a channel may be outputting on clock A, with full bandwidth, and then be transported onto clock A*2 at half its bandwidth. One neat way of expressing the signal throughput is done by [Aetherling](https://aetherling.org/). Signals are expressed as sequences of valid and invalid elements. This can then again filter out bad designs, where the bandwidth from one clock may not be carryable by another clock. 

### Strong Standard Library
- Avoids repeating common structures
- Refuse to rely on "inference" for hard logic blocks, instead start from the constraints inherent in these hard logic blocks to adapt the hardware around these blocks. For example hard logic registers around multiply blocks and BRAM blocks. This integrates well with streams for example. 

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
- 