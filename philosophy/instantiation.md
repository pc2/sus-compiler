# Instantiation Modifiers
Because we have a broader vocabulary describing our modules, it becomes possible to modify instantiations of modules to add functionality. 

- Continuous (default): The module behaves like a freestanding module, inputs and outputs are expected on each clock pulse
- Push: The module only advances when instructed by the parent module. This only affects `state` registers. Latency is unaffected. 

Additional modifiers
- Latency-free: All latency registers are removed
- Set latency: between two 