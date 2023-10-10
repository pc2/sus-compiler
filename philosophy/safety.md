# Safety

So what does the Safety-First in Safety-First HDL mean? Like with our counterparts in Software Design such as Rust, it does not mean that the code you write is guaranteed to be correct. Rather it eliminates common classes of bugs that would otherwise have to be found through manual debugging. Counterintuitively however, is that the safety abstractions employed should never limit the programmer in the hardware they want to design. This means *any* hardware design one could possibly build in Verilog or VHDL, should also be representable in SUS. The difference should be that safe hardware should be easy to design, while unsafe should be comparatively difficult. Finally, as with Safe Software Languages, the goal is to enable fearless development and maintenance. The programmer should be able to rest easy that after implementing their change and fixing all compilation errors, the code again works properly. 

## Common classes of HW bugs are: 
### Cycle-wise timing errors through incorrectly pipelined HW. 
Manually keeping their pipeline in sync is taken out of the programmer's hands. The language makes a distinction between registers used for *latency* and those used for *state*. Latency registers are handled by latency counting and adding registers the other paths to keep them in sync. 

### Misunderstood module documentation leading to incorrect use. 
The system of Flow Descriptors is there to prevent incorrect use of library modules. Flow descriptors are not optional, so they force the programmer to add the proper descriptors when they define a module containing state. 

### Operation results being cast to a too small integer bitwidth. 
SUS disallows implicit casts that lose information. Instead, the programmer is required to specify either unsafe casts, where runtime checks can be inserted, or adding modular arithmetic to specify overflow behaviour.

### Data loss or duplication
Examples: 
- Data loss or state corruption for unready modules
- Data duplication from held state
- Data loss or duplication at Clock Domain Boundaries. 

Compiler warnings or errors on data that sits unused, ports that go unread, or ports that are written when no data is expected are all prevented by the flow descriptor system. Of course, it's not possible to prevent data from being lost within the module state itself. 

## Flow Descriptors
On any module or interface we can specify flow descriptors. These describe how and in which patterns data are allowed to flow through a module. Much like rust's borrow checker, this provides an additional layer of code flow analysis that must be verified for correctness. They are written in a kind of regex-like syntax, ideally with the full descriptive power of Linear Temporal Logic (LTL). Like with typing and borrow checking, the additional information describes the *what*, whereas the code describes the *how*. 

The exact notation of this is still in flux. A straight-forward option would be to straight up just use LTL notation, though I have some reservations about this. Certainly there's already a great body of work on LTL notation, making it an attractive choice, but the first big spanner in the works is that LTL allows itself to be recursively nested within arbitrary boolean expressions. Allowing this much freedom would require the compiler to effectively contain a SAT solver as part of this typechecking. Instead, perhaps only a subset of LTL could be used, which provides only simple regex-like pattern matching. 

## Error locality
While the primary objective of a safety-first compiler is to prevent compilation of code that has proven errors, just providing a binary yes-no at the end of compilation doesn't help the programmer much. Indeed errors should be descriptive and point to a relevant position in the code. Errors should in the best case also point the programmer towards a way to fix the error. 

But a type of locality that isn't often discussed is across template bounds. It's still a long way off for a fresh language, but it's important to think ahead a bit. 

There's broadly two types of error reporting with templates: Pre-Instantiation and Post-Instantiation errors. As the name implies, these are errors that can be located before, and after instantiation. 

There is a conflict between these error reporting types though. On the one had, Post-Instantiation errors are easy to implement. The compiler can deal with only known types and values. Any code generation code will have already run, giving the compiler a simple component network to work with. However, Post-Instantiation errors can only be reported after the module is instantiated of course. So after the user writes a templated module, they can only know it correct after actually using it somewhere with concrete parameters. Even then, the user will only see errors that occurred with this specific set of parameters, leaving them unsure their code is correct in the general case. 

This is why users strongly prefer Pre-Instantiation error reports. For these, the compiler only needs to look at the templated code itself to generate these errors. Famously, this is one of the biggest reason Rust programmers cite for preferring the language over C++. Rust with it's Trait system forces the user to apply the proper trait bounds to their template arguments to be able to use the abilities provided by the trait, allowing both error reporting within the function code, as well as reporting errors at instantiation time if the provided type doesn't implement the trait bounds. This is in stark contrast to C++'s approach, which doesn't even perform name resolution before template instantiation. 

Sadly, Pre-Instantiation error reporting comes with a lot of strings attached, or in many cases it may actually be impossible. Errors such as unused variables are impossible to detect in the general case with generative code for example, because ideally code generation should be turing-complete. For the same reason, errors in integer bounds also can't be caught in the general case. Perhaps typing errors could, by following Rust's approach of using Traits. 

In any case, Post-Instantiation errors are just easier to implement. For a working first version, it's probably for the best to leave out these nice programmer-friendly improvements. 
