# Control Flow
While hardware doesn't actually have control flow, it is often useful to have _generative_ control flow for repeating hardware structures. 

## `if`
The humble if statement is the most basic form of control flow. It comes in two flavors: generation time, and runtime. 

### Generation time if
If its condition is met, then the code in its block is executed. It can optionally have an `else` block, which then also allows chaining `else if` statements. 

#### Example
```verilog
gen int a
gen bool b
if a == 5 {
    // Do the first thing
} else if b {
    // otherwise do something here
} else {
    // etc
}
```

### Runtime if
In practice, the biggest difference between these variants is that the code within both branches of the runtime if is executed regardless of the condition. Only assignments are performed conditionally. This means any assignments within the block will have a combinatorial dependency on the condition wire. To avoid confusion, it is not allowed to assign to generative variables within a runtime if. 

#### Example
```verilog
module m {
    interface m : int a, bool b -> int c 
    if a == 5 {
        c = 4
    } else if b {
        c = 2
    } else {
        c = 1
    }
}
```

### Conditional bindings

In hardware design, pretty much all data signals will be coupled with `valid` signals. Having dedicated syntactic sugar for this is thus valuable to lift some mental load for the hardware designer. 

As an example, take the `pop` interface of a FIFO. 
```verilog
interface pop : bool do_pop -> bool pop_valid, T data
```

This is both an action (setting the `do_pop` signal), but also may fail (`pop_valid`). Both control signals can be hidden with this syntactic sugar. Furthermore, the output data of the FIFO is only available when the pop was successful. This adds nice implicit semantics that for example the formal verifier could then check. 

```verilog
FIFO myFifo
if myFifo.pop() : T data {
    ...
}
```
Which is equivalent to this:
```verilog
FIFO myFifo
myFifo.do_pop = true
if myFifo.pop_valid {
    T data = myFifo.data_out
    ...
}
```

This syntax can also be used to approximate imperative control flow. We would want something in hardware _like_ the lambda functions in software, but what semantics should they have? As a first approximation, we can have the submodule 'trigger' some of our hardware using this validity logic. In this example we use a submodule that generates an index stream of valid matrix indices, and calls our code with that:
```verilog
MatrixIterator mit

state bool start
initial start = true

if start {
    mit.start(40, 40)
    start = false
}

if mit.next() : int x, int y {
    ...
}
```

Finally, this might be a good syntax alternative for implementing Sum Types. Sum types map weirdly to hardware, as their mere existence may or may not introduce wire dependencies on the variants, depending on how the wires were reused. Instead, we could use these conditional bindings to make a bootleg match:

```verilog
if my_instruction.is_jump() : int target_addr {
    ...
}
if my_instruction.is_add() : int reg_a, int reg_b, int reg_target {
    ...
}
```

## `for`
The `for` statement only comes in its generative form. It's used to generate repetitive hardware. 

#### Example
```verilog
module add_stuff_to_indices {
    interface add_stuff_to_indices : int[10] values -> int[10] added_values 
	int[5] arr
	for int i in 0..10 {
		int t = values[i]
		added_values[i] = t + i

		int tt = arr[i] + values[0]
	}
}
```

## `while`
Similar to the `for` loop. Also generation only. **Not yet implemented.**

## `chain` and `first`
the `chain` construct is one of SUS' unique features. **Not yet implemented.**

Often it is needed to have some kind of priority encoding in hardware. It only fires the first time it is valid. 

As a bit of syntactic sugar, the `first` statement uses a chain to check if it's the first time the condition was valid. 

It comes in two variants: standalone `first` and `if first`. 

#### Examples
```verilog
module first_enabled_bit {
    interface first_enabled_bit : bool[10] values -> bool[10] is_first_bit 
    chain bool found = false
	for int i in 0..10 {
        if values[i] {
            first in found {
                // First i for which values[i]==true
                is_first_bit[i] = true
            } else {
                // values[i]==true but not the first
                is_first_bit[i] = false
            }
        } else {
            // values[i]!=true
            is_first_bit[i] = false
        }
	}
}
```

With `if first` we can merge both `else` blocks. 

```verilog
module first_enabled_bit {
    interface first_enabled_bit : bool[10] values -> bool[10] is_first_bit 
    chain bool found = false
	for int i in 0..10 {
        if first values[i] in found {
            // First i for which values[i]==true
            is_first_bit[i] = true
        } else {
            // values[i]!=true or not first values[i]==true
            is_first_bit[i] = false
        }
	}
}
```

Often with uses of `first` one also wants to have a case where the condition never was valid. 

```verilog
module first_enabled_bit_index {
    interface first_enabled_bit_index : bool[10] values -> int first_bit, bool all_zero 
    chain bool found = false
	for int i in 0..10 {
		if first values[i] in found {
            first_bit = i
            all_zero = false
        }
	}
    if !found {
        all_zero = true
    }
}
```
