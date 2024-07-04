# On State
For latency see [latency](latency.md)

State goes hand-in-hand with the flow descriptors on the ports of modules. Without state all a module could represent is a simple flow-through pipeline. 

But once we introduce state, suddenly modules can have a wide range of output patterns and required input patterns. A simple example would be a data packer or unpacker. An unpacker receives a data packet, and outputs its contents in parts over the next N cycles. How should this unpacker behave when it receives another data packet before it finishes? It can either discard what it's currently working on, or discard the incoming data. Either way, data is lost. So the packer's interface must prohibit incoming data for N-1 cycles after a valid packet. 

The language we choose for the interfaces is that of the regex. This is a natural choice, since in effect any module the user writes is a state machine, and regexes can be converted to state machines. State machines have a nice property, that operators for working with state machines are polynomial and easy to understand.

## Structural and Data State
We have to check the state machine that is each module against the state machines of the modules it uses of course. Sadly, this checking can only really be done in a generic way by generating the full module state machine, and checking its behavior against the state machine from its dependents' interfaces, as well as its own. 

Generating the whole state machine is a combinatorial endeavour however, and a too wide state vector quickly leads to an unmanageable number of states. This encourages us to differentiate between two types of state. Structural State (namely state whose instances are incorporated into the module STM), and Data State, which (aside from its validity) is not. We wouldn't care about every possible bitpattern of a floating point number we happened to include in our state right?

## Examples
### Summing module
```Verilog
timeline (X, false -> /)* .. (X, true -> T)
module Accumulator {
    interface Accumulator : int term, bool done -> int total 
    state int tot := 0 // Initial value, not a real assignment

    int new_tot = tot + term
    if done {
        total = new_tot
        tot = 0
        finish // packet is hereby finished. 
    } else {
        tot = new_tot
    }
}
```

In this case the compiler would generate a state machine with two states. One state for when the module is active, and one is generated implicitly for the inactive case. The regex is mapped to a 3-state state machine. Represented below:

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

The code's state machine must be proven equivalent to the regex state machine. This is done by simulating the code STM based on the regex. The code must properly request inputs at regex states where inputs are provided, and may not when not. It's inputs must be valid for _any_ path in the regex STM, while it's outputs must conform to _some_ path of the regex. 

Any module working on finite packet sizes must also specify the `finish` keyword when the module is finished sending a packet. 
At this point the initial conditions must be reestablished explicitly. After this, the module goes back into the inactive state. 

In this example, the code simulation starts right in its initial state. Then the different paths of the regex STM are all simulated. For the case of infinite loops, we save any distinct (regex, code-STM) pair we come across, and skip combinations we've already come across. 

Since in this example the only active state for the code corresponds to both active states of the regex, the code must abide by the constraints of both regex paths. And it does, in the case `done == false` the module may not output `total` Likewise, in the case `done == true`, the module *must* output `total`. And in the case of `done == true`, the code has to go back to the initial state through the `finish` keyword. 

The caller is then responsible for providing a stream of the form of the regex. 

## Unpacker
The previous example was quite simple though, with the code's active state machine containing only one state. In this example we explore a module that does have structural state. 

```Verilog
timeline (X -> X) .. (/ -> X) .. (/ -> X) .. (/ -> X)
module Unpack4<T> {
    interface Unpack4 : T[4] packed -> T out_stream 
    state int st := 0 // Initial value, not a real assignment
    state T[3] stored_packed

    if st == 0 {
        out_stream = packed[0]
        stored_packed[0] = packed[1] // Shorthand notation is possible here "stored_packed[0:2] = packed[1:3]"
        stored_packed[1] = packed[2]
        stored_packed[2] = packed[3]
        st = 1
    } else if st == 1 {
        out_stream = stored_packed[0]
        st = 2
    } else if st == 2 {
        out_stream = stored_packed[1]
        st = 3
    } else if st == 3 {
        out_stream = stored_packed[2]
        st = 0
        finish // packet is hereby finished. 
    }
}
```

In this case, the regex has 4 states, but we don't know what number of states the code has. One could bound the integer `st` of course, and for the number of states multiply together the counts of all structural state objects we find. But we don't need to. We can simply simulate the code, only explicitly saving the structural state fields. 

In this case, we know the starting value of `st`, and we just need to simulate the hardware with this. So in the first cycle, we are obligated to read from `packed`, and write to `out_stream`. Following the code that is the case, as we execute the first branch: `st == 0`. We know the next state `st = 1`, so we continue going along. This continues for the remaining states of the regex, ending at `st == 3` where we also call `finish`. 

