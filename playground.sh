tree-sitter generate &&
tree-sitter build --wasm -o tree-sitter-sus.wasm &&
head -n 16 src/parser.c &&
tree-sitter playground --grammar-path .
