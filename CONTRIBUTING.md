
# Contributing
The repository itself is simple to set up. Simply clone the repository and build it with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). 
There are no extra dependencies one needs for building the repository as-is. 

```sh
git clone https://github.com/pc2/sus-compiler.git
```

And build it:
```sh
cargo build
```

To edit the tree-sitter grammar and regenerate [tree-sitter-sus/src/parser.c](./tree-sitter_sus/src/parser.c), you will need [`tree-sitter-cli`](https://docs.rs/tree-sitter-cli/latest/tree_sitter_cli/). 

To avoid headaches due to mismatching versions, we use `tree-sitter-cli v0.24.7`
```sh
cargo install tree-sitter-cli@0.24.7
```

You can use the [tree-sitter-sus/tree.sh](tree-sitter-sus/tree.sh) script to regenerate the parser, and show some useful information about it. 

## Formatting and clippy
We require all contributions to be run through `cargo fmt` and that no warnings remain. 

### VSCode
Add the following to your `.vscode/settings.json`
```json
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "rust-analyzer.check.command": "clippy"
```

## Behavior-altering changes
To better defend against regressions during development, we require that any change that alters the _behavior_ of the compiler (IE, adding or changing diagnostics, or altering the test file [test.sus](test.sus)) also commits the changed effects. 
This can be done by simply re-running 
```sh
./test.sus_regression.sh
```
Afterwards, inspect the changes to [test.sus_output.txt](./test.sus_output.txt) and [test.sus_errors.txt](./test.sus_errors.txt) before committing. 
