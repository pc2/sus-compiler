# Product Types
Product types or structs are quite natural to express in hardware and should be supported by the language. A product type is represented as a bundle of the field data lines that together form the whole struct. 

# Sum Types
Sum types, are the natural companion of Product Types. Though they are far less commonly supported in Software Languages, they have been gaining ground in recent years due to the popularity of the Rust Language. For software compilers, their implementation is quite natural. Since we only ever use one variant of a sum type at a time, we can share the memory that each of them would take up. This has many benefits: reduced struct size, possibility for shared field access optimization, and no real downsides. 

The same however, cannot be said for hardware design. In hardware design, there are two main ways one could implement a sum type. Either sharing the wires for the variants, or having each variant have their own separate set. Which implemenation is most efficient depends on how the sum type is used. In the case of sharing the wires, we incur the cost of the multiplexers to put the signals on the same wire, as well as the additional routing and timing cross dependencies it introduces between the variants. On the other hand, in the case of separating out the variants into their own wires does not incur this cost, but storing and moving the sum type around takes far more wires and registers. 

No natural implementation choice exists for Sum Types, and thus they shouldn't be supported at the language level. 

One exception however, is quite natural in hardware, and that is the Maybe (or Option) type. Sum types in general actually fit nicely with the flow descriptors system, where the developer can specify which level of wire sharing they want, and which ports should describe separate variants. 

Finally, there should be a type-safe implementation for a full wire-sharing sum type. That should be supported by the standard library, using something like a Union type, for those cases where the reduction in bus width is worth the additional multiplexers and routing constraints. 

# Enums
Enums are lovely. It's important that the programmer can specify what the exact representation is, such that the compiler can optimize their use. Be it as a one-hot vector, binary encoding, or a combination of the two. 
