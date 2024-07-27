# Proc-Macros for use in the SUS compiler. 

Right now this just contains three proc-macros. `kind!`, `kw!`, and `field!`. They are there to make compile-time queries to the [tree-sitter-sus](https://github.com/pc2/tree-sitter-sus) grammar. It's compile-time, requires no runtime queries, and they can be used in more contexts, like `match` arms. 

The grammar is generated using [tree-sitter](https://tree-sitter.github.io/tree-sitter/). Used as the parsing front-end of the [sus-compiler](https://crates.io/crates/sus-compiler). 

The SUS compiler repository is [here](https://github.com/pc2/sus-compiler). 

This package provides bindings for C, C++, and Rust. (And tree-sitter implemented a few more but who's counting. )

In the future I'll probably add macros for derive macros like migrating or collecting Spans
