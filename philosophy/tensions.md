# Implementation Tensions

## HW Design wants as much templating as possible --- Turing-Complete code generation can't be generically checked
### Solutions
- Don't analyze Templated Code before instantiation (C++ way)
- Default Args required, do user-facing compile based on these. 
- Limit Code Generation to a limited subset that can be analyzed generically. (Lot of work, will eliminate otherwise valid code)

## Compilation Ordering: Code Generation --- Flow Analysis --- Latency Counting
Most of the time, Latency Counting is dependent on Template Instantiation. For example, a larger Memory may incur more latency overhead for reads and writes. 

On the other hand, one could want a measured latency count to be usable at compile time, to generate hardware that can specifically deal with this latency. For example, a FIFO's almostFull threshold. 

Another important case: Automatic Generation of compact latency using BRAM shift registers. The compiler could instantiate a user-provided module with as template argument the latency between the bridging wires, but the module may then add vary in its latency depending on the memory block size, requiring the compiler to again fiddle with the template argument. 

### Solutions
- Always compile in order Template Instantiation -> Flow Analysis & Latency Counting. Explicitly state template args. Add asserts for latencies that are too high and reject designs that violate this. 
- For each nested module separately, perform Template Instantiation & Latency Counting first, and allow use of measured latency of higher modules
- Add latency template arguments, allowing higher modules to set the latency of lower modules. Reject lower modules that violate this latency. 
