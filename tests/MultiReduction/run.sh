cargo run --\
    --sus-home ../..\
    --top "MultiReduction#(T: type bool[32], NEUTRAL_ELEMENT: 32'h00000000, OPERATOR_LATENCY: 7, DATA_IN_LATENCY: 2, MAX_DATAS: 64)"\
    --gen-tb\
    -o codegen.sv\
    --no-optimization

vivado -mode batch -script sim.tcl
