cargo run fifo_tests.sus -o codegen.sv --top TestFIFO --top TestFWFT

vivado -mode batch -script sim.tcl
