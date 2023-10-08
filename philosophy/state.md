

# On Registers

## State vs Latency

In my experience, the use of registers usually boils down to two use cases: 
- Representing a current working state, which gets updated across clock cycles
- Improving timing closure by introducing registers on tight paths. 

While this distinction exists in the programmer's mind, it isn't in the vocabulary of common compilers. Verilog and VHDL just call both 'reg' (And non-registers too, but that's another can of worms.) 

Philosophically, the difference is quite important though. Registers that are part of the state are critical, and they directly direct the functioning of the device. While latency registers should not affect the functioning of the design at all, aside from trivially affecting the latency of the whole design. Some would argue that worrying about latency registers is a solved problem, with retiming tools that can automatically migrate latency registers across a design to place them wherever more timing slack is required. In practice though, this capability is limited, usually by explicitly marking specific paths as latency insensitive, or in a limited way by synthesizing a block of registers somewhere, which should then be migrated across the design. Still, this practice is always limited by the first design register it comes across along the path. Explicitly differentiating between state and latency registers could make this automatic retiming much more powerful. 

While indeed generally latency can't affect the actual operation of the device, it can be disallowed in certain circumstances. Certain paths are latency sensitive, and would no longer produce correct results if latency were introduced. A trivial example is any kind of feedback loop. In this case, no latency can be introduced within the feedback loop itself, as the result for the current feedback loop cycle wouldn't arrive in time. In this case the latency should either be forbidden, or reincorporated in a different way, such as interpreting the state loop as a [C-Slowed](https://en.wikipedia.org/wiki/C-slowing) state loop. 

## Latency Counting
Inserting latency registers on every path that requires them is an incredibly tedious job. Especicially if one has many signals that have to be kept in sync for every latency register added. This is why I propose a terse pipelining notation. Simply add the `reg` keyword to any critical path and any paths running parallel to it will get latency added to compensate. This is accomplished by adding a 'latency' field to every path. Starting from an arbitrary starting point, all locals connected to it can then get an 'absolute' latency value, where locals dependent on multiple paths take the maximum latency of their source paths. From this we can then recompute the path latencies to be exact latencies, and add the necessary registers. 

Example:
```
(start - 0)
A -----------+-- reg -- reg --\
(-1)        /                  +-- C (2)
B -- reg --/------------------/
```

### Latency counting with state
Of course, state registers are also moved around by latency. This means that while it appears like two state modules get updated at the same time, if they are independent they need not. 

However, state registers should not count towards the latency count. So specifying `reg reg` should increase the latency count by 2, but specifying `state` does not. This makes sense, because this means a feedback loop to a state register has a latency of 0, which it requires to stay within. Also, this maintains that by removing all latency registers, the total latency count becomes 0 on all ports. 

If this rule holds for all possible hardware designs is up for further research. 

## On State
State goes hand-in-hand with the flow descriptors on the ports of modules. Without state all a module could represent is a simple flow-through pipeline. 

But once we introduce state, suddenly modules can have a wide range of output patterns and required input patterns. A simple example would be a data packer or unpacker. An unpacker receives a data packet, and outputs its contents in parts over the next N cycles. How should this unpacker behave when it receives another data packet before it finishes? It can either discard what it's currently working on, or discard the incoming data. Either way, data is lost. So the packer's interface must prohibit incoming data for N-1 cycles after a valid packet. 

The language we choose for the interfaces is that of the regex. This is a natural choice, since in effect any module the user writes is a state machine, and regexes can be converted to state machines. State machines have a nice property, that operators for working with state machines are polynomial and easy to understand.

### Structural and Data State
We have to check the state machine that is each module against the state machines of the modules it uses of course. Sadly, this checking can only really be done in a generic way by generating the full module state machine, and checking its behavior against the state machine from its dependents' interfaces, as well as its own. 

Generating the whole state machine is a combinatorial endeavour however, and a too wide state vector quickly leads to an unmanageable number of states. This encourages us to differentiate between two types of state. Structural State (namely state whose instances are incorporated into the module STM), and Data State, which (aside from its validity) is not. We wouldn't care about every possible bitpattern of a floating point number we happened to include in our state right?

### Examples
#### Summing module
```Verilog
timeline (X, false -> /)* .. (X, true -> T)
module Accumulator : int term, bool done -> int total {
    state int tot init 0;

    int new_tot = tot + term;
    if done {
        total = new_tot;
        tot = 0;
    } else {
        tot = new_tot;
    }
}
```

In this case the compiler would generate a state machine with one state. The regex is mapped to a 3-state state machine. Represented below:

- A: `inactive`
- B: `(X, false - /)`
- C: `(X, true - T)` 

The regex produces the following NFA: (-> is a consuming transition, => is not)
- A -> A when !valid
- A => B when valid
- B -> B when !done
- B => C when done
- C -> A

Compiled to a DFA this gives:
- A -> A when !valid
- A -> B when valid & !done
- A -> C when valid & done
- B -> B when !done
- B -> C when done
- C -> A when !valid
- C -> B when valid & !done
- C -> C when valid & done

This state machine m
These two state machines must be proven equivalent. There must be exactly one edge-preserving mapping from the regex to the code. This means, each code state should uphold the constraints of all regex states that map to it. There may be no additional reachable edges. 

Finally the initial conditions must be reestablished on any edge back to inactive. 

In this example all three states are mapped on the single code state. So the code must abide by all their constraints. And it does, in the case `done == false` the module may not output `total` Likewise, in the case `done == true`, the module *must* output `total`. 

The caller is then responsible for providing a stream of the form of the regex. 
