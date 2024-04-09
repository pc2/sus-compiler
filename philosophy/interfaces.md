# Interfaces

A module can have one or more interfaces, each of which consists of multiple input and output ports. 

Interfaces form the only method by which one can cross latency and clock domains. Each interface has with it its associated hardware, that operates within the same clock and latency counting domain. Wires which belong to the same latency counting group should be placed in the same interface. 

The code in one interface can read wires from other interfaces, provided the proper clock domain crossing method is used, in case of a clock domain crossing. Writes however, can naturally only be done by the interface that owns that wire. 

To transfer data from one interface to another, use the `cross` keyword. To ensure that multiple wires stay in sync when needed, you can cross multiple wires together: `cross wire_a, wire_b, wire_c`. This ensures that any relative latencies are maintained in the target interface. Latencies are not kept in sync for wires in separate `cross` statements. 

## Examples
Example implementation of `memory_block`:
```Verilog
module memory_block<gen int DEPTH, T> {
    interface write : T data, int addr, bool wr {
        T[DEPTH] memory

        if wr {
            memory[addr] = data
        }
    }
    interface read : int addr -> T data {
        cross memory

        data = memory[addr]
    }
}
```

Actually, we can implement `rebase_latency` from [latency.md](latency.md). 

Example implementation of `rebase_latency`:
```Verilog
// This module rebases the latency by an offset DELTA without adding registers. 
module rebase_latency<gen int DELTA, T> : T data_in'0 -> T data_out'DELTA {
    cross cross_o
    data_out = cross_o

    // Create an anonymous interface, such that we can break the latency dependency. 
    interface _ {
        cross data_in

        T cross_o = a
    }
}
```

