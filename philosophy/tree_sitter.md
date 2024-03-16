# To use, or not to use Tree Sitter
Should the SUS compiler use tree-sitter as its parser? Of all the parsers I've looked at, tree sitter is the most promising, combining both strong error tolerance with incredibly efficient parser generation. But switching may be a big task. 

### For custom parser
- Possibly easier to report errors, even though tree sitter is error tolerant, it can't produce diagnostics such as "Expected token such and such here"
- Already have a big parser built out
- Custom parser produces nicely typed syntax tree

### Against custom parser
- Probably I'm not skilled enough to build a proper recursive descent parser for more complex cases, like templates
- Still lots of work, also possibility of many panics in custom parser, tree sitter is quite reliable error wise

### For Tree Sitter
- Really efficient parser, using a direct state machine is something I could never beat myself. 
- Incremental updates. Speed of development is a big selling point for SUS
- Actually can parse the stuff I need to parse
- More reliable error recovery. 

### Against Tree Sitter
- Cumbersome interface, lots of Node::kind calls etc
- Don't know when tree sitter produces ERROR nodes and when not, when can I assume I can `unwrap()` stuff?
- Don't like deeply adhering myself to a parser library, because it makes future changes more difficult, like with Ariadne
- People online say it's unsuitable for compiler frontend development. 
