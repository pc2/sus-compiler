# 07-07-2025

## SerialLoop (Once every N cycles) (II = N inferred)
```sus
module SerialLoop#(T1, T2, ..., int D, int O1, int O2) {
    trigger loop'0: T1 v1'O1, T2 v2'O2

    action push'D: T1 next_v1'O1, T2 next v2'O2
}

/// use
SerialLoop loop

when loop.loop : int a, bool b {
    // ...

    when loop_end {

    } else {
        reg reg reg int heavy_computation = a + 3
        loop.push(heavy_computation, !b)
    }
} else {
    // Ready to receive data

    when input_data.pop() : int a {
        loop.push(a, false)
    }
}


```

## ConcurrentLoop (N datapoints in flight) (II = 1)
Same syntax as SerialLoop

## ReorderedLoop
Same as ConcurrentLoop, but keeps internal state index
```sus
module ReorderedLoop#(T1, T2, ..., int D, int O1, int O2, int MAX_ITERATIONS) {
    state int#(MIN: 0, MAX: D * MAX_ITERATIONS - 1) cur_idx
    next cur_idx = LatencyOffset #(OFFSET: -D)(cur_idx)

    state Tuple2#(T1, T2)[D * MAX_ITERATIONS] reorder_buffer

    trigger loop'0: T1 v1'O1, T2 v2'O2

    action push'D: T1 next_v1'O1, T2 next v2'O2

    action finish: T1 f1, T2 f2

    trigger result_valid: T1 r1, T2 r2
}
```
## Mandelbrot is an excellent example for SUS vs HLS


## Handle passing context by explicitly referencing parent module? 
module top {

    // Interface + absolute latencies must be fully specified
    // Similar issues to export submodule interface
    context action write #(T)'0: T data'0, int addr'0

    // write_to_mem is submodule somewhere of top
}

module write_to_mem {

    // There can only be at most ONE "top" module in its tree. Otherwise infinite module stack. 
    // Deals with 
    context top.write("MEEP", 10)

}
