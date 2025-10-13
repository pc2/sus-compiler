# Changelog
## 0.3.3-devel
- Can now bake in custom SUS_HOME directories into compiler while building: `INSTALL_SUS_HOME=/path/to/home/ cargo install sus_compiler`. Also, default SUS_HOME is now $XDG_DATA_HOME. (#104)
- SUS_HOME is now controllable via cli override `--sus-home`, env var `$SUS_HOME`, and falls back to the baked-in `$INSTALL_SUS_HOME` set while installing. 
- Crash dumps try to dump in `$SUS_HOME/crash_dumps`, if `SUS_HOME` is ready-only they dump to `./sus_crash_dumps`
- Fix crash when crash dump name too long (ironic, isn't it?)
- Replace `--codegen`, `--standalone`, `--standalone-file` with `-o` and `--top` (#123)
- Fix ICE on assign `[]` to `bool[3][4]` (#124)
- Fix double underscores in codegen (#128)
- Better display of IO errors (#125) (#98)
- Work around Vivado sim wrong result for single-bit combinatorial assigns (#127)

## 0.3.2
- Execute: Add `BitsToUIntGen`, `BitsToIntGen`, `UIntToBitsGen` and `IntToBitsGen` for compiletime `gen bool[] <-> gen int` conversion (#120)
- Codegen: Fix incorrect codegen for negative int literals (#117)
- Dot output: Replace `shape=record` with HTML tables, to sidestep graphviz bug (#116)
- Use logging library for prints -> All prints now go over stderr instead of stdout
- LSP: Add `--stdio` support, so no more need for TCP LSP. LSP now enforces minimum sus_compiler version of 0.3.2
- LSP: Fix crash when renaming module inputs/outputs (#119)
- LSP: Fix crash when renaming files (#118)

## 0.3.1
- Add VIM & NeoVim LSP support (#113) (Thanks @papeg!)
- Downgrade to Rust 1.88 to use on EasyBuild-powered clusters
- Add float literals
- Codegen: Invalid use of `{}` converted to `'{}` #112
- Codegen: Inlining of constants was to eager (#110 and #111)
- `--standalone` now properly generates `./verilog_output` if it doesn't exist. (#109)
- When bindings now properly bind binding domain to condition domain (#97)

## 0.3.0: The Integral Update
### Standard Library is delivered with the compiler
This means from now on `cargo install sus_compiler` will create a new directory in `~/.sus/{version}` (so now `~/.sus/0.3.0/` that contains the standard library. It also contains a folder `crash_dumps` to which crash dumps are saved for easier debugging. You can change this by passing `sus_compiler --sus-home /new/path/to/other/home/`, but it's mostly used to support development. 

### Subtyping System
Subtyping currently only occurs for integer parameters, and the only subtyping rules are "equality" (such as for array sizes), and "<=" and ">=" which are currently only used for integer subtyping (`int #(FROM: F_A, TO: T_A)` can be assigned to `int #(FROM: F_B, TO: T_B)` iff `F_A >= F_B && T_A <= T_B`.)

Described more formally in #87 . 

A parameter in targeted by subtyping constraints will be inferred to the MIN of all possibilities for `<=`, and MAX of all possibilities for `>=`. 

### Actions, Triggers and Conditional Bindings
These function as syntactic sugar for common hardware constructs. A "fire" boolean, attached to some data that is valid when "fire" is '1'. 

```sus
/// Declaration of Actions
module memory#(T, int DEPTH) {
    T[DEPTH] mem

    action write: int#(FROM: 0, TO: DEPTH) addr, T data {
        mem[addr] = data
    }
    action read: int#(FROM: 0, TO: DEPTH) addrb -> T datab {
        datab = mem[addrb]
    }
}
/// Calling of Actions
module use_memory {
    memory#(T: type bool[20], DEPTH: 5) mem

    state int cur_idx
    action get_next : -> bool[20] d {
        d = mem.read(cur_idx)
        cur_idx = (cur_idx + 1) % 5
    }
}

/// Declaration of triggers
module iterator#(int MAX) {
    state int cur

    action start {
        cur = 0
    }
    trigger iter : int v, bool last

    when cur != MAX {
        iter(cur, cur == MAX - 1)
        cur = (cur + 1) % MAX
    }
}
/// Use of triggers and conditional bindings
module use_iter {
    int[6] terms = [5, 7, -9, 6, 5, 2]
    state int total

    iterator#(MAX: 6) iter
    action sum_up {
        total = 0
        iter.start()
    }
    trigger done : int sum
    when iter.iter : int idx, bool last {
        int new_total = (total + terms[idx]) % 256
        when last {
            done(new_total)
        }
        total = new_total
    }
}
```

### Bounded Integers
With SUS 0.3.0, we finally support bounded integers. (Before integers would always be 32 bit, with no semantics around operators). 

The type of a bounded int is written as `int #(FROM: 0, TO: 256)` for an 8-bit integer (`FROM` is inclusive, `TO` is exclusive). For general N-bit integers, you can use `int #(FROM: 0, TO: pow2#(E: 32))` for a 32-bit integer. 

With bounded integers now also comes bounds checking for arrays :tada:. 

The typechecking system for integers uses subtyping rules for assigning integers, so any integer can be assigned to integers of equal or greater bound. The inference system can also figure out the type of an int if all writers are known. 

### Latency Inference refinement
The Latency Inference system has been refined significantly. It now uses the same subtyping system as the Bounded Integers, instead of its own ad-hoc inference system. 

With this came a push for much-improved visibility into the inference process. Inference errors and hover info will now display why an inference couldn't be performed (either "bad problem" for a latency problem which has a net-positive latency cycle, "not reached" for when the output isn't connected to the input, or "poisoned by ..." which means it couldn't be inferred due to another module not yet being resolved). 

### Rank-Polymorphism
We can now apply binary operators to multidimensional arrays of ints & bools, instead of needing to resort to for loops on scalars. 

So `bool[25] & bool[25] => bool[25]` of AND-ed bools. 

### Multidimensional Array Slicing
The work on rank polymorphism has also allowed us to add array slicing, and immediately we've been able to include multidimensional slicing. Likewise slicing part-select is now also in. 

Example: 
```sus
int#(FROM: 0, TO: 15) x
bool[20][30][40] bool_tensor
bool[3][5] = bool_tensor[30:33][27][x +: 5]
```

## Minor Changes
- Add [x, y, z] array construction expression
- Add `pow2, pow, clog2, comb, factorial, falling_factorial, noinfer, sizeof` STL builtin math functions
- Add `transmute_to_bits`, `transmute_from_bits`, `unsafe_int_cast`
- Switch to IBig instead of num-bigint for small integer optimization. 
- Add dot debugging output for debugging Latency Counting problems. 
- Add OutOfTimeKiller to kill LSP if it takes too long. 
- Basic Xilinx XPM and float wrappers
- Update to Rust 1.89

**Full Changelog**: https://github.com/pc2/sus-compiler/compare/v0.2.1...v0.3.0

Large portions of the work in this release are thanks to the generous contributions of @IBims1NicerTobi and @pbeart

## 0.2.1
- Update ariadne 0.4.1 => 0.5.1 to fix Multiline errors
- Switch to Bellman-Ford for Latency Counting
- Latency Counting now covers any fully-connected graph

## 0.2.0
- Add Template Inference
    - Type
    - Generative Value
    - Latency Count Inference #69
- `extern` verilog modules can now have template arguments #42
- Add if/when distinction #43
- Add `assert`, `clog2` and `sizeof`
- Rename standard library: stl => std
- Add doc-comments #63
- Generated SystemVerilog now uses unpacked arrays to represent SUS arrays, instead of the original packed arrays. 
- https://sus-lang.org is now live!

### Technical Changes
- Hindley-Milner for Concrete Typing
- Submodule Instantiation done During Concrete Typing
- tree-sitter-sus has been merged into sus-compiler and is no longer a separate repository
- Rewrote HM Unifier because it didn't properly handle infinite types #55
- Add test.sus_regression.sh testing to CI
- Fix test.sus_regression.sh was not OS independent #40
- The CI now enforces rustfmt and clippy warnings #62
- Fix double Panics on Drop #45
- Update tree-sitter to 0.24.7 #54
- Update Clap to 4.5 #32
- Many refactors

### SUS-LSP changes:
- Disable restarting on compiler crash for easier debugging
- Add `when` keyword

## 0.1.1
- Change sus stl installation directory to $HOME/.sus/VERSION/stl

## 0.1.0
- Standard library is shipped with the compiler now in the [stl/](stl/) directory. 
- Changed template definition and usage syntax to named arguments in `#()` instead of `::<>` (See [philosophy/template_troubles.md](philosophy/template_troubles.md))
- Fix #16: Incorrect operator prescedence for | and &
- Show template arguments on hover
- Add --nocolor for terminals that don't support it
- Add --upto (flatten, typecheck, lint, instantiation, etc) for improved debugging
- Add --debug-whitelist for --debug-ing specific modules
- Add output & error regression test: [test.sus_regression.sh]
### Technical Changes
- Lints are now a separate compile stage
- For Abstract Types and Domains, switch to Hindley-Milner type checking
- All builtin names (bool, int, true, false) are now defined in [stl/core.sus](stl/core.sus). This is so the templating system works for all of them. 
- Since link_info is now shared between Modules, Types and Constants, we now share all code for templating, typing, etc. 
- Instructions are now part of LinkInfo. 
