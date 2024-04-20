cd tree-sitter-sus
tree-sitter generate &&
tree-sitter parse ../tinyTestFile.sus &&
head -n 16 src/parser.c
cd ..

