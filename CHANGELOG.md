# Changelog
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
## 0.1.1
- Change sus stl installation directory to $HOME/.sus/VERSION/stl

## 0.2.0 (indev)
- Add Type Inference
- Add Generative Parameter Inference

### Technical Changes
- Hindley-Milner for Concrete Typing
- Submodule Instantiation done During Concrete Typing
- tree-sitter-sus has been merged into sus-compiler and is no longer a separate repository
