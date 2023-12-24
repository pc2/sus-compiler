# Combinatorial Dependency

Still a vague idea. 

Ports in interfaces are combinatorially dependent on each other. To break combinatorial dependency, one has to use multiple interfaces. 

`module m : int a, int b -> int out`

`out` is combinatorially dependent on `a` and `b`
