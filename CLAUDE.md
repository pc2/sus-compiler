# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`sus_compiler` is the compiler for SUS, a synchronous RTL hardware design language. It takes `.sus` files and emits SystemVerilog. The same binary also runs as an LSP server (`--lsp`, default feature `lsp`). Cargo workspace with three crates: the compiler (`src/`), `tree-sitter-sus` (the grammar + generated parser), and `sus-proc-macro`.

## The SUS language in brief

Read `../sus-website/book/src/docs/` (if checked out) or https://sus-lang.org/docs/ for detail. The essentials:

- **Modules and ports.** `module Name#(T, int N) { ... }`. Continuous ports are `input T x` / `output T y`; a parent reads/writes them as fields (`inst.x = ...`). No semicolons. Statements are declarations, assignments, `when`/`if`/`for` blocks.
- **Actions and triggers** are the preferred port style. `action name: T a, T b -> T out { body }` is sugar for a `bool` input called `name` (the "valid" wire) plus data inputs/outputs and a `when name { body }`. The parent calls it like a method: `out = inst.name(a, b)`. A `trigger name : T v` is the reverse: a `bool` output called `name` plus data outputs; the submodule fires `name(v)`, and the parent reacts with a **conditional binding** `when inst.name : T v { ... }`. Both represent physical wires, so they fire at most once per cycle. Convention: a `may_x` output gates whether `x()` may be called (see the `may_x then x` idiom).
- **`interface name : inputs -> outputs`** still appears in `std/` and `test_old.sus` but is no longer really used; the author prefers `action`/`trigger`, so do not write new code with it.
- **Generative (compile-time) code.** `gen int X = ...` variables, `if` (compile-time) and `for int I in 0..N` loops run during instantiation and leave no hardware. Runtime conditionals use `when`, which becomes guards on the assignments inside. Module parameters `#(T, int N)` are generative; type args are passed as `#(T: type bool[4], N: 5)` and may be inferred from port types, integer bounds (min/max constraints), or latency inference.
- **Latency counting** is the core feature. `reg x = ...` requests a pipeline register; the compiler assigns every wire an absolute latency and inserts registers on parallel paths to keep them in sync. `'N` on a declaration (`input bool i'0`, `output bool o'5`) pins an absolute latency. Ports are placed so inputs are as late and outputs as early as possible, and the solution is unique up to a constant offset. `reg(N)` and `next(N)` are the counted variants. `state T x` declares a register that holds across cycles (`initial x = v` sets its reset value).
- **Domains and clocks.** `domain name` (latency domain) and `clock name` (clock domain, implies a latency domain) apply to all ports declared lexically after them. Wires that connect must share a domain; crossing requires `CrossDomain` (unsafe plain connection) or vendor primitives. Latency domains stop unrelated interfaces (e.g. FIFO push vs pop) from being pipeline-balanced against each other.
- **`split`** on an array declaration makes each outer index a separate wire with independently inferred latency/size/bounds, used to break net-positive latency cycles in cascades.
- **Types.** `bool`, `int#(a..b)` (shorthand for `int#(FROM: a, TO: b)`; the other slice forms `a..=b`, `a+..w`, `a-..w`, `a*..w` work too; inclusive/exclusive, signed iff `FROM < 0`, width derived from bounds; generative ints are unbounded), `float`, `double`, gen-only `string`, arrays `T[N]`. Operators broadcast over arrays (rank polymorphism); unary `&`/`|`/`^`/`+`/`*` are reductions (and/or/xor/sum/product). There are no `&&`/`||`; boolean `&`/`|` bind looser than comparisons.
- **`extern module`** declares a SystemVerilog module to wrap; all port types and latencies must be explicit. `__builtin__` declarations in `std/core.sus` have no SUS body: builtin constants (`sizeof`, `clog2`, ...) are evaluated in `src/instantiation/builtins.rs`, builtin modules (`LatencyOffset`, `CrossDomain`) are emitted directly by `src/codegen/system_verilog.rs`.

## Commands

```sh
cargo build                                  # debug build, includes LSP
cargo build --no-default-features            # faster: no LSP deps
cargo test --all-features                    # unit tests (they live inline in src/, no tests/ crate)
cargo test --all-features name_of_test       # single test
cargo fmt --check                            # CI enforces
RUSTFLAGS="-A unused" cargo clippy -- -Dwarnings   # CI enforces (exactly this invocation)
./tinyTest.sh                                # compile tinyTestFile.sus (scratch file for quick experiments)
cargo run -- myFile.sus -o out.sv --top MyTop
```

CI pins Rust 1.88 (`rust-version` in Cargo.toml); edition is 2024 and `let ... && let` chains are used.

### Regression test (required for behavior changes)

The regression input is three files: `test.sus` (modern tests), `test_old.sus` (tests using the old `interface` syntax, and tests that produce a lot of diagnostics) and `test_latencies.sus` (latency counting tests). Their combined outputs `test.sus_errors.txt` (all diagnostics) and `test.sus_codegen.sv` are committed, and CI diffs against them. Any change that alters diagnostics, codegen, or these input files must regenerate and commit them:

```sh
./test.sus_regression.sh
git diff test.sus_errors.txt test.sus_codegen.sv   # inspect before committing
```

CI also requires stdout to be empty during that run.

### Hardware simulation tests

`tests/*/run.sh` compile a `.sus` file and run a Vivado simulation (`sim.tcl`). They need Vivado on PATH and are not run in CI.

### Editing the grammar

`tree-sitter-sus/grammar.js` is the source; `tree-sitter-sus/src/parser.c` is generated and committed. Regenerate with `tree-sitter-sus/tree.sh`, which requires `tree-sitter-cli` **exactly v0.26.9** (`cargo install tree-sitter-cli@0.26.9`). Node kinds, keywords and fields are referenced in Rust via the `kind!("...")`, `kw!("...")`, `field!("...")` proc macros, which validate the name against the grammar at compile time.

### Other scripts

- `build_lsp.sh`: release build with LSP.
- `make_docs.sh` / `make_docs_local.sh`: generate std library docs into `../sus-website/static/std` (`--gen-docs`).
- `clear_fixed_panics.sh <crash_dumps dir>`: re-runs every crash dump and deletes those that no longer crash.
- `coverage.sh` (cargo llvm-cov), `miri.sh`.

## Logging rules (from CONTRIBUTING.md)

`print!`/`println!` are banned: stdout is the LSP transport. Use the `log` macros (`trace`/`debug`/`info`/`warn`/`error`, re-exported from `prelude`). Inside `crate::debug::is_enabled(...)` blocks use `eprintln!`. Fatal errors use the `fatal_exit!` macro. `cargo fmt` + zero warnings are required.

## Standard library location

`std/*.sus` is **copied by `build.rs`** into `$XDG_DATA_HOME/sus/<version>/std` (override with `INSTALL_SUS_HOME` at build time, or `--sus-home` at runtime). The compiler reads std from there, not from the repo, so after editing `std/` a `cargo build` is needed for the change to take effect.

## Architecture

Everything hangs off `Linker` (`src/linker/mod.rs`): `files` (tree-sitter trees + text), `globals` (modules, types, constants, all addressed by typed `UUID`s from `src/alloc.rs`'s `FlatAlloc`/`ArenaAllocator`), and `instantiator`. `config()` is a process-wide singleton built from clap args in `src/config.rs`.

Compilation is staged, and **every global must finish a stage before any global enters the next**. Per-global state is checkpointed (`src/linker/checkpoint.rs`: `AFTER_INITIAL_PARSE_CP`, `AFTER_FLATTEN_CP`, `AFTER_TYPE_CHECK_CP`, `AFTER_LINTS_CP`) so the LSP can reset to a checkpoint and recompile. `--upto` exits after a chosen stage. The driver is `Linker::recompile_all` in `src/compiler_top.rs`.

1. **Initialization** (`src/flattening/initialization.rs`, `gather_initial_file_data`): runs when a file is added/updated. Walks the tree-sitter tree, creates `Module`/`StructType`/`NamedConstant` objects with names, ports, parameters. No bodies yet.
2. **Flattening** (`src/flattening/flatten.rs` + `parser.rs`): turns each body into a flat `Instruction` list stored in `LinkInfo::instructions`. Name resolution goes through `GlobalResolver` (`src/linker/passes.rs`), which records referenced globals.
3. **Abstract typecheck + lints** (`src/flattening/typecheck/`, `src/typing/abstract_*`, `domain_type.rs`): Hindley-Milner style unification over abstract types (type without concrete sizes, plus array rank as a Peano number) and over clock domains. Types are `UniCell`s (`src/typing/unifyable_cell.rs`). Template parameters are not resolved here; that is deliberate ("dependent types" are sidestepped by typechecking again after instantiation).
4. **Instantiation** (`src/instantiation/`): per `ConcreteGlobalReference` (module + concrete template args), cached in `Instantiator`. Steps in `instantiator.rs`:
   - `execute.rs`: runs the generative code (`gen` vars, `for`, `if`) over the instruction list, producing concrete wires/submodules. Submodules are instantiated recursively (with `--recursion-limit`).
   - `concrete_typecheck.rs`: unifies concrete types (`src/typing/concrete_type.rs`, `value_unifier.rs`), including template argument inference and integer bounds.
   - `src/latency/`: latency counting. `latency_algorithm.rs` solves the pipeline-register placement problem on the wire graph; `port_latency_inference.rs` infers port latencies for submodules. This is the core SUS feature; see `philosophy/latency.md`.
   - `clocks.rs`, `final_checks.rs`, `post_processing.rs`.
5. **Codegen** (`src/codegen/system_verilog.rs`): walks `InstantiatedModule`s reachable from the tops. `patches.rs` holds workarounds for simulator bugs documented in `KNOWN_SIMULATOR_BUGS.md`.

`--top` modules are handled by synthesizing a hidden `__top_modules` file containing `module _top { TopName _ }` per top, so tops can be given template args. With no `--top`, every parameter-less module is instantiated.

`src/dev_aid/` is tooling on top of the pipeline: ariadne error printing, the LSP (`lsp/`), dot-graph dumps, doc generation, port diagrams.

## Key types you will touch constantly

### `UniCell<T>` (`src/typing/unifyable_cell.rs`)

The type-variable cell used by every typechecker in the compiler. It wraps abstract types (`AbstractRankedType { inner: UniCell<AbstractInnerType>, rank: UniCell<PeanoType> }`), clock domains (`UniCell<ClockDomain>`, `UniCell<ClockID>`), and concrete template/array-size values (`UniCell<Value>` inside `ConcreteType`). Read the module-level doc comment in that file before changing anything that unifies.

- **Construction**: `UniCell::new(v)` / `UniCell::from(v)` make a *known* cell. `UniCell::UNKNOWN` makes a fresh, unique type variable. Building a type out of known parts therefore yields a known type; any `UNKNOWN` inside makes it a type variable to be solved.
- **Reading**: `cell.get() -> Option<&T>` returns `Some` only if the cell itself is `Known`. During unification a cell may be resolved through a substitution chain without being `Known`, so inside a typecheck pass use `unifier.resolve(cell)` / `resolve_all` rather than `get`. After the pass, `fully_substitute` collapses every cell to `Known` (or leaves it unknown, which is a type error to report). `unwrap()` panics if not known.
- **Unifying**: `unifier.unify(&a, &b)` makes two cells that already live in the program structure identical. `unifier.set(&cell, &mut owned)` unifies a structure cell with a freshly built owned `UniCell` (this is how you "assign" a type to something). Both return `UnifyResult::{Success, Failure, FailureInfiniteTypes}`; the caller reports the error, never the unifier. Nested types implement `UnifyRecurse` so unification descends into array element types, template args, etc.
- **Cloning**: plain `Clone` is only correct after typechecking. Before/during, use `clone_prototype` (fresh `UNKNOWN`s become distinct variables) or `unifier.clone_unify` (the clone stays unified with the original).
- **Delayed constraints**: `unifier.delayed_constraint(|u| ...)` registers a closure that runs once the cells it needs are resolved (e.g. "once this is an array type, unify its element type with X"). Call `execute_ready_constraints` before the unifier is dropped.
- **Lifecycle rule**: cells are created with `new`/`UNKNOWN`, then exactly one `Unifier<'s>` touches them, then they are fully substituted and the unifier is dropped. The code relies on this and does not enforce it.
- The two concrete unifiers are `AbstractUnifier` (`src/typing/abstract_unifier.rs`, stage 3) and `ValueUnifier` (`src/typing/value_unifier.rs`, used by `concrete_typecheck.rs` in stage 4). Unifier methods take `&self` on purpose; do not try to make them `&mut`.

### `Instruction` (`src/flattening/mod.rs`)

Flattening turns each module body into one flat `FlatAlloc<Instruction, FlatIDMarker>` in `LinkInfo::instructions`. Subexpressions are not nested; they refer to each other by `FlatID`, so later stages can point at any intermediate value. Execution walks the list in order. Variants: `Declaration`, `Expression`, `SubModule`, `Interface`, `IfStatement`, `ForStatement`. Related types you will see constantly: `ExpressionSource`, `ExpressionOutput` (`SubExpression` vs `MultiWrite`), `WireReference` + `WireReferenceRoot`, `WriteTo`, `DeclarationKind`, `GlobalReference`, `WrittenType`. Use the `Instruction::unwrap_*` accessors and the iterators in `src/flattening/walk.rs` rather than matching variants by hand.

### `RealWire`, `SubModule`, `InstantiatedModule` (`src/instantiation/mod.rs`)

After execution a module is an `InstantiatedModule`: `wires: FlatAlloc<RealWire, WireIDMarker>` and `submodules: FlatAlloc<SubModule, SubModuleIDMarker>`, plus `generation_state` mapping each `FlatID` to what it became (`SubModuleOrWire`). One `RealWire` is one actual hardware wire; `gen` values never become wires. `RealWire.source: RealWireDataSource` says how it is driven, and `original_instruction` links back to the `Instruction`. Concrete typecheck, latency counting, clocks, codegen and LSP hover all operate on these.

### IDs and allocation (`src/alloc.rs`, `src/prelude.rs`)

Nearly every collection is a `FlatAlloc<T, Marker>` indexed by a typed `UUID<Marker>` (`FlatID` for instructions, `WireID`, `SubModuleID`, `PortID`, `ModuleUUID`, ...). All markers are declared in `prelude.rs`. `let_unwrap!(Pattern(x), expr)` and `unwrap_variant!` are the idiomatic way to destructure enums you know the variant of.

## Debugging

- `--debug <name>` enables a named debug path; names are the string literals passed to `crate::debug::is_enabled(...)` (e.g. `print-abstract`, `print-concrete`, `print-concrete-pre-typecheck`, `print-execution-state`, `dot-latency-problem`, `dot-latency-solution`, `dot-dependency-graph`, `dot-svg`/`dot-png`, `lsp-debug`). `--debug-whitelist <substr>` restricts them to stages/globals whose debug-context label contains the substring (e.g. `"Typechecking FIFO"`).
- `__debug_span!`, `__debug_dbg!`, `__debug_breakpoint!` are available from the prelude; `--debug TEST` is the conventional ad-hoc flag to gate temporary prints.
- On panic, the compiler writes a **crash dump** (inputs + `reproduce.sh`) to `<sus-home>/crash_dumps/` (falls back to `./sus_crash_dumps/`). Pass `--no-redump` when reproducing a dump so you don't create another. `.vscode/launch.json` has lldb configs for `tinyTestFile.sus`, the three regression test files, and crash dumps.
- `--kill-timeout <secs>` watchdog-kills a stage that hangs; useful for infinite-loop bugs.

## Code style preferences (from the author)

- **No defensive programming.**/**Validate in one place, then assume validity.** Do not add `Result`s, early returns or `let _ =` to tolerate states that should never occur. A panic on a violated invariant is a useful ICE that points at the real bug; silently skipping hides it. Use `unwrap`, `assert!`, `let_unwrap!`, `unreachable!`.
- **Keep stages and abstraction levels separate.** Driver functions like `finish_instantiation` should read as a sequence of stage calls with an error gate between them. Put each stage's work in its own named function in the file where that work lives (`final_checks` / `final_lints` in `final_checks.rs`, `optimize` in `post_processing.rs`), and give functions that require a valid module an `assert!(!self.errors.did_error())` plus a doc comment stating the precondition.
- **The main structure of the codebase is already finished. Fit your changes within the existing architecture, rather than changing the architecture.** Ask explicitly if you wish to change the architecture. 