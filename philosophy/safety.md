# Safety

So what does the Safety-First in Safety-First HDL mean? Like with our counterparts in Software Design such as Rust, it does not mean that the code you write is guaranteed to be correct. Rather it eliminates common classes of bugs that would otherwise have to be found through manual debugging. Counterintuitively however, is that the safety abstractions employed should never limit the programmer in the hardware they want to design. This means *any* hardware design one could possibly build in Verilog or VHDL, should also be representable in SUS. The difference should be that safe hardware should be easy to design, while unsafe should be comparatively difficult. Finally, as with Safe Software Languages, the goal is to enable fearless development and maintenance. The programmer should be able to rest easy that after implementing their change and fixing all compilation errors, the code again works properly. 

Common classes of HW bugs are: 
- Cycle-wise timing errors through incorrectly pipelined HW. 
- Misunderstood module documentation leading to incorrect use. 
- Operation results being cast to a too small integer bitwidth. 
- Data loss or state corruption for unready modules
- Data duplication from held state
- Data loss or duplication at Clock Domain Boundaries. 

The SUS compiler attempts to make these classes impossible through the following ways:
- Cycle-wise timing errors through incorrectly pipelined HW. 

Manually keeping their pipeline in sync is taken out of the programmer's hands. The language makes a distinction between registers used for *latency* and those used for *state*. Latency registers are handled by latency counting and adding registers the other paths to keep them in sync. 


## Flow Descriptors
On any module or interface we can specify flow descriptors. These describe how and in which patterns data are allowed to flow through a module. Much like rust's borrow checker, this provides an additional layer of code flow analysis that must be verified for correctness. They are written in a kind of regex-like syntax, ideally with the full descriptive power of Linear Temporal Logic (LTL). Like with typing and borrow checking, the additional information describes the *what*, whereas the code describes the *how*. 

The exact notation of this is still in flux. A straight-forward option would be to straight up just use LTL notation, though I have some reservations about this. Certainly there's already a great body of work on LTL notation, making it an attractive choice, but the first big spanner in the works is that LTL allows itself to be recursively nested within arbitrary boolean expressions. Allowing this much freedom would require the compiler to effectively contain a SAT solver as part of this typechecking. Instead, perhaps only a subset of LTL could be used, which provides only simple regex-like pattern matching. 


