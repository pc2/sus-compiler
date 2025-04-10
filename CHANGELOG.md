# Changelog
## 0.3.0 (indev)
- Add `pow` and `pow2`
- Fix ICEs when improperly using constants
- 

### Technical changes
- Revamped SpanDebugger system. Now use inline string IDs for debug code paths (use --debug and --debug-whitelist only)

### Planned
- Sized integers
- float library

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
