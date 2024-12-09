// Extern module for test.sus:sized_int_add

module sized_int_add #(
	parameter int LEFT_SIZE,
	parameter int RIGHT_SIZE,
	parameter int OUTPUT_SIZE
) (
	input clk,
	input[LEFT_SIZE-1:0] a,
	input[RIGHT_SIZE-1:0] b,
    // c is output 1 cycle after a and b are provided
	output[OUTPUT_SIZE-1:0] c
);

reg[OUTPUT_SIZE-1:0] c_reg;

always_ff @(posedge clk) c_reg = a + b;
assign c = c_reg;

endmodule
