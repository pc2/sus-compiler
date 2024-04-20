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
module m : int a, bool b -> int c {
    if a == 5 {
        c = 4
    } else if b {
        c = 2
    } else {
        c = 1
    }
}
```

## `for`
The `for` statement only comes in its generative form. It's used to generate repetitive hardware. 

#### Example
```verilog
module add_stuff_to_indices : int[10] values -> int[10] added_values {
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
module first_enabled_bit : bool[10] values -> bool[10] is_first_bit {
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
module first_enabled_bit : bool[10] values -> bool[10] is_first_bit {
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
module first_enabled_bit_index : bool[10] values -> int first_bit, bool all_zero {
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
