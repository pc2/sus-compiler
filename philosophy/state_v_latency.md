# On Registers

## State vs Latency

In my experience, the use of registers usually boils down to two use cases: 
- Representing a current working state, which gets updated across clock cycles
- Improving timing closure by introducing registers on tight paths. 

While this distinction exists in the programmer's mind, it isn't in the vocabulary of common compilers. Verilog and VHDL just call both 'reg' (And non-registers too, but that's another can of worms.) 

Philosophically, the difference is quite important though. Registers that are part of the state are critical, and they directly direct the functioning of the device. While latency registers should not affect the functioning of the design at all, aside from trivially affecting the latency of the whole design. Some would argue that worrying about latency registers is a solved problem, with retiming tools that can automatically migrate latency registers across a design to place them wherever more timing slack is required. In practice though, this capability is limited, usually by explicitly marking specific paths as latency insensitive, or in a limited way by synthesizing a block of registers somewhere, which should then be migrated across the design. Still, this practice is always limited by the first design register it comes across along the path. Explicitly differentiating between state and latency registers could make this automatic retiming much more powerful. 

While indeed generally latency can't affect the actual operation of the device, it can be disallowed in certain circumstances. Certain paths are latency sensitive, and would no longer produce correct results if latency were introduced. A trivial example is any kind of feedback loop. In this case, no latency can be introduced within the feedback loop itself, as the result for the current feedback loop cycle wouldn't arrive in time. In this case the latency should either be forbidden, or reincorporated in a different way, such as interpreting the state loop as a [C-Slowed](https://en.wikipedia.org/wiki/C-slowing) state loop. 

## On State
See [state](state.md)

## On Latency
See [latency](latency.md)
