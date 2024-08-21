# To use, or not to use Tree Sitter
Should the SUS compiler use tree-sitter as its parser? Of all the parsers I've looked at, tree sitter is the most promising, combining both strong error tolerance with incredibly efficient parser generation. But switching may be a big task. 

## Arguments
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
- [People online say it's unsuitable for compiler frontend development. ](https://github.com/tree-sitter/tree-sitter/discussions/831)

## Verdict
Having gone through the effort of switching to tree-sitter, I can say I'm very content with the change. 

I'll now again go through the advantages and disadvantages I listed above, and show new perspectives. 

### Post-use For Tree Sitter
- Tree-sitter has proven an incredibly performant parser
- The incremental updates are still a big selling point for the future
- While I have found tree-sitter poorly documented, and difficult to debug, it has managed the things I needed it for
- Error recovery

### Post-use Against Tree Sitter
- The interface turned out far nicer than expected, once I built a [wrapper](https://github.com/pc2/sus-compiler/blob/5314928aaf9aa95ff4328be95bc4aed4f09d11b5/src/parser.rs#L81-L322) for it. Also some proc-macros to request the node kinds at compile time was a godsent.
- So ERROR node production is still a mystery in many cases, but with a decently unambiguous grammar where it fails to parse is pretty much always obvious
- Welp, I've adhered myself now, and I'm happy I did.
- I've placed a comment on this thread explaining my experience.
