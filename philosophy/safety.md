# Safety

So what does the Safety-First in Safety-First HDL mean? Like with our counterparts in Software Design such as Rust, it does not mean that the code you write is guaranteed to be correct. Rather it eliminates common classes of bugs that would otherwise have to be found through manual debugging. Counterintuitively however, is that the safety abstractions employed should never limit the programmer in the hardware they want to design. This means *any* hardware design one could possibly build in Verilog or VHDL, should also be representable in SUS. The difference should be that safe hardware should be easy to design, while unsafe should be comparatively difficult. Finally, as with Safe Software Languages, the goal is to enable fearless development and maintenance. The programmer should be able to rest easy that after implementing their change and fixing all compilation errors, the code again works properly. 

## Common classes of HW bugs
### Cycle-wise timing errors through incorrectly pipelined HW
Manually keeping their pipeline in sync is taken out of the programmer's hands. The language makes a distinction between registers used for *latency* and those used for *state*. Latency registers are handled by latency counting and adding registers the other paths to keep them in sync. 

### Misunderstood module documentation leading to incorrect use
The system of Flow Descriptors is there to prevent incorrect use of library modules. Flow descriptors are not optional, so they force the programmer to add the proper descriptors when they define a module containing state. 

### Operation results being cast to a too small integer bitwidth
SUS disallows implicit casts that lose information. Instead, the programmer is required to specify either unsafe casts, where runtime checks can be inserted, or adding modular arithmetic to specify overflow behaviour.

### Data loss or duplication
Examples: 
- Data loss or state corruption for unready modules
- Data duplication from held state
- Data loss or duplication at Clock Domain Boundaries. 

Compiler warnings or errors on data that sits unused, ports that go unread, or ports that are written when no data is expected are all prevented by the flow descriptor system. Of course, it's not possible to prevent data from being lost within the module state itself. 

#### Safe Clock Domain Crossing through rhythms
Special timelines called 'rhythm' timelines are used to make things like safe synchronous clock domain crossings possible. 

let's take a 3-5 clock domain crossing:
```
Slow: !---------!---------!---------!---------!---------!---------!
Fast:  !-----!-----!-----!-----!-----!-----!-----!-----!-----!-----!
```
To properly define the crossings, no two clocks may land at the exact same time. We offset the fast clock by a small amount to do this. 
Data coming from the slow domain to the fast domain encounters no constraint on the sender. 
A full stream of data will result in the following rhythm:
`v v v v v v v v v v v v v v v ...` -> `v _ v _ v v _ v _ v v _ v _ v ...`

To send data from the fast to the slow clock, we must do the opposite. Our sender has to be careful to only send data when it will be picked up properly:
`_ v _ v v _ v _ v v _ v _ v ...` -> `v v v v v v v v v v v v v v v ...`

Of course, connecting a data stream to a clock domain crossing without the proper rhythm is an error. 

Rhythms can be generated through built-in opaque modules.
```
rhythmGenerator(clk*5, clk*3) : 
  left: () -> rhythm v / v / v
  right: () -> rhythm v v v
```

These either use compile-time information from the tool that implements the clocks, or it generates a module that tests the clock domain crossing for the proper rhythm at initialization time. 

Delayed rhythms follow a modular arithmetic. For example a rhythm between clocks with a ratio of `rhythm(clk*3,clk*5)`, will repeat every 5 clock cycles of the first clock, and 3 clock cycles of the second clock. `reg reg reg reg reg rhythm(clk*3,clk*5).left = rhythm(clk*3,clk*5).left`, `reg reg reg rhythm(clk*3,clk*5).right = rhythm(clk*3,clk*5).right`

## Flow Descriptors
On any module or interface we can specify flow descriptors. These describe how and in which patterns data are allowed to flow through a module. Much like rust's borrow checker, this provides an additional layer of code flow analysis that must be verified for correctness. They are written in a kind of regex-like syntax, ideally with the full descriptive power of Linear Temporal Logic (LTL). Like with typing and borrow checking, the additional information describes the *what*, whereas the code describes the *how*. 

The exact notation of this is still in flux. A straight-forward option would be to straight up just use LTL notation, though I have some reservations about this. Certainly there's already a great body of work on LTL notation, making it an attractive choice, but the first big spanner in the works is that LTL allows itself to be recursively nested within arbitrary boolean expressions. Allowing this much freedom would require the compiler to effectively contain a SAT solver as part of this typechecking. Instead, perhaps only a subset of LTL could be used, which provides only simple regex-like pattern matching. 

### Flow Endings
With the Flow Descriptors we define what happens while the module is working, but due to the nature of hardware design, the module is still being clocked even while it is not in operation. In fact, the flow descriptor does not even require that the module state advance every clock cycle. We can hold up the module in between valid parts, simply by adding clock enables to all registers. But, since the flow descriptor does assume we start at some given initial state, we must naturally also be certain that the previous packet has properly ended. 

The way we resolve this is by requiring the final state of the flow descriptor to be 'distinguishable'. Be it because the last statement in the flow descriptor has an extra boolean, or because the flow has a finite size. 

## Error locality
While the primary objective of a safety-first compiler is to prevent compilation of code that has proven errors, just providing a binary yes-no at the end of compilation doesn't help the programmer much. Indeed errors should be descriptive and point to a relevant position in the code. Errors should in the best case also point the programmer towards a way to fix the error. 

But a type of locality that isn't often discussed is across template bounds. It's still a long way off for a fresh language, but it's important to think ahead a bit. 

There's broadly two types of error reporting with templates: Pre-Instantiation and Post-Instantiation errors. As the name implies, these are errors that can be located before, and after instantiation. 

There is a conflict between these error reporting types though. On the one had, Post-Instantiation errors are easy to implement. The compiler can deal with only known types and values. Any code generation code will have already run, giving the compiler a simple component network to work with. However, Post-Instantiation errors can only be reported after the module is instantiated of course. So after the user writes a templated module, they can only know it correct after actually using it somewhere with concrete parameters. Even then, the user will only see errors that occurred with this specific set of parameters, leaving them unsure their code is correct in the general case. 

This is why users strongly prefer Pre-Instantiation error reports. For these, the compiler only needs to look at the templated code itself to generate these errors. Famously, this is one of the biggest reason Rust programmers cite for preferring the language over C++. Rust with it's Trait system forces the user to apply the proper trait bounds to their template arguments to be able to use the abilities provided by the trait, allowing both error reporting within the function code, as well as reporting errors at instantiation time if the provided type doesn't implement the trait bounds. This is in stark contrast to C++'s approach, which doesn't even perform name resolution before template instantiation. 

Sadly, Pre-Instantiation error reporting comes with a lot of strings attached, or in many cases it may actually be impossible. Errors such as unused variables are impossible to detect in the general case with generative code for example, because ideally code generation should be turing-complete. For the same reason, errors in integer bounds also can't be caught in the general case. Perhaps typing errors could, by following Rust's approach of using Traits. 

In any case, Post-Instantiation errors are just easier to implement. For a working first version, it's probably for the best to leave out these nice programmer-friendly improvements. 

## Bounds through Typing?
A core part of the compiler will be integer bounds. Instead of specifying integers by bitwidth, we specify the exact bounds they shall take. This both allows for much safer code (since we can reject for instance array accesses even for arrays not a power of two), but also allows us to automatically do the minor adder optimizations. Instead of the programmer being responsible for both ensuring integers are within certain bounds and writing proper bitwidths, we make the compiler responsible. 

The first instinct would be to include these integer bounds into the types themselves. This leads to a natural extention onto the typing of operators. Simply define the operators over how they affect the bounds. `a+b` with bounds `a:[0,N]` and `b:[0:M]` would yield the bound `[0:N+M]`. Sadly, while this approach works perfectly for feed-forward pipelines, once we get any kind of feedback this approach falls flat. Any feedback loop would have to trapdoor out of the bounds system with an explicit cast. As an example, `a = a + x` would fail to typecheck for any nonzero `x`. This would force us to explicitly add something like `a = a + x bound [0,N]` or something. 

Instead, we can integrate the bounds analysis into the state machine analysis pass. This wouldn't really be feasible in normal programming flows as the control flow can be quite complicated, but since we assume finite state machines, which we explicitly analyze over the flow descriptor, we can simply keep track of variable bounds along the way. This way, we can bound finite counters starting from known values automatically. Of course, if the user specifies a module with a potentially infinite flow descriptor, any recurrent state is suspect by definition, as it is likely a valid sequence of inputs if long enough could overflow whatever bound the user sets. In this case an explicit bound is still required, or a rework of the flow descriptor to no longer be infinite. 
