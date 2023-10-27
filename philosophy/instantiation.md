# Instantiation Modifiers
Because we have a broader vocabulary describing our modules, it becomes possible to modify instantiations of modules to add functionality. 

- Continuous (default): The module behaves like a freestanding module, inputs and outputs are expected on each clock pulse
- Push: The module only advances when instructed by the parent module. This only affects `state` registers. Latency is unaffected. 

Additional modifiers
- Latency-free: All latency registers are removed
- Set latency: sets the latency between two connectors (ports, locals, fields etc), adding or removing latency registers as needed. Mustly used to override latency for tight feedback loops. 

# Structs and Modules and Constants
So structs, modules and constants all very much look alike in a certain sense. But modules must be distinct from structs and constants. Because Modules *cannot* be freely copied or moved around. But, one would want it possible to instantiate modules in arrays. 

## Templates

## Template - Flow - Lifetime dichotomy
When instantiating 
