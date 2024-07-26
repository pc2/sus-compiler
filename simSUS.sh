cargo run -- --standalone BitSerialMatrixMultiply && iverilog -g2012 verilog_output/BitSerialMatrixMultiply_standalone.sv verilog_output/BitSerialMatrixMultiply_tb.sv && ./a.out && surfer test.vcd
