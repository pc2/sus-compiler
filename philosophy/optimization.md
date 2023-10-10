# Optimization
One important observation I recently made was on Optimization. I had been quite proud of my unusual stance on "Optimization is a Non-Goal". But as I add more and more abstractions to the language, I come around to a different conclusion. Being a hardware designer in the lowest abstraction layer - Verilog, I believed that hardware itself was fundamentally unoptimizable when written out in this lowest abstraction layer, because any optimization the compiler could do could go against the intention of the programmer, undoing for example place-and-route considerations the programmer had made. As I introduced new abstractions however, I kept bumping into the problem of "what if I don't need this abstraction?". 

Making the abstractions always optional seemed to run counter to the safety promises I wanted to make, instead what would we ideal was if the programmer can specify their interface within the bounds of the abstraction, and somehow prove that the compiler will recognise the situation and remove the abstraction's overhead. In essense, this is what it means to do optimization. 

This more nuanced view is summarized as follows:

> Optimization should be a goal insofar as it is the un-doing of abstractions. 
> 
> Likewise, abstractions are only permissible if there is some all-encompassing optimization the compiler can perform to undo the abstraction if needed.

I still believe hardware is still broadly unoptimizeable. In contrast to software design, where the primary optimization target is Speed, in hardware there are multiple targets that are mutually at odds with each other. Clock speed, Cycle latency, logic, memory and DLS utilization, and routing congestion. No 'optimal' HW solution exists, and so it is up to the programmer to make this tradeoff. 

This is why I consider HLS to be misguided in their "Just write it as software code and we'll do the optimization for you" approach. 
