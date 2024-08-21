# The Trouble with Parsing Templates
Templates in the modern style of C++ or Java are incredibly hard to parse, and seemingly manage to conflict with just about every other syntax in common use. 
They can occur in many circumstances. Most commonly in types, but also in function calls and constants. 
Their ubiquity compounds the issues described below. 

```cpp
void myFunc() {
  vector<int> myVec;
}
```

All languages that choose to adopt this standard employ limitations around their use. In Java it's not possible to pass values as template arguments, in C++ often the [`template` keyword must be inserted where they're used for the parser to understand](https://stackoverflow.com/questions/610245/where-and-why-do-i-have-to-put-the-template-and-typename-keywords). 

Take the fully ambiguous case of the function call:
```
myFunc<beep>(3)
```

This can be parsed in two ways: 
- The way we as the programmer intend, IE it to be a template instantiation
- Two comparison operators with a value between parentheses: `myfunc<beep > (3)`

This is a proper grammatical ambiguity. We wish to avoid grammatical ambiguities. 

## Template troubles for SUS
In SUS there's quite a few things that come together that make this notation of templates difficult. For starters, SUS also has the `<` `>` comparison operators, which provides an immediate conflict, just like in the above example. 

Another less well known conflict from this notation comes from the commas. Take calling: 
```cpp
myFunc(myConst<int, bool>, x, y, z);
```

If the parser interprets the `<` as a comparison, then the commas separate function arguments, but if it were to interpret them as a template then the first comma separates template arguments. 

What's more, SUS has a few extra notations that also conflict with this idea, namely multiple value declarations, which are used for functions that return multiple values. 

To illustrate:
```cpp
int b;
int[5] c;
int a, b, c[0], myType<int, bool> d = myFuncReturningFourResults();
```

Where the intent is to assign to a newly declared `a`, an existing variable `b`, indexing into array `c`, and a new variable `d`. 

This notation combines declarations with assignable expressions. 
The issue is that if the compiler can accept both declarations and arbitrary expressions, then there's two perfectly valid parses for `d`. Either the one we intend, or it becomes two expressions `myType<int` and `bool> d`. 
While it's perhaps a bit dumb to assign to the output of a comparison operator to the parser it's all `_expression`. 

## Solutions
There's two solution paths I see: The Rust solution, and the Verilog solution. 
### Rust 
In so-called "type contexts", where the only this that's allowed to be written is a type, types are simple: `Option<i32>`, `Result<Vec<i32>, String>`, etc. 
Rust solves it with adding `::<` in cases where it would otherwise be a parsing ambiguity, like `my_func::<3>(5)`. This disambiguates it from the comparison operators. But here still, a comparison expression inside the arguments list breaks it again: `my_func::<3 > 1>`. 
Luckily, Rust sidesteps this by banning expressions in template all-together, as allowing that itself would also introduce a whole lot of dependent types mess that [turns pre-monomorphization into an undecidable problem](https://hackmd.io/OZG_XiLFRs2Xmw5s39jRzA?view). 

### Verilog
The Verilog solution is to simply move away from the angle bracket notation, and use a different one that doesn't conflict so heavily. In verilog's case, that's the `#(.varA(x), .varB(y), ...)` notation. 

Verilog does have some redundancy here though, with note the `defparam` syntax. It is wholly unnecessary. 

### SUS
Honestly, for languages where the vast majority of template instantiations do not depend on types, but rather on values, using an unambiguous syntax may just be the solution here. 
And yes, while Verilog's solution may not be familiar to software programmers, but it's the standard hardware programmers are used to. 

I will, however, make one change to it. Taking inspiration from Rust, and in accordance with Hardware programmers' desire for explicitness, I'll change the template instantiation syntax to use named arguments with short form syntax:
```sus
module FIFO<T, int SIZE, int LATENCY> {...}

module use_FIFO {
  gen int LATENCY = 3
  FIFO#(SIZE: 32, LATENCY, type T: int[3])
}
```
So in this example, `SIZE` is set to the result of the expression '32', `LATENCY` happens to be named identically to the variable we assign to it, thus short form. And the type we pass in requires a special `type` keyword so the parser can distinguish it. 

Critically, this syntax is aimed at the hardware designers, because templates in hardware design far more commonly involve values rather than types. And in many of those cases, types are easier to infer than values so the `type` fallback syntax should be a rare occurence. 
