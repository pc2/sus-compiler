# Changelog
## 0.1.0
- Standard library is shipped with the compiler now in the [stl/](stl/) directory. 
- Changed template definition and usage syntax to named arguments in `#()` instead of `::<>` (See [philosophy/template_troubles.md](philosophy/template_troubles.md))
- For Abstract Types and Domains, switch to Hindley-Milner type checking
- Fix #16: Incorrect operator prescedence for | and &
