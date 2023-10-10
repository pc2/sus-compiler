# The SUS Standard Library
By making cycle-latency mostly transparent to the programmer, we enable the use of more generic building blocks. These should be grouped into a standard library built specifically for the language. 

## Common interfaces
The STL should provide standardized names for common interface types, such as memory read and write ports and AXI ports. This helps standardise the interfaces of distinct libraries, allowing for easier integration. 

## Memory blocks and FIFOs
Configurable Memory primitives and FIFOs should certainly be part of the standard library. These are so fundamental to any hardware design, and appear to be uniquitous across FPGA vendors. These Memory primitives should however not be fully fixed. Attributes such as read latency and read-write conflict resolution vary substantially between vendors, so this should be left up to the target platform. However of course it should always be possible to properly fix these values in situations where the programmer needs them. Such as when one needs a 0-cycle memory read, even if that would mean it would reach terrible timing, or not synthesize at all on some platforms. 

This is also the reason why I believe the 'inference' doctrine of defining memory blocks is fundamentally flawed. An inference implementation will always make implicit assumptions about the read latency and read-write conflict, meaning the code isn't properly portable across devices. 

### Multi-Clock Memories and FIFOs
It is still up for debate whether multi-clock variants should be implicit from the use, or explicit different types. There are arguments to be made for both approaches. Certainly this gets blurry when making the distinction between synchronous and asynchronous clocks. In any case, multi-clock modules should be available in the STL in some form. 

## Shift registers, skid buffers, packers, unpackers
These are quite natural utilities that any project could use. 

## Clock Domain Crossings
Clock Domain Crossings are a famously difficult problem in hardware design, and are a source of many sporadic and difficult to find bugs. The way one does a clock domain crossing also very much depends on the circumstances. Thus again, no all-encompassing generic solution can really be given. However, various common CDC implementations should be offered in the STL, which can then prove connecting code safe using *rythms*. 

# Implementation of the STL
Generally, a generic implementation can be given for all STL types. But these won't work well on most platforms. For each platform, there should be platform-specific implementations of each of these. 

# STL extensions
In fields like HPC, certain interfaces are ubiquitous, such as DDR memory interfaces, HBM, PCIE and Ethernet. In general these don't fit in the standard library itself, as these features are not available on the majority of platforms, but instead could be offered as separate libraries. These could (like with the STL) provide differing implementations over a generic interface, to again enable more cross-platform code. 
