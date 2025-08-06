# Debugging the SUS compiler
The SUS compiler has various ways to aid in debugging. These are listed below. All debugging code is found in [src/debug.rs](../src/debug.rs). 

## Debug Whitelist
Usually we only want to debug a specific module in which we can see that something is going wrong, but sadly, any breakpoint we place will be triggered by many other modules, and also standard library modules. By passing a debug whitelist we can restrict breakpoints and other debugging features to only occur while processing the module in question. 
Pass `sus_compiler --debug-whitelist module_name` for flattening and *all* instantiations of the `module_name`, or `sus_compiler --debug-whitelist "module_name #(PARAM_A: ..., PARAM_B: type ...)"` for a specific instantiation. (BEWARE: The parameter list must be complete and in the same order of the template args as it is printed in the SUS log. The is_enabled circuitry just does substring matching)

If no whitelist is provided then debugging is enabled globally. Outside `debug_context()` debugging is also enabled globally. 

## Debug Optional Paths
These are optional paths that can be enabled at commandline. They allow for a more detailed look into the inner workings of the compiler. In the code they are marked by `crate::debug::is_enabled("print-abstract")`. 

| Flag | Effect |
| --- | --- |
| `--debug print-abstract` | Prints the instructions+spans and their types after flattening and typechecking |
| `--debug print-abstract-pre-typecheck` | Prints the instructions+spans after flattening but before typechecking |
| `--debug print-unused-vars-map` | Used for debugging the "unused variable" warning. Prints the dependency graph of all instructions |
| `--debug print-execution-state` | After each instruction is executed, prints the whole execution state |
| `--debug print-concrete` | Prints the generated wires and their types for a given module instance after execution and concrete typechecking |
| `--debug print-concrete-pre-typecheck` | Prints the generated wires for a given module instance before concrete typechecking |
| `--debug dot-dependency-graph` | Creates a `{module_name}.dot` file with a graph representation of the generated circuit |
| `--debug print-solve_latencies-test-case` | Prints a `#[test]` case representation of the Latency Counting problem of a given module for use in [src/latency/latency_algorithm.rs](../src/latency/latency_algorithm.rs) |
| `--debug print-infer_unknown_latency_edges-test-case` | Prints a `#[test]` case representation of the Latency Inference Counting problem of a given module for use in [src/latency/latency_algorithm.rs](../src/latency/latency_algorithm.rs) |
| `--debug dot-latency-problem` | Dot debug the problem graph for latency counting in `solve_latencies_problem.dot` |
| `--debug dot-latency-solution` | Dot debug the solution graph for latency counting in `solve_latencies_solution.dot` |
| `--debug dot-latency-infer` | Dot debug the problem graph for latency inference in `latency_inference_problem.dot` |
| `--debug lsp-debug` | Instead of regular LSP hover info, provide raw debug info |
| `--debug TEST` | Temporary marker for debugging |

## Crash Dumps
When the compiler panics, it dumps the contents of the user-specified files in `~/.sus/{version}/crash_dumps`. Standard library files are not stored. Each crash is stored in a folder, named after the compiler stage, the module in which the crash took place, as well as a timestamp. Crashes can be reproduced by running `sus_compiler --no-redump` in the directory of the crash dump, as it will automatically include all .sus files in the directory. `--no-redump` should be used such that no duplicate dumps are created when reproducing the crash. 

Crashes can also be debugged on master with the "Debug Crash Dump" [.vscode/launch.json](../.vscode/launch.json) configuration. 

## Timeout crash
`sus_compiler --kill-timeout 2.0` will enable a timeout killer that kills the compiler if any `debug_context()`-ed context takes longer than 2s to complete. Useful to prevent runaway LSPs

## Breakpoints, debug prints, spans
All debug features abide by `--debug-whitelist`
| Name | Use |
| --- | --- |
| `crate::debug::debug_context("typechecking", "module_name", \|\| {...})` | Everything inside the lambda runs within a debug context, enabling debugging if any item in `--debug-whitelist` is a substring of `module_name`. Can be nested. |
| `if crate::debug::is_enabled("name") {...}` | Check if `--debug name` is used |
| `__debug_breakpoint!()` | breakpoint |
| `__debug_breakpoint_if!(x == 5)` | conditional breakpoint |
| `__debug_span!(span, "This here")` | Show a span visually |
| `__debug_dbg!(obj_a, obj_b, ...)` | Equivalent to Rust's `dbg!(obj_a, obj_b, ...)` |
