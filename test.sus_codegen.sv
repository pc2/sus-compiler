// testRemainder #()
module testRemainder(
	input clk
);

/*mux_wire*/ logic[3:0] a;
/*mux_wire*/ logic[1:0] b;
wire[1:0] _5;
assign _5 = a % b;
wire signed[4:0] _7;
assign _7 = -a;
wire signed[1:0] _9;
assign _9 = _7 % b;
wire signed[2:0] _12;
assign _12 = -b;
wire[1:0] _13;
assign _13 = a % _12;
wire signed[4:0] _15;
assign _15 = -a;
wire signed[2:0] _17;
assign _17 = -b;
wire signed[1:0] _18;
assign _18 = _15 % _17;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a = 4'dx;
	a = 4'd10;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	b = 2'dx;
	b = 2'd3;
end
endmodule

// testShifts #()
module testShifts(
	input clk
);

endmodule

// test_all_modulos #()
module test_all_modulos(
	input clk
);

/*mux_wire*/ logic[3:0] unsigned_val;
/*mux_wire*/ logic signed[5:0] signed_val;
/*mux_wire*/ logic[2:0] dynamic_mod;
wire[2:0] _3;
assign _3 = unsigned_val; // == mod 8 (target is 3 bits wide)
wire[1:0] _6;
assign _6 = unsigned_val; // == mod 4 (target is 2 bits wide)
wire[0:0] _9;
assign _9 = unsigned_val; // == mod 2 (target is 1 bits wide)
wire[2:0] _12;
assign _12 = signed_val; // == mod 8 (target is 3 bits wide)
wire[1:0] _15;
assign _15 = signed_val; // == mod 4 (target is 2 bits wide)
wire[0:0] _18;
assign _18 = signed_val; // == mod 2 (target is 1 bits wide)
// (zero sized) _21
// (zero sized) _24
wire[3:0] _27;
assign _27 = unsigned_val; // == mod 13
wire[3:0] _30;
assign _30 = unsigned_val + 1'd1;
wire[3:0] _32;
assign _32 = (_30 == 13) ? 0 : _30; // == mod 13
wire[3:0] _35;
assign _35 = unsigned_val + 2'd2;
wire[3:0] _37;
assign _37 = _35 - ((_35 >= 13) ? 13 : 0); // == mod 13
wire[4:0] _40;
assign _40 = unsigned_val + 4'd13;
wire[3:0] _42;
assign _42 = _40 - ((_40 >= 13) ? 13 : 0); // == mod 13
wire[4:0] _45;
assign _45 = unsigned_val + 4'd14;
wire[3:0] _47;
assign _47 = _45 % 13; // == mod 13
wire signed[4:0] _50;
assign _50 = unsigned_val - 1'd1;
wire[3:0] _52;
assign _52 = (_50 < 0) ? 12 : _50; // == mod 13
wire signed[4:0] _55;
assign _55 = unsigned_val - 2'd2;
wire[3:0] _57;
assign _57 = _55 + ((_55 < 0) ? 13 : 0); // == mod 13
wire signed[4:0] _60;
assign _60 = unsigned_val - 4'd13;
wire[3:0] _62;
assign _62 = _60 + ((_60 < 0) ? 13 : 0); // == mod 13
wire signed[4:0] _65;
assign _65 = unsigned_val - 4'd14;
wire[3:0] _67;
assign _67 = ((_65 % 13) + 13) % 13; // == mod 13
wire[2:0] _70;
assign _70 = ((unsigned_val % dynamic_mod) + dynamic_mod) % dynamic_mod; // == mod
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	unsigned_val = 4'dx;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	signed_val = 6'sdx;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	dynamic_mod = 3'dx;
end
endmodule

// zero_sized_stuffs #()
module zero_sized_stuffs(
	input clk
);

// (zero sized) x
// (zero sized) as_bits
// (zero sized) _UIntToBits_value
// (zero sized) _UIntToBits_bits
// (zero sized) zero_sized_arr
// (zero sized) _3
// (zero sized) make_real
// (zero sized) zero_sized_gen
/*mux_wire*/ logic[2:0] addr;
// (zero sized) a
// (zero sized) zero_sized_gen_2
// (zero sized) _6
// (zero sized) b
// (zero sized) _8
UIntToBits_NUM_BITS_0 UIntToBits(
	.clk(clk)
	// (zero sized port) .value(_UIntToBits_value)
	// (zero sized port) .bits(_UIntToBits_bits)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	addr = 3'dx;
	addr = 3'd4;
end
endmodule

// floats_and_doubles #()
module floats_and_doubles(
	input clk
);

/*mux_wire*/ logic[31:0] x;
/*mux_wire*/ logic[31:0] y;
/*mux_wire*/ logic[63:0] a;
/*mux_wire*/ logic[63:0] b;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x = 'x;
	x = 32'h3f000000 /* 0.5 */;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 'x;
	y = 32'h4dee6b28 /* 500000000.0 */;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a = 'x;
	a = 64'h3fe0000000000000 /* 0.5 */;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	b = 'x;
	b = 64'h41bdcd6500000000 /* 500000000.0 */;
end
endmodule

// boolean_array_literals #()
module boolean_array_literals(
	input clk
);

/*mux_wire*/ logic[49:0] b;
localparam[49:0] _1 = 50'b00000000000000000000000000000000000000010100100110;
/*mux_wire*/ logic[99:0] ob;
localparam[99:0] _2 = 100'b0000000000000000000000000000000000000000000000000000000000000000000011101100011100000111100110100111;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	b = 50'bxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx;
	b = _1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	ob = 100'bxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx;
	ob = _2;
end
endmodule

// test_vivado_bug #()
module test_vivado_bug(
	input clk,
	input wire b
);

/*mux_wire*/ logic[0:0] x;
/*mux_wire*/ logic _Repeat_v;
wire[0:0] _Repeat_result;
/*mux_wire*/ logic[0:0] y;
// (zero sized) u
// (zero sized) v
/*mux_wire*/ logic[1:0] x_2;
/*mux_wire*/ logic _Repeat_2_v;
wire[1:0] _Repeat_2_result;
/*mux_wire*/ logic[1:0] y_2;
/*mux_wire*/ logic[0:0] u_2;
/*mux_wire*/ logic[0:0] v_2;
/*mux_wire*/ logic[2:0] x_3;
/*mux_wire*/ logic _Repeat_3_v;
wire[2:0] _Repeat_3_result;
/*mux_wire*/ logic[2:0] y_3;
/*mux_wire*/ logic[1:0] u_3;
/*mux_wire*/ logic[1:0] v_3;
Repeat_T_type_bool_SIZE_1 Repeat(
	.clk(clk),
	.v(_Repeat_v),
	.result(_Repeat_result)
);
Repeat_T_type_bool_SIZE_2 Repeat_2(
	.clk(clk),
	.v(_Repeat_2_v),
	.result(_Repeat_2_result)
);
Repeat_T_type_bool_SIZE_3 Repeat_3(
	.clk(clk),
	.v(_Repeat_3_v),
	.result(_Repeat_3_result)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x = 1'bx;
	if(b) x = _Repeat_result;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	x = x;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_Repeat_v = 1'bx;
	if(b) _Repeat_v = 1'b0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_Repeat_v = _Repeat_v;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 1'bx;
	if(b) y = x;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	y = y;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x_2 = 2'bxx;
	if(b) x_2 = _Repeat_2_result;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_Repeat_2_v = 1'bx;
	if(b) _Repeat_2_v = 1'b0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_Repeat_2_v = _Repeat_2_v;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y_2 = 2'bxx;
	if(b) y_2 = x_2;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	u_2 = 1'dx;
	if(b) u_2 = 1'd0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	u_2 = u_2;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	v_2 = 1'dx;
	if(b) v_2 = u_2;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	v_2 = v_2;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x_3 = 3'bxxx;
	if(b) x_3 = _Repeat_3_result;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_Repeat_3_v = 1'bx;
	if(b) _Repeat_3_v = 1'b0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_Repeat_3_v = _Repeat_3_v;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y_3 = 3'bxxx;
	if(b) y_3 = x_3;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	u_3 = 2'dx;
	if(b) u_3 = 1'd0;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	v_3 = 2'dx;
	if(b) v_3 = u_3;
end
endmodule

// check_non_inlineds #()
module check_non_inlineds(
	input clk
);

/*mux_wire*/ logic[31:0] fs[4:0];
localparam[31:0] floats[4:0] = '{32'h3e99999a /* 0.3 */, 32'h3ecccccd /* 0.4 */, 32'h3e4ccccd /* 0.2 */, 32'h3e99999a /* 0.3 */, 32'h3f000000 /* 0.5 */};
/*mux_wire*/ logic[1:0] y;
/*mux_wire*/ logic[31:0] x;
localparam[31:0] floats_2[4:0] = '{32'h3e99999a /* 0.3 */, 32'h3ecccccd /* 0.4 */, 32'h3e4ccccd /* 0.2 */, 32'h3e99999a /* 0.3 */, 32'h3f000000 /* 0.5 */};
wire[31:0] _3 = floats_2[y];
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	fs = '{'x, 'x, 'x, 'x, 'x};
	for(int _v0 = 0; _v0 < 5; _v0 = _v0 + 1) begin
fs[_v0] = floats[_v0];
end
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 2'dx;
	y = 2'd3;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x = 'x;
	x = _3;
end
endmodule

// float_literal #()
module float_literal(
	input clk
);

/*mux_wire*/ logic[31:0] ff;
/*mux_wire*/ logic[31:0] fff;
/*mux_wire*/ logic[31:0] many_floats[4:0];
localparam[31:0] _2[4:0] = '{32'h3f333333 /* 0.7 */, 32'h3f666666 /* 0.9 */, 32'hbf333333 /* -0.7 */, 32'hbf666666 /* -0.9 */, 32'h3dcccccd /* 0.1 */};
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	ff = 'x;
	ff = 32'h4cbebc20 /* 100000000.0 */;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	fff = 'x;
	fff = 32'h3d800000 /* 0.0625 */;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	many_floats = '{'x, 'x, 'x, 'x, 'x};
	for(int _v0 = 0; _v0 < 5; _v0 = _v0 + 1) begin
many_floats[_v0] = _2[_v0];
end
end
endmodule

// multi_slice_reverse #()
module multi_slice_reverse(
	input clk,
	input wire[4:0] a,
	input wire[4:0] b,
	input wire[19:0] slice[8:0],
	input wire[1:0] slice2[26:0],
	input wire[4:0] gen_partselect[8:0],
	input wire[1:0] partselect[2:0],
	output /*mux_wire*/ logic[19:0] matrix[29:0]
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	matrix = '{20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx};
	for(int _v0 = 0; _v0 < 9; _v0 = _v0 + 1) begin
for(int _v1 = 0; _v1 < 20; _v1 = _v1 + 1) begin
matrix[_v0][_v1] = slice[_v0][_v1];
end
end
	for(int _v0 = 0; _v0 < 27; _v0 = _v0 + 1) begin
for(int _v1 = 0; _v1 < 2; _v1 = _v1 + 1) begin
matrix[3 + _v0][4 + _v1] = slice2[_v0][_v1];
end
end
	for(int _v0 = 0; _v0 < 9; _v0 = _v0 + 1) begin
for(int _v1 = 0; _v1 < 5; _v1 = _v1 + 1) begin
matrix[3 + _v0][7 + _v1] = gen_partselect[_v0][_v1];
end
end
	for(int _v0 = 0; _v0 < 3; _v0 = _v0 + 1) begin
for(int _v1 = 0; _v1 < 2; _v1 = _v1 + 1) begin
matrix[a + _v0][b - (1 - _v1)] = partselect[_v0][_v1];
end
end
end
endmodule

// multi_slice #()
module multi_slice(
	input clk,
	input wire[19:0] matrix[29:0],
	input wire[4:0] a,
	input wire[4:0] b,
	output /*mux_wire*/ logic[19:0] slice[8:0],
	output /*mux_wire*/ logic[1:0] slice2[26:0],
	output /*mux_wire*/ logic[4:0] gen_partselect[8:0],
	output /*mux_wire*/ logic[1:0] partselect[2:0]
);

genvar _g0;
genvar _g1;
wire[19:0] _1[8:0];
generate
for(_g0 = 0; _g0 < 9; _g0 = _g0 + 1) begin
for(_g1 = 0; _g1 < 20; _g1 = _g1 + 1) begin
assign _1[_g0][_g1] = matrix[_g0][_g1];
end
end
endgenerate
wire[1:0] _2[26:0];
generate
for(_g0 = 0; _g0 < 27; _g0 = _g0 + 1) begin
for(_g1 = 0; _g1 < 2; _g1 = _g1 + 1) begin
assign _2[_g0][_g1] = matrix[3 + _g0][4 + _g1];
end
end
endgenerate
wire[4:0] _3[8:0];
generate
for(_g0 = 0; _g0 < 9; _g0 = _g0 + 1) begin
for(_g1 = 0; _g1 < 5; _g1 = _g1 + 1) begin
assign _3[_g0][_g1] = matrix[3 + _g0][7 + _g1];
end
end
endgenerate
wire[1:0] _6[2:0];
generate
for(_g0 = 0; _g0 < 3; _g0 = _g0 + 1) begin
for(_g1 = 0; _g1 < 2; _g1 = _g1 + 1) begin
assign _6[_g0][_g1] = matrix[a + _g0][b - (1 - _g1)];
end
end
endgenerate
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	slice = '{20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx};
	for(int _v0 = 0; _v0 < 9; _v0 = _v0 + 1) begin
slice[_v0] = _1[_v0];
end
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	slice2 = '{2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx, 2'bxx};
	for(int _v0 = 0; _v0 < 27; _v0 = _v0 + 1) begin
slice2[_v0] = _2[_v0];
end
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	gen_partselect = '{5'bxxxxx, 5'bxxxxx, 5'bxxxxx, 5'bxxxxx, 5'bxxxxx, 5'bxxxxx, 5'bxxxxx, 5'bxxxxx, 5'bxxxxx};
	for(int _v0 = 0; _v0 < 9; _v0 = _v0 + 1) begin
gen_partselect[_v0] = _3[_v0];
end
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	partselect = '{2'bxx, 2'bxx, 2'bxx};
	for(int _v0 = 0; _v0 < 3; _v0 = _v0 + 1) begin
partselect[_v0] = _6[_v0];
end
end
endmodule

// use_use_trigger #()
module use_use_trigger(
	input clk,
	output /*mux_wire*/ logic pass_it_up
);

/*mux_wire*/ logic b;
/*mux_wire*/ logic _submod_maybe_use_trigger;
wire _submod_beep;
wire[2:0] _submod_boop;
/*mux_wire*/ logic[2:0] x;
/*mux_wire*/ logic[2:0] y;
use_trigger submod(
	.clk(clk),
	.beep(_submod_beep),
	.boop(_submod_boop),
	.maybe_use_trigger(_submod_maybe_use_trigger)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	b = 1'bx;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	b = b;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	pass_it_up = 1'bx;
	pass_it_up = 1'b0;
	if(_submod_beep) pass_it_up = 1'b1;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	pass_it_up = pass_it_up;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_submod_maybe_use_trigger = 1'bx;
	_submod_maybe_use_trigger = 1'b0;
	_submod_maybe_use_trigger = 1'b1;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_submod_maybe_use_trigger = _submod_maybe_use_trigger;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x = 3'dx;
	if(_submod_beep) x = _submod_boop;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 3'dx;
	if(_submod_beep) y = x;
end
endmodule

// use_trigger #()
module use_trigger(
	input clk,
	output /*mux_wire*/ logic beep,
	output /*mux_wire*/ logic[2:0] boop,
	input wire maybe_use_trigger
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	beep = 1'bx;
	beep = 1'b0;
	if(maybe_use_trigger) beep = 1'b1;
	if(!maybe_use_trigger) beep = 1'b1;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	beep = beep;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	boop = 3'dx;
	if(maybe_use_trigger) boop = 3'd5;
	if(!maybe_use_trigger) boop = 3'd7;
end
endmodule

// testInts #()
module testInts(
	input clk
);

/*mux_wire*/ logic[2:0] vs[4:0];
localparam[2:0] _1[4:0] = '{3'd1, 3'd2, 3'd3, 3'd4, 3'd5};
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	vs = '{3'dx, 3'dx, 3'dx, 3'dx, 3'dx};
	for(int _v0 = 0; _v0 < 5; _v0 = _v0 + 1) begin
vs[_v0] = _1[_v0];
end
end
endmodule

// use_infer_me_with_negative_delta #()
module use_infer_me_with_negative_delta(
	input clk,
	input wire x,
	output /*mux_wire*/ logic y
);

/*mux_wire*/ logic _inf_x;
wire _inf_y;
/*latency*/ logic __inf_y_N30; always_ff @(posedge clk) begin __inf_y_N30 <= _inf_y; end
/*latency*/ logic __inf_y_N29; always_ff @(posedge clk) begin __inf_y_N29 <= __inf_y_N30; end
/*latency*/ logic __inf_y_N28; always_ff @(posedge clk) begin __inf_y_N28 <= __inf_y_N29; end
/*latency*/ logic __inf_y_N27; always_ff @(posedge clk) begin __inf_y_N27 <= __inf_y_N28; end
/*latency*/ logic __inf_y_N26; always_ff @(posedge clk) begin __inf_y_N26 <= __inf_y_N27; end
/*latency*/ logic __inf_y_N25; always_ff @(posedge clk) begin __inf_y_N25 <= __inf_y_N26; end
/*latency*/ logic __inf_y_N24; always_ff @(posedge clk) begin __inf_y_N24 <= __inf_y_N25; end
/*latency*/ logic __inf_y_N23; always_ff @(posedge clk) begin __inf_y_N23 <= __inf_y_N24; end
/*latency*/ logic __inf_y_N22; always_ff @(posedge clk) begin __inf_y_N22 <= __inf_y_N23; end
/*latency*/ logic __inf_y_N21; always_ff @(posedge clk) begin __inf_y_N21 <= __inf_y_N22; end
/*latency*/ logic __inf_y_N20; always_ff @(posedge clk) begin __inf_y_N20 <= __inf_y_N21; end
/*latency*/ logic __inf_y_N19; always_ff @(posedge clk) begin __inf_y_N19 <= __inf_y_N20; end
/*latency*/ logic __inf_y_N18; always_ff @(posedge clk) begin __inf_y_N18 <= __inf_y_N19; end
/*latency*/ logic __inf_y_N17; always_ff @(posedge clk) begin __inf_y_N17 <= __inf_y_N18; end
/*latency*/ logic __inf_y_N16; always_ff @(posedge clk) begin __inf_y_N16 <= __inf_y_N17; end
/*latency*/ logic __inf_y_N15; always_ff @(posedge clk) begin __inf_y_N15 <= __inf_y_N16; end
/*latency*/ logic __inf_y_N14; always_ff @(posedge clk) begin __inf_y_N14 <= __inf_y_N15; end
/*latency*/ logic __inf_y_N13; always_ff @(posedge clk) begin __inf_y_N13 <= __inf_y_N14; end
/*latency*/ logic __inf_y_N12; always_ff @(posedge clk) begin __inf_y_N12 <= __inf_y_N13; end
/*latency*/ logic __inf_y_N11; always_ff @(posedge clk) begin __inf_y_N11 <= __inf_y_N12; end
/*latency*/ logic __inf_y_N10; always_ff @(posedge clk) begin __inf_y_N10 <= __inf_y_N11; end
/*latency*/ logic __inf_y_N9; always_ff @(posedge clk) begin __inf_y_N9 <= __inf_y_N10; end
/*latency*/ logic __inf_y_N8; always_ff @(posedge clk) begin __inf_y_N8 <= __inf_y_N9; end
/*latency*/ logic __inf_y_N7; always_ff @(posedge clk) begin __inf_y_N7 <= __inf_y_N8; end
/*latency*/ logic __inf_y_N6; always_ff @(posedge clk) begin __inf_y_N6 <= __inf_y_N7; end
/*latency*/ logic __inf_y_N5; always_ff @(posedge clk) begin __inf_y_N5 <= __inf_y_N6; end
/*latency*/ logic __inf_y_N4; always_ff @(posedge clk) begin __inf_y_N4 <= __inf_y_N5; end
/*latency*/ logic __inf_y_N3; always_ff @(posedge clk) begin __inf_y_N3 <= __inf_y_N4; end
/*latency*/ logic __inf_y_N2; always_ff @(posedge clk) begin __inf_y_N2 <= __inf_y_N3; end
/*latency*/ logic __inf_y_N1; always_ff @(posedge clk) begin __inf_y_N1 <= __inf_y_N2; end
/*mux_wire*/ logic _inf_p;
wire _inf_q;
infer_me_with_negative_delta_V_31 inf(
	.clk(clk),
	.x(_inf_x),
	.y(_inf_y),
	.p(_inf_p),
	.q(_inf_q)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 1'bx;
	y = __inf_y_N1;
	y = _inf_q;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	y = y;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_inf_x = 1'bx;
	_inf_x = x;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_inf_x = _inf_x;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_inf_p = 1'bx;
	_inf_p = x;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_inf_p = _inf_p;
end
endmodule

// infer_me_with_negative_delta #(V: 31)
module infer_me_with_negative_delta_V_31(
	input clk,
	input wire x,
	output /*mux_wire*/ logic y,
	input wire p,
	output /*mux_wire*/ logic q
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 1'bx;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	y = y;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	q = 1'bx;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	q = q;
end
endmodule

// use_infer_me_with_delta #()
module use_infer_me_with_delta(
	input clk,
	input wire x,
	output /*mux_wire*/ logic y
);

/*mux_wire*/ logic _inf_x;
wire _inf_y;
/*latency*/ logic __inf_y_N30; always_ff @(posedge clk) begin __inf_y_N30 <= _inf_y; end
/*latency*/ logic __inf_y_N29; always_ff @(posedge clk) begin __inf_y_N29 <= __inf_y_N30; end
/*latency*/ logic __inf_y_N28; always_ff @(posedge clk) begin __inf_y_N28 <= __inf_y_N29; end
/*latency*/ logic __inf_y_N27; always_ff @(posedge clk) begin __inf_y_N27 <= __inf_y_N28; end
/*latency*/ logic __inf_y_N26; always_ff @(posedge clk) begin __inf_y_N26 <= __inf_y_N27; end
/*latency*/ logic __inf_y_N25; always_ff @(posedge clk) begin __inf_y_N25 <= __inf_y_N26; end
/*latency*/ logic __inf_y_N24; always_ff @(posedge clk) begin __inf_y_N24 <= __inf_y_N25; end
/*latency*/ logic __inf_y_N23; always_ff @(posedge clk) begin __inf_y_N23 <= __inf_y_N24; end
/*latency*/ logic __inf_y_N22; always_ff @(posedge clk) begin __inf_y_N22 <= __inf_y_N23; end
/*latency*/ logic __inf_y_N21; always_ff @(posedge clk) begin __inf_y_N21 <= __inf_y_N22; end
/*latency*/ logic __inf_y_N20; always_ff @(posedge clk) begin __inf_y_N20 <= __inf_y_N21; end
/*latency*/ logic __inf_y_N19; always_ff @(posedge clk) begin __inf_y_N19 <= __inf_y_N20; end
/*latency*/ logic __inf_y_N18; always_ff @(posedge clk) begin __inf_y_N18 <= __inf_y_N19; end
/*latency*/ logic __inf_y_N17; always_ff @(posedge clk) begin __inf_y_N17 <= __inf_y_N18; end
/*latency*/ logic __inf_y_N16; always_ff @(posedge clk) begin __inf_y_N16 <= __inf_y_N17; end
/*latency*/ logic __inf_y_N15; always_ff @(posedge clk) begin __inf_y_N15 <= __inf_y_N16; end
/*latency*/ logic __inf_y_N14; always_ff @(posedge clk) begin __inf_y_N14 <= __inf_y_N15; end
/*latency*/ logic __inf_y_N13; always_ff @(posedge clk) begin __inf_y_N13 <= __inf_y_N14; end
/*latency*/ logic __inf_y_N12; always_ff @(posedge clk) begin __inf_y_N12 <= __inf_y_N13; end
/*latency*/ logic __inf_y_N11; always_ff @(posedge clk) begin __inf_y_N11 <= __inf_y_N12; end
/*latency*/ logic __inf_y_N10; always_ff @(posedge clk) begin __inf_y_N10 <= __inf_y_N11; end
/*latency*/ logic __inf_y_N9; always_ff @(posedge clk) begin __inf_y_N9 <= __inf_y_N10; end
/*latency*/ logic __inf_y_N8; always_ff @(posedge clk) begin __inf_y_N8 <= __inf_y_N9; end
/*latency*/ logic __inf_y_N7; always_ff @(posedge clk) begin __inf_y_N7 <= __inf_y_N8; end
/*latency*/ logic __inf_y_N6; always_ff @(posedge clk) begin __inf_y_N6 <= __inf_y_N7; end
/*latency*/ logic __inf_y_N5; always_ff @(posedge clk) begin __inf_y_N5 <= __inf_y_N6; end
/*latency*/ logic __inf_y_N4; always_ff @(posedge clk) begin __inf_y_N4 <= __inf_y_N5; end
/*latency*/ logic __inf_y_N3; always_ff @(posedge clk) begin __inf_y_N3 <= __inf_y_N4; end
/*latency*/ logic __inf_y_N2; always_ff @(posedge clk) begin __inf_y_N2 <= __inf_y_N3; end
/*latency*/ logic __inf_y_N1; always_ff @(posedge clk) begin __inf_y_N1 <= __inf_y_N2; end
/*mux_wire*/ logic _inf_p;
wire _inf_q;
infer_me_with_delta_V_31 inf(
	.clk(clk),
	.x(_inf_x),
	.y(_inf_y),
	.p(_inf_p),
	.q(_inf_q)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 1'bx;
	y = __inf_y_N1;
	y = _inf_q;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	y = y;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_inf_x = 1'bx;
	_inf_x = x;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_inf_x = _inf_x;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_inf_p = 1'bx;
	_inf_p = x;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_inf_p = _inf_p;
end
endmodule

// infer_me_with_delta #(V: -31)
module infer_me_with_delta_V_31(
	input clk,
	input wire x,
	output /*mux_wire*/ logic y,
	input wire p,
	output /*mux_wire*/ logic q
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 1'bx;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	y = y;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	q = 1'bx;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	q = q;
end
endmodule

// specified_latencies_not_ports_edge_case #()
module specified_latencies_not_ports_edge_case(
	input clk,
	input wire in_port,
	output /*mux_wire*/ logic out_port
);

/*latency*/ logic _in_port_D1; always_ff @(posedge clk) begin _in_port_D1 <= in_port; end
/*latency*/ logic _in_port_D2; always_ff @(posedge clk) begin _in_port_D2 <= _in_port_D1; end
/*latency*/ logic _in_port_D3; always_ff @(posedge clk) begin _in_port_D3 <= _in_port_D2; end
/*latency*/ logic _in_port_D4; always_ff @(posedge clk) begin _in_port_D4 <= _in_port_D3; end
/*latency*/ logic _in_port_D5; always_ff @(posedge clk) begin _in_port_D5 <= _in_port_D4; end
/*mux_wire*/ logic in_spec;
/*mux_wire*/ logic out_spec;
wire _4;
assign _4 = out_spec | _in_port_D5;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	out_port = 1'bx;
	out_port = _4;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	out_port = out_port;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	in_spec = 1'bx;
	in_spec = in_port;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	in_spec = in_spec;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	out_spec = 1'bx;
	out_spec = 1'b0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	out_spec = out_spec;
end
endmodule

// infer_from_local_context #()
module infer_from_local_context(
	input clk,
	input wire in_val,
	output /*mux_wire*/ logic out_val
);

/*latency*/ logic _in_val_D1; always_ff @(posedge clk) begin _in_val_D1 <= in_val; end
/*latency*/ logic _in_val_D2; always_ff @(posedge clk) begin _in_val_D2 <= _in_val_D1; end
/*latency*/ logic _in_val_D3; always_ff @(posedge clk) begin _in_val_D3 <= _in_val_D2; end
/*latency*/ logic _in_val_D4; always_ff @(posedge clk) begin _in_val_D4 <= _in_val_D3; end
/*latency*/ logic _in_val_D5; always_ff @(posedge clk) begin _in_val_D5 <= _in_val_D4; end
/*mux_wire*/ logic heavily_pipelined_computation;
/*mux_wire*/ logic _infer_me_x;
wire _infer_me_y;
wire _4;
assign _4 = _infer_me_y | heavily_pipelined_computation;
infer_me_A_5 infer_me(
	.clk(clk),
	.x(_infer_me_x),
	.y(_infer_me_y)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	out_val = 1'bx;
	out_val = _4;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	out_val = out_val;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	heavily_pipelined_computation = 1'bx;
	heavily_pipelined_computation = _in_val_D5;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	heavily_pipelined_computation = heavily_pipelined_computation;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_infer_me_x = 1'bx;
	_infer_me_x = in_val;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_infer_me_x = _infer_me_x;
end
endmodule

// infer_me #(A: 5)
module infer_me_A_5(
	input clk,
	input wire x,
	output /*mux_wire*/ logic y
);

/*latency*/ logic _x_D1; always_ff @(posedge clk) begin _x_D1 <= x; end
/*latency*/ logic _x_D2; always_ff @(posedge clk) begin _x_D2 <= _x_D1; end
/*latency*/ logic _x_D3; always_ff @(posedge clk) begin _x_D3 <= _x_D2; end
/*latency*/ logic _x_D4; always_ff @(posedge clk) begin _x_D4 <= _x_D3; end
/*latency*/ logic _x_D5; always_ff @(posedge clk) begin _x_D5 <= _x_D4; end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 1'bx;
	y = _x_D5;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	y = y;
end
endmodule

// infer_me #(A: 2)
module infer_me_A_2(
	input clk,
	input wire x,
	output /*mux_wire*/ logic y
);

/*latency*/ logic _x_D1; always_ff @(posedge clk) begin _x_D1 <= x; end
/*latency*/ logic _x_D2; always_ff @(posedge clk) begin _x_D2 <= _x_D1; end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 1'bx;
	y = _x_D2;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	y = y;
end
endmodule

// use_sized_int_add #()
module use_sized_int_add(
	input clk,
	input wire[3:0] a,
	input wire[2:0] b,
	output /*mux_wire*/ logic[4:0] c
);

/*mux_wire*/ logic[3:0] _sized_int_add_a;
/*mux_wire*/ logic[2:0] _sized_int_add_b;
wire[4:0] _sized_int_add_c;
sized_int_add #(.LEFT_SIZE(4), .RIGHT_SIZE(3), .OUTPUT_SIZE(5)) sized_int_add(
	.clk(clk),
	.a(_sized_int_add_a),
	.b(_sized_int_add_b),
	.c(_sized_int_add_c)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	c = 5'bxxxxx;
	c = _sized_int_add_c;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_sized_int_add_a = 4'bxxxx;
	_sized_int_add_a = a;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_sized_int_add_b = 3'bxxx;
	_sized_int_add_b = b;
end
endmodule

// sized_int_add #(LEFT_SIZE: 4, RIGHT_SIZE: 3, OUTPUT_SIZE: 5)
// Provided externally
// module sized_int_add_LEFT_SIZE_4_RIGHT_SIZE_3_OUTPUT_SIZE_5(
// 	input clk,
// 	input wire[3:0] a,
// 	input wire[2:0] b,
// 	output /*mux_wire*/ logic[4:0] c
// );
// numbersToAddUp #()
module numbersToAddUp(
	input clk
);

/*mux_wire*/ logic[1:0] arr[4:0];
/*mux_wire*/ logic[3:0] total;
/*mux_wire*/ logic[1:0] _adder_values[4:0];
wire[3:0] _adder_total;
TreeAdd_WIDTH_5_FROM_3_TO_4 adder(
	.clk(clk),
	.values(_adder_values),
	.total(_adder_total)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	arr = '{2'dx, 2'dx, 2'dx, 2'dx, 2'dx};
	arr[0] = 2'd3;
	arr[1] = 2'd3;
	arr[2] = 2'd3;
	arr[3] = 2'd3;
	arr[4] = 2'd3;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	total = 4'dx;
	total = _adder_total;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_adder_values = '{2'dx, 2'dx, 2'dx, 2'dx, 2'dx};
	for(int _v0 = 0; _v0 < 5; _v0 = _v0 + 1) begin
_adder_values[_v0] = arr[_v0];
end
end
endmodule

// no_main_interface #()
module no_main_interface(
	input clk
);

endmodule

// instruction_decoder #()
module instruction_decoder(
	input clk,
	input wire[31:0] instr
);

endmodule

// use_permute #()
module use_permute(
	input clk
);

/*mux_wire*/ logic[19:0] inArr[1:0];
/*mux_wire*/ logic[2:0] beep[7:0];
/*mux_wire*/ logic[2:0] _permut_d_in[7:0];
wire[2:0] _permut_d_out[7:0];
localparam[2:0] SOURCES[7:0] = '{3'd3, 3'd2, 3'd4, 3'd5, 3'd1, 3'd2, 3'd7, 3'd6};
permute_t_T_type_int_FROM_1_TO_8_SIZE_8_SOURCES_3_2_4_5_1_2_7_6 permut(
	.clk(clk),
	.d_in(_permut_d_in),
	.d_out(_permut_d_out)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	inArr = '{20'dx, 20'dx};
	inArr[0] = 12'd2387;
	inArr[1] = 20'd786823;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	beep = '{3'dx, 3'dx, 3'dx, 3'dx, 3'dx, 3'dx, 3'dx, 3'dx};
	for(int _v0 = 0; _v0 < 8; _v0 = _v0 + 1) begin
beep[_v0] = _permut_d_out[_v0];
end
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_permut_d_in = '{3'dx, 3'dx, 3'dx, 3'dx, 3'dx, 3'dx, 3'dx, 3'dx};
	for(int _v0 = 0; _v0 < 8; _v0 = _v0 + 1) begin
_permut_d_in[_v0] = SOURCES[_v0];
end
end
endmodule

// permute_t #(T: type int #(FROM: 1, TO: 8), SIZE: 8, SOURCES: [3, 2, 4, 5, 1, 2, 7, 6])
module permute_t_T_type_int_FROM_1_TO_8_SIZE_8_SOURCES_3_2_4_5_1_2_7_6(
	input clk,
	input wire[2:0] d_in[7:0],
	output /*mux_wire*/ logic[2:0] d_out[7:0]
);

wire[2:0] _1 = d_in[3];
wire[2:0] _2 = d_in[2];
wire[2:0] _3 = d_in[4];
wire[2:0] _4 = d_in[5];
wire[2:0] _5 = d_in[1];
wire[2:0] _6 = d_in[2];
wire[2:0] _7 = d_in[7];
wire[2:0] _8 = d_in[6];
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	d_out = '{3'dx, 3'dx, 3'dx, 3'dx, 3'dx, 3'dx, 3'dx, 3'dx};
	d_out[0] = _1;
	d_out[1] = _2;
	d_out[2] = _3;
	d_out[3] = _4;
	d_out[4] = _5;
	d_out[5] = _6;
	d_out[6] = _7;
	d_out[7] = _8;
end
endmodule

// replicate #(T: type int #(FROM: 3, TO: 4), NUM_REPLS: 30)
module replicate_T_type_int_FROM_3_TO_4_NUM_REPLS_30(
	input clk,
	input wire[1:0] data,
	output /*mux_wire*/ logic[1:0] result[29:0]
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	result = '{2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx, 2'dx};
	result[0] = data;
	result[1] = data;
	result[2] = data;
	result[3] = data;
	result[4] = data;
	result[5] = data;
	result[6] = data;
	result[7] = data;
	result[8] = data;
	result[9] = data;
	result[10] = data;
	result[11] = data;
	result[12] = data;
	result[13] = data;
	result[14] = data;
	result[15] = data;
	result[16] = data;
	result[17] = data;
	result[18] = data;
	result[19] = data;
	result[20] = data;
	result[21] = data;
	result[22] = data;
	result[23] = data;
	result[24] = data;
	result[25] = data;
	result[26] = data;
	result[27] = data;
	result[28] = data;
	result[29] = data;
end
endmodule

// testTinyTestMod #()
module testTinyTestMod(
	input clk
);

tinyTestMod_beep_3 a(
	.clk(clk),
	.o()
);
tinyTestMod_beep_4 b(
	.clk(clk),
	.o()
);
tinyTestMod_beep_3 c(
	.clk(clk),
	.o()
);
endmodule

// tinyTestMod #(beep: 4)
module tinyTestMod_beep_4(
	input clk,
	output /*mux_wire*/ logic[2:0] o
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 3'dx;
	o = 3'd4;
end
endmodule

// tinyTestMod #(beep: 3)
module tinyTestMod_beep_3(
	input clk,
	output /*mux_wire*/ logic[1:0] o
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 2'dx;
	o = 2'd3;
end
endmodule

// mod_with_unused_interface #()
module mod_with_unused_interface(
	input clk
);

endmodule

// no_port_module #()
module no_port_module(
	input clk
);

endmodule

// offset_backwards #()
module offset_backwards(
	input clk,
	input wire i,
	output /*mux_wire*/ logic o
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 1'bx;
	o = 1'b1;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	o = o;
end
endmodule

// cross_memory #()
module cross_memory(
	input clk,
	input wire[19:0] i[511:0],
	output /*mux_wire*/ logic[19:0] o[511:0]
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = '{20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx, 20'bxxxxxxxxxxxxxxxxxxxx};
	o[0][0] = 1'b1;
end
endmodule

// cross_int #()
module cross_int(
	input clk,
	input wire[6:0] i,
	output /*mux_wire*/ logic[0:0] o
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 1'dx;
	o = 1'd1;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	o = o;
end
endmodule

// cross_bool #()
module cross_bool(
	input clk,
	input wire i,
	output /*mux_wire*/ logic o
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 1'bx;
	o = 1'b1;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	o = o;
end
endmodule

// submodule_named_ports #()
module submodule_named_ports(
	input clk,
	input wire[6:0] port_a,
	input wire[6:0] port_b,
	output /*mux_wire*/ logic[7:0] port_c
);

wire[7:0] _3;
assign _3 = port_a + port_b;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	port_c = 8'dx;
	port_c = _3;
end
endmodule

// use_my_mod #()
module use_my_mod(
	input clk,
	output /*mux_wire*/ logic either
);

/*mux_wire*/ logic x;
/*mux_wire*/ logic y;
/*mux_wire*/ logic[6:0] _my_mod_i;
wire _my_mod_a;
wire _my_mod_b;
wire _4;
assign _4 = x | y;
my_mod my_mod(
	.clk(clk),
	.i(_my_mod_i),
	.a(_my_mod_a),
	.b(_my_mod_b)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	either = 1'bx;
	either = _4;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	either = either;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x = 1'bx;
	x = _my_mod_a;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	x = x;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 1'bx;
	y = _my_mod_b;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	y = y;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_my_mod_i = 7'dx;
	_my_mod_i = 2'd3;
end
endmodule

// my_mod #()
module my_mod(
	input clk,
	input wire[6:0] i,
	output /*mux_wire*/ logic a,
	output /*mux_wire*/ logic b
);

wire _3;
assign _3 = i == 2'd3;
wire _6;
assign _6 = i == 3'd5;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a = 1'bx;
	a = _3;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	a = a;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	b = 1'bx;
	b = _6;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	b = b;
end
endmodule

// monotonize_down #()
module monotonize_down(
	input clk,
	input wire[15:0] mbf,
	output /*mux_wire*/ logic[15:0] mtDown
);

/*mux_wire*/ logic[15:0] mbf2;
/*mux_wire*/ logic[15:0] mbf4;
/*mux_wire*/ logic[15:0] mbf8;
wire _1 = mbf[0];
wire _2 = mbf[1];
wire _3;
assign _3 = _1 | _2;
wire _4 = mbf[1];
wire _5 = mbf[2];
wire _6 = mbf[3];
wire _7;
assign _7 = _5 | _6;
wire _8 = mbf[3];
wire _9 = mbf[4];
wire _10 = mbf[5];
wire _11;
assign _11 = _9 | _10;
wire _12 = mbf[5];
wire _13 = mbf[6];
wire _14 = mbf[7];
wire _15;
assign _15 = _13 | _14;
wire _16 = mbf[7];
wire _17 = mbf[8];
wire _18 = mbf[9];
wire _19;
assign _19 = _17 | _18;
wire _20 = mbf[9];
wire _21 = mbf[10];
wire _22 = mbf[11];
wire _23;
assign _23 = _21 | _22;
wire _24 = mbf[11];
wire _25 = mbf[12];
wire _26 = mbf[13];
wire _27;
assign _27 = _25 | _26;
wire _28 = mbf[13];
wire _29 = mbf[14];
wire _30 = mbf[15];
wire _31;
assign _31 = _29 | _30;
wire _32 = mbf[15];
wire _33 = mbf2[0];
wire _34 = mbf2[2];
wire _35;
assign _35 = _33 | _34;
wire _36 = mbf2[1];
wire _37 = mbf2[3];
wire _38;
assign _38 = _36 | _37;
wire _39 = mbf2[2];
wire _40 = mbf2[3];
wire _41 = mbf2[4];
wire _42 = mbf2[6];
wire _43;
assign _43 = _41 | _42;
wire _44 = mbf2[5];
wire _45 = mbf2[7];
wire _46;
assign _46 = _44 | _45;
wire _47 = mbf2[6];
wire _48 = mbf2[7];
wire _49 = mbf2[8];
wire _50 = mbf2[10];
wire _51;
assign _51 = _49 | _50;
wire _52 = mbf2[9];
wire _53 = mbf2[11];
wire _54;
assign _54 = _52 | _53;
wire _55 = mbf2[10];
wire _56 = mbf2[11];
wire _57 = mbf2[12];
wire _58 = mbf2[14];
wire _59;
assign _59 = _57 | _58;
wire _60 = mbf2[13];
wire _61 = mbf2[15];
wire _62;
assign _62 = _60 | _61;
wire _63 = mbf2[14];
wire _64 = mbf2[15];
wire _65 = mbf4[0];
wire _66 = mbf4[4];
wire _67;
assign _67 = _65 | _66;
wire _68 = mbf4[1];
wire _69 = mbf4[5];
wire _70;
assign _70 = _68 | _69;
wire _71 = mbf4[2];
wire _72 = mbf4[6];
wire _73;
assign _73 = _71 | _72;
wire _74 = mbf4[3];
wire _75 = mbf4[7];
wire _76;
assign _76 = _74 | _75;
wire _77 = mbf4[4];
wire _78 = mbf4[5];
wire _79 = mbf4[6];
wire _80 = mbf4[7];
wire _81 = mbf4[8];
wire _82 = mbf4[12];
wire _83;
assign _83 = _81 | _82;
wire _84 = mbf4[9];
wire _85 = mbf4[13];
wire _86;
assign _86 = _84 | _85;
wire _87 = mbf4[10];
wire _88 = mbf4[14];
wire _89;
assign _89 = _87 | _88;
wire _90 = mbf4[11];
wire _91 = mbf4[15];
wire _92;
assign _92 = _90 | _91;
wire _93 = mbf4[12];
wire _94 = mbf4[13];
wire _95 = mbf4[14];
wire _96 = mbf4[15];
wire _97 = mbf8[0];
wire _98 = mbf8[8];
wire _99;
assign _99 = _97 | _98;
wire _100 = mbf8[1];
wire _101 = mbf8[9];
wire _102;
assign _102 = _100 | _101;
wire _103 = mbf8[2];
wire _104 = mbf8[10];
wire _105;
assign _105 = _103 | _104;
wire _106 = mbf8[3];
wire _107 = mbf8[11];
wire _108;
assign _108 = _106 | _107;
wire _109 = mbf8[4];
wire _110 = mbf8[12];
wire _111;
assign _111 = _109 | _110;
wire _112 = mbf8[5];
wire _113 = mbf8[13];
wire _114;
assign _114 = _112 | _113;
wire _115 = mbf8[6];
wire _116 = mbf8[14];
wire _117;
assign _117 = _115 | _116;
wire _118 = mbf8[7];
wire _119 = mbf8[15];
wire _120;
assign _120 = _118 | _119;
wire _121 = mbf8[8];
wire _122 = mbf8[9];
wire _123 = mbf8[10];
wire _124 = mbf8[11];
wire _125 = mbf8[12];
wire _126 = mbf8[13];
wire _127 = mbf8[14];
wire _128 = mbf8[15];
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	mtDown = 16'bxxxxxxxxxxxxxxxx;
	mtDown[0] = _99;
	mtDown[1] = _102;
	mtDown[2] = _105;
	mtDown[3] = _108;
	mtDown[4] = _111;
	mtDown[5] = _114;
	mtDown[6] = _117;
	mtDown[7] = _120;
	mtDown[8] = _121;
	mtDown[9] = _122;
	mtDown[10] = _123;
	mtDown[11] = _124;
	mtDown[12] = _125;
	mtDown[13] = _126;
	mtDown[14] = _127;
	mtDown[15] = _128;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	mbf2 = 16'bxxxxxxxxxxxxxxxx;
	mbf2[0] = _3;
	mbf2[1] = _4;
	mbf2[2] = _7;
	mbf2[3] = _8;
	mbf2[4] = _11;
	mbf2[5] = _12;
	mbf2[6] = _15;
	mbf2[7] = _16;
	mbf2[8] = _19;
	mbf2[9] = _20;
	mbf2[10] = _23;
	mbf2[11] = _24;
	mbf2[12] = _27;
	mbf2[13] = _28;
	mbf2[14] = _31;
	mbf2[15] = _32;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	mbf4 = 16'bxxxxxxxxxxxxxxxx;
	mbf4[0] = _35;
	mbf4[1] = _38;
	mbf4[2] = _39;
	mbf4[3] = _40;
	mbf4[4] = _43;
	mbf4[5] = _46;
	mbf4[6] = _47;
	mbf4[7] = _48;
	mbf4[8] = _51;
	mbf4[9] = _54;
	mbf4[10] = _55;
	mbf4[11] = _56;
	mbf4[12] = _59;
	mbf4[13] = _62;
	mbf4[14] = _63;
	mbf4[15] = _64;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	mbf8 = 16'bxxxxxxxxxxxxxxxx;
	mbf8[0] = _67;
	mbf8[1] = _70;
	mbf8[2] = _73;
	mbf8[3] = _76;
	mbf8[4] = _77;
	mbf8[5] = _78;
	mbf8[6] = _79;
	mbf8[7] = _80;
	mbf8[8] = _83;
	mbf8[9] = _86;
	mbf8[10] = _89;
	mbf8[11] = _92;
	mbf8[12] = _93;
	mbf8[13] = _94;
	mbf8[14] = _95;
	mbf8[15] = _96;
end
endmodule

// mbf_dual #()
module mbf_dual(
	input clk,
	input wire[127:0] mbf,
	output /*mux_wire*/ logic[127:0] dual
);

wire _1 = mbf[127];
wire _2;
assign _2 = !_1;
wire _3 = mbf[126];
wire _4;
assign _4 = !_3;
wire _5 = mbf[125];
wire _6;
assign _6 = !_5;
wire _7 = mbf[124];
wire _8;
assign _8 = !_7;
wire _9 = mbf[123];
wire _10;
assign _10 = !_9;
wire _11 = mbf[122];
wire _12;
assign _12 = !_11;
wire _13 = mbf[121];
wire _14;
assign _14 = !_13;
wire _15 = mbf[120];
wire _16;
assign _16 = !_15;
wire _17 = mbf[119];
wire _18;
assign _18 = !_17;
wire _19 = mbf[118];
wire _20;
assign _20 = !_19;
wire _21 = mbf[117];
wire _22;
assign _22 = !_21;
wire _23 = mbf[116];
wire _24;
assign _24 = !_23;
wire _25 = mbf[115];
wire _26;
assign _26 = !_25;
wire _27 = mbf[114];
wire _28;
assign _28 = !_27;
wire _29 = mbf[113];
wire _30;
assign _30 = !_29;
wire _31 = mbf[112];
wire _32;
assign _32 = !_31;
wire _33 = mbf[111];
wire _34;
assign _34 = !_33;
wire _35 = mbf[110];
wire _36;
assign _36 = !_35;
wire _37 = mbf[109];
wire _38;
assign _38 = !_37;
wire _39 = mbf[108];
wire _40;
assign _40 = !_39;
wire _41 = mbf[107];
wire _42;
assign _42 = !_41;
wire _43 = mbf[106];
wire _44;
assign _44 = !_43;
wire _45 = mbf[105];
wire _46;
assign _46 = !_45;
wire _47 = mbf[104];
wire _48;
assign _48 = !_47;
wire _49 = mbf[103];
wire _50;
assign _50 = !_49;
wire _51 = mbf[102];
wire _52;
assign _52 = !_51;
wire _53 = mbf[101];
wire _54;
assign _54 = !_53;
wire _55 = mbf[100];
wire _56;
assign _56 = !_55;
wire _57 = mbf[99];
wire _58;
assign _58 = !_57;
wire _59 = mbf[98];
wire _60;
assign _60 = !_59;
wire _61 = mbf[97];
wire _62;
assign _62 = !_61;
wire _63 = mbf[96];
wire _64;
assign _64 = !_63;
wire _65 = mbf[95];
wire _66;
assign _66 = !_65;
wire _67 = mbf[94];
wire _68;
assign _68 = !_67;
wire _69 = mbf[93];
wire _70;
assign _70 = !_69;
wire _71 = mbf[92];
wire _72;
assign _72 = !_71;
wire _73 = mbf[91];
wire _74;
assign _74 = !_73;
wire _75 = mbf[90];
wire _76;
assign _76 = !_75;
wire _77 = mbf[89];
wire _78;
assign _78 = !_77;
wire _79 = mbf[88];
wire _80;
assign _80 = !_79;
wire _81 = mbf[87];
wire _82;
assign _82 = !_81;
wire _83 = mbf[86];
wire _84;
assign _84 = !_83;
wire _85 = mbf[85];
wire _86;
assign _86 = !_85;
wire _87 = mbf[84];
wire _88;
assign _88 = !_87;
wire _89 = mbf[83];
wire _90;
assign _90 = !_89;
wire _91 = mbf[82];
wire _92;
assign _92 = !_91;
wire _93 = mbf[81];
wire _94;
assign _94 = !_93;
wire _95 = mbf[80];
wire _96;
assign _96 = !_95;
wire _97 = mbf[79];
wire _98;
assign _98 = !_97;
wire _99 = mbf[78];
wire _100;
assign _100 = !_99;
wire _101 = mbf[77];
wire _102;
assign _102 = !_101;
wire _103 = mbf[76];
wire _104;
assign _104 = !_103;
wire _105 = mbf[75];
wire _106;
assign _106 = !_105;
wire _107 = mbf[74];
wire _108;
assign _108 = !_107;
wire _109 = mbf[73];
wire _110;
assign _110 = !_109;
wire _111 = mbf[72];
wire _112;
assign _112 = !_111;
wire _113 = mbf[71];
wire _114;
assign _114 = !_113;
wire _115 = mbf[70];
wire _116;
assign _116 = !_115;
wire _117 = mbf[69];
wire _118;
assign _118 = !_117;
wire _119 = mbf[68];
wire _120;
assign _120 = !_119;
wire _121 = mbf[67];
wire _122;
assign _122 = !_121;
wire _123 = mbf[66];
wire _124;
assign _124 = !_123;
wire _125 = mbf[65];
wire _126;
assign _126 = !_125;
wire _127 = mbf[64];
wire _128;
assign _128 = !_127;
wire _129 = mbf[63];
wire _130;
assign _130 = !_129;
wire _131 = mbf[62];
wire _132;
assign _132 = !_131;
wire _133 = mbf[61];
wire _134;
assign _134 = !_133;
wire _135 = mbf[60];
wire _136;
assign _136 = !_135;
wire _137 = mbf[59];
wire _138;
assign _138 = !_137;
wire _139 = mbf[58];
wire _140;
assign _140 = !_139;
wire _141 = mbf[57];
wire _142;
assign _142 = !_141;
wire _143 = mbf[56];
wire _144;
assign _144 = !_143;
wire _145 = mbf[55];
wire _146;
assign _146 = !_145;
wire _147 = mbf[54];
wire _148;
assign _148 = !_147;
wire _149 = mbf[53];
wire _150;
assign _150 = !_149;
wire _151 = mbf[52];
wire _152;
assign _152 = !_151;
wire _153 = mbf[51];
wire _154;
assign _154 = !_153;
wire _155 = mbf[50];
wire _156;
assign _156 = !_155;
wire _157 = mbf[49];
wire _158;
assign _158 = !_157;
wire _159 = mbf[48];
wire _160;
assign _160 = !_159;
wire _161 = mbf[47];
wire _162;
assign _162 = !_161;
wire _163 = mbf[46];
wire _164;
assign _164 = !_163;
wire _165 = mbf[45];
wire _166;
assign _166 = !_165;
wire _167 = mbf[44];
wire _168;
assign _168 = !_167;
wire _169 = mbf[43];
wire _170;
assign _170 = !_169;
wire _171 = mbf[42];
wire _172;
assign _172 = !_171;
wire _173 = mbf[41];
wire _174;
assign _174 = !_173;
wire _175 = mbf[40];
wire _176;
assign _176 = !_175;
wire _177 = mbf[39];
wire _178;
assign _178 = !_177;
wire _179 = mbf[38];
wire _180;
assign _180 = !_179;
wire _181 = mbf[37];
wire _182;
assign _182 = !_181;
wire _183 = mbf[36];
wire _184;
assign _184 = !_183;
wire _185 = mbf[35];
wire _186;
assign _186 = !_185;
wire _187 = mbf[34];
wire _188;
assign _188 = !_187;
wire _189 = mbf[33];
wire _190;
assign _190 = !_189;
wire _191 = mbf[32];
wire _192;
assign _192 = !_191;
wire _193 = mbf[31];
wire _194;
assign _194 = !_193;
wire _195 = mbf[30];
wire _196;
assign _196 = !_195;
wire _197 = mbf[29];
wire _198;
assign _198 = !_197;
wire _199 = mbf[28];
wire _200;
assign _200 = !_199;
wire _201 = mbf[27];
wire _202;
assign _202 = !_201;
wire _203 = mbf[26];
wire _204;
assign _204 = !_203;
wire _205 = mbf[25];
wire _206;
assign _206 = !_205;
wire _207 = mbf[24];
wire _208;
assign _208 = !_207;
wire _209 = mbf[23];
wire _210;
assign _210 = !_209;
wire _211 = mbf[22];
wire _212;
assign _212 = !_211;
wire _213 = mbf[21];
wire _214;
assign _214 = !_213;
wire _215 = mbf[20];
wire _216;
assign _216 = !_215;
wire _217 = mbf[19];
wire _218;
assign _218 = !_217;
wire _219 = mbf[18];
wire _220;
assign _220 = !_219;
wire _221 = mbf[17];
wire _222;
assign _222 = !_221;
wire _223 = mbf[16];
wire _224;
assign _224 = !_223;
wire _225 = mbf[15];
wire _226;
assign _226 = !_225;
wire _227 = mbf[14];
wire _228;
assign _228 = !_227;
wire _229 = mbf[13];
wire _230;
assign _230 = !_229;
wire _231 = mbf[12];
wire _232;
assign _232 = !_231;
wire _233 = mbf[11];
wire _234;
assign _234 = !_233;
wire _235 = mbf[10];
wire _236;
assign _236 = !_235;
wire _237 = mbf[9];
wire _238;
assign _238 = !_237;
wire _239 = mbf[8];
wire _240;
assign _240 = !_239;
wire _241 = mbf[7];
wire _242;
assign _242 = !_241;
wire _243 = mbf[6];
wire _244;
assign _244 = !_243;
wire _245 = mbf[5];
wire _246;
assign _246 = !_245;
wire _247 = mbf[4];
wire _248;
assign _248 = !_247;
wire _249 = mbf[3];
wire _250;
assign _250 = !_249;
wire _251 = mbf[2];
wire _252;
assign _252 = !_251;
wire _253 = mbf[1];
wire _254;
assign _254 = !_253;
wire _255 = mbf[0];
wire _256;
assign _256 = !_255;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	dual = 128'bxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx;
	dual[0] = _2;
	dual[1] = _4;
	dual[2] = _6;
	dual[3] = _8;
	dual[4] = _10;
	dual[5] = _12;
	dual[6] = _14;
	dual[7] = _16;
	dual[8] = _18;
	dual[9] = _20;
	dual[10] = _22;
	dual[11] = _24;
	dual[12] = _26;
	dual[13] = _28;
	dual[14] = _30;
	dual[15] = _32;
	dual[16] = _34;
	dual[17] = _36;
	dual[18] = _38;
	dual[19] = _40;
	dual[20] = _42;
	dual[21] = _44;
	dual[22] = _46;
	dual[23] = _48;
	dual[24] = _50;
	dual[25] = _52;
	dual[26] = _54;
	dual[27] = _56;
	dual[28] = _58;
	dual[29] = _60;
	dual[30] = _62;
	dual[31] = _64;
	dual[32] = _66;
	dual[33] = _68;
	dual[34] = _70;
	dual[35] = _72;
	dual[36] = _74;
	dual[37] = _76;
	dual[38] = _78;
	dual[39] = _80;
	dual[40] = _82;
	dual[41] = _84;
	dual[42] = _86;
	dual[43] = _88;
	dual[44] = _90;
	dual[45] = _92;
	dual[46] = _94;
	dual[47] = _96;
	dual[48] = _98;
	dual[49] = _100;
	dual[50] = _102;
	dual[51] = _104;
	dual[52] = _106;
	dual[53] = _108;
	dual[54] = _110;
	dual[55] = _112;
	dual[56] = _114;
	dual[57] = _116;
	dual[58] = _118;
	dual[59] = _120;
	dual[60] = _122;
	dual[61] = _124;
	dual[62] = _126;
	dual[63] = _128;
	dual[64] = _130;
	dual[65] = _132;
	dual[66] = _134;
	dual[67] = _136;
	dual[68] = _138;
	dual[69] = _140;
	dual[70] = _142;
	dual[71] = _144;
	dual[72] = _146;
	dual[73] = _148;
	dual[74] = _150;
	dual[75] = _152;
	dual[76] = _154;
	dual[77] = _156;
	dual[78] = _158;
	dual[79] = _160;
	dual[80] = _162;
	dual[81] = _164;
	dual[82] = _166;
	dual[83] = _168;
	dual[84] = _170;
	dual[85] = _172;
	dual[86] = _174;
	dual[87] = _176;
	dual[88] = _178;
	dual[89] = _180;
	dual[90] = _182;
	dual[91] = _184;
	dual[92] = _186;
	dual[93] = _188;
	dual[94] = _190;
	dual[95] = _192;
	dual[96] = _194;
	dual[97] = _196;
	dual[98] = _198;
	dual[99] = _200;
	dual[100] = _202;
	dual[101] = _204;
	dual[102] = _206;
	dual[103] = _208;
	dual[104] = _210;
	dual[105] = _212;
	dual[106] = _214;
	dual[107] = _216;
	dual[108] = _218;
	dual[109] = _220;
	dual[110] = _222;
	dual[111] = _224;
	dual[112] = _226;
	dual[113] = _228;
	dual[114] = _230;
	dual[115] = _232;
	dual[116] = _234;
	dual[117] = _236;
	dual[118] = _238;
	dual[119] = _240;
	dual[120] = _242;
	dual[121] = _244;
	dual[122] = _246;
	dual[123] = _248;
	dual[124] = _250;
	dual[125] = _252;
	dual[126] = _254;
	dual[127] = _256;
end
endmodule

// fizz_buzz #()
module fizz_buzz(
	input clk,
	input wire[6:0] v,
	output /*mux_wire*/ logic[19:0] fb
);

/*mux_wire*/ logic fizz;
wire[1:0] _3;
assign _3 = v % 3; // == mod 3
wire _5;
assign _5 = _3 == 1'd0;
/*mux_wire*/ logic buzz;
wire[2:0] _8;
assign _8 = v % 5; // == mod 5
wire _10;
assign _10 = _8 == 1'd0;
wire _13;
assign _13 = fizz & buzz;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	fb = 20'dx;
	if(_13) fb = 20'd888555;
	if(!_13) if(fizz) fb = 10'd888;
	if(!_13) if(!fizz) if(buzz) fb = 10'd555;
	if(!_13) if(!fizz) if(!buzz) fb = v;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	fizz = 1'bx;
	fizz = _5;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	fizz = fizz;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	buzz = 1'bx;
	buzz = _10;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	buzz = buzz;
end
endmodule

// use_xor #()
module use_xor(
	input clk
);

/*mux_wire*/ logic b;
/*mux_wire*/ logic _xor_1_x1;
/*mux_wire*/ logic _xor_1_x2;
wire _xor_1_y;
xor xor_1(
	.clk(clk),
	.x1(_xor_1_x1),
	.x2(_xor_1_x2),
	.y(_xor_1_y)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	b = 1'bx;
	b = _xor_1_y;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	b = b;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_xor_1_x1 = 1'bx;
	_xor_1_x1 = 1'b1;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_xor_1_x1 = _xor_1_x1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_xor_1_x2 = 1'bx;
	_xor_1_x2 = 1'b0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_xor_1_x2 = _xor_1_x2;
end
endmodule

// xor #()
module xor(
	input clk,
	input wire x1,
	input wire x2,
	output /*mux_wire*/ logic y
);

/*mux_wire*/ logic w1;
wire _2;
assign _2 = !x1;
/*mux_wire*/ logic w2;
wire _4;
assign _4 = !x2;
/*mux_wire*/ logic w3;
wire _7;
assign _7 = x1 & w2;
/*mux_wire*/ logic w4;
wire _10;
assign _10 = x2 & w1;
wire _13;
assign _13 = w3 | w4;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 1'bx;
	y = _13;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	y = y;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	w1 = 1'bx;
	w1 = _2;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	w1 = w1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	w2 = 1'bx;
	w2 = _4;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	w2 = w2;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	w3 = 1'bx;
	w3 = _7;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	w3 = w3;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	w4 = 1'bx;
	w4 = _10;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	w4 = w4;
end
endmodule

// doNothing #()
module doNothing(
	input clk
);

endmodule

// submodule #()
module submodule(
	input clk,
	input wire[6:0] a,
	input wire[6:0] b,
	output /*mux_wire*/ logic[13:0] r
);

wire[13:0] _3;
assign _3 = a * b;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	r = 14'dx;
	r = _3;
end
endmodule

// multiple_outputs_only #()
module multiple_outputs_only(
	input clk,
	output /*mux_wire*/ logic o,
	output /*mux_wire*/ logic o2
);

/*state*/ logic loop = 1'b0;
/*latency*/ logic _loop_N1; always_ff @(posedge clk) begin _loop_N1 <= loop; end
/*latency*/ logic _loop_D0; always_ff @(posedge clk) begin _loop_D0 <= _loop_N1; end
wire _2;
assign _2 = !loop;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 1'bx;
	o = _loop_D0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	o = o;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o2 = 1'bx;
	o2 = _loop_D0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	o2 = o2;
end
always_ff @(posedge clk) begin
	loop <= _2;
end
endmodule

// output_only #()
module output_only(
	input clk,
	output /*mux_wire*/ logic o
);

/*state*/ logic loop = 1'b0;
/*latency*/ logic _loop_D0; always_ff @(posedge clk) begin _loop_D0 <= loop; end
wire _2;
assign _2 = !loop;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 1'bx;
	o = _loop_D0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	o = o;
end
always_ff @(posedge clk) begin
	loop <= _2;
end
endmodule

// multiple_inputs_only #()
module multiple_inputs_only(
	input clk,
	input wire i,
	input wire i2
);

/*state*/ logic loop = 1'b0;
wire _3;
assign _3 = loop ^ i;
wire _5;
assign _5 = _3 ^ i2;
always_ff @(posedge clk) begin
	loop <= _5;
end
endmodule

// input_only #()
module input_only(
	input clk,
	input wire i
);

/*state*/ logic loop = 1'b0;
wire _3;
assign _3 = loop ^ i;
always_ff @(posedge clk) begin
	loop <= _3;
end
endmodule

// good_cycle #()
module good_cycle(
	input clk,
	input wire a,
	output /*mux_wire*/ logic r
);

/*state*/ logic test = 1'b0;
/*mux_wire*/ logic new_test;
wire _3;
assign _3 = test ^ a;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	r = 1'bx;
	r = new_test;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	r = r;
end
always_ff @(posedge clk) begin
	test <= new_test;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	new_test = 1'bx;
	new_test = _3;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	new_test = new_test;
end
endmodule

// module_taking_a_lot_of_time #()
module module_taking_a_lot_of_time(
	input clk,
	input wire[6:0] data_in,
	output /*mux_wire*/ logic[6:0] data_out
);

/*latency*/ logic[6:0] _data_in_D1; always_ff @(posedge clk) begin _data_in_D1 <= data_in; end
/*latency*/ logic[6:0] _data_in_D2; always_ff @(posedge clk) begin _data_in_D2 <= _data_in_D1; end
/*latency*/ logic[6:0] _data_in_D3; always_ff @(posedge clk) begin _data_in_D3 <= _data_in_D2; end
/*latency*/ logic[6:0] _data_in_D4; always_ff @(posedge clk) begin _data_in_D4 <= _data_in_D3; end
/*latency*/ logic[6:0] _data_in_D5; always_ff @(posedge clk) begin _data_in_D5 <= _data_in_D4; end
/*latency*/ logic[6:0] _data_in_D6; always_ff @(posedge clk) begin _data_in_D6 <= _data_in_D5; end
/*latency*/ logic[6:0] _data_in_D7; always_ff @(posedge clk) begin _data_in_D7 <= _data_in_D6; end
/*latency*/ logic[6:0] _data_in_D8; always_ff @(posedge clk) begin _data_in_D8 <= _data_in_D7; end
/*latency*/ logic[6:0] _data_in_D9; always_ff @(posedge clk) begin _data_in_D9 <= _data_in_D8; end
/*latency*/ logic[6:0] _data_in_D10; always_ff @(posedge clk) begin _data_in_D10 <= _data_in_D9; end
/*latency*/ logic[6:0] _data_in_D11; always_ff @(posedge clk) begin _data_in_D11 <= _data_in_D10; end
/*latency*/ logic[6:0] _data_in_D12; always_ff @(posedge clk) begin _data_in_D12 <= _data_in_D11; end
/*latency*/ logic[6:0] _data_in_D13; always_ff @(posedge clk) begin _data_in_D13 <= _data_in_D12; end
/*latency*/ logic[6:0] _data_in_D14; always_ff @(posedge clk) begin _data_in_D14 <= _data_in_D13; end
/*latency*/ logic[6:0] _data_in_D15; always_ff @(posedge clk) begin _data_in_D15 <= _data_in_D14; end
/*latency*/ logic[6:0] _data_in_D16; always_ff @(posedge clk) begin _data_in_D16 <= _data_in_D15; end
/*latency*/ logic[6:0] _data_in_D17; always_ff @(posedge clk) begin _data_in_D17 <= _data_in_D16; end
/*latency*/ logic[6:0] _data_in_D18; always_ff @(posedge clk) begin _data_in_D18 <= _data_in_D17; end
/*latency*/ logic[6:0] _data_in_D19; always_ff @(posedge clk) begin _data_in_D19 <= _data_in_D18; end
/*latency*/ logic[6:0] _data_in_D20; always_ff @(posedge clk) begin _data_in_D20 <= _data_in_D19; end
/*latency*/ logic[6:0] _data_in_D21; always_ff @(posedge clk) begin _data_in_D21 <= _data_in_D20; end
/*latency*/ logic[6:0] _data_in_D22; always_ff @(posedge clk) begin _data_in_D22 <= _data_in_D21; end
/*latency*/ logic[6:0] _data_in_D23; always_ff @(posedge clk) begin _data_in_D23 <= _data_in_D22; end
/*latency*/ logic[6:0] _data_in_D24; always_ff @(posedge clk) begin _data_in_D24 <= _data_in_D23; end
/*latency*/ logic[6:0] _data_in_D25; always_ff @(posedge clk) begin _data_in_D25 <= _data_in_D24; end
/*latency*/ logic[6:0] _data_in_D26; always_ff @(posedge clk) begin _data_in_D26 <= _data_in_D25; end
/*latency*/ logic[6:0] _data_in_D27; always_ff @(posedge clk) begin _data_in_D27 <= _data_in_D26; end
/*latency*/ logic[6:0] _data_in_D28; always_ff @(posedge clk) begin _data_in_D28 <= _data_in_D27; end
/*latency*/ logic[6:0] _data_in_D29; always_ff @(posedge clk) begin _data_in_D29 <= _data_in_D28; end
/*latency*/ logic[6:0] _data_in_D30; always_ff @(posedge clk) begin _data_in_D30 <= _data_in_D29; end
/*latency*/ logic[6:0] _data_in_D31; always_ff @(posedge clk) begin _data_in_D31 <= _data_in_D30; end
/*latency*/ logic[6:0] _data_in_D32; always_ff @(posedge clk) begin _data_in_D32 <= _data_in_D31; end
/*latency*/ logic[6:0] _data_in_D33; always_ff @(posedge clk) begin _data_in_D33 <= _data_in_D32; end
/*latency*/ logic[6:0] _data_in_D34; always_ff @(posedge clk) begin _data_in_D34 <= _data_in_D33; end
/*latency*/ logic[6:0] _data_in_D35; always_ff @(posedge clk) begin _data_in_D35 <= _data_in_D34; end
/*latency*/ logic[6:0] _data_in_D36; always_ff @(posedge clk) begin _data_in_D36 <= _data_in_D35; end
/*latency*/ logic[6:0] _data_in_D37; always_ff @(posedge clk) begin _data_in_D37 <= _data_in_D36; end
/*latency*/ logic[6:0] _data_in_D38; always_ff @(posedge clk) begin _data_in_D38 <= _data_in_D37; end
/*latency*/ logic[6:0] _data_in_D39; always_ff @(posedge clk) begin _data_in_D39 <= _data_in_D38; end
/*latency*/ logic[6:0] _data_in_D40; always_ff @(posedge clk) begin _data_in_D40 <= _data_in_D39; end
/*latency*/ logic[6:0] _data_in_D41; always_ff @(posedge clk) begin _data_in_D41 <= _data_in_D40; end
/*latency*/ logic[6:0] _data_in_D42; always_ff @(posedge clk) begin _data_in_D42 <= _data_in_D41; end
/*latency*/ logic[6:0] _data_in_D43; always_ff @(posedge clk) begin _data_in_D43 <= _data_in_D42; end
/*latency*/ logic[6:0] _data_in_D44; always_ff @(posedge clk) begin _data_in_D44 <= _data_in_D43; end
/*latency*/ logic[6:0] _data_in_D45; always_ff @(posedge clk) begin _data_in_D45 <= _data_in_D44; end
/*latency*/ logic[6:0] _data_in_D46; always_ff @(posedge clk) begin _data_in_D46 <= _data_in_D45; end
/*latency*/ logic[6:0] _data_in_D47; always_ff @(posedge clk) begin _data_in_D47 <= _data_in_D46; end
/*latency*/ logic[6:0] _data_in_D48; always_ff @(posedge clk) begin _data_in_D48 <= _data_in_D47; end
/*latency*/ logic[6:0] _data_in_D49; always_ff @(posedge clk) begin _data_in_D49 <= _data_in_D48; end
/*latency*/ logic[6:0] _data_in_D50; always_ff @(posedge clk) begin _data_in_D50 <= _data_in_D49; end
/*latency*/ logic[6:0] _data_in_D51; always_ff @(posedge clk) begin _data_in_D51 <= _data_in_D50; end
/*latency*/ logic[6:0] _data_in_D52; always_ff @(posedge clk) begin _data_in_D52 <= _data_in_D51; end
/*latency*/ logic[6:0] _data_in_D53; always_ff @(posedge clk) begin _data_in_D53 <= _data_in_D52; end
/*latency*/ logic[6:0] _data_in_D54; always_ff @(posedge clk) begin _data_in_D54 <= _data_in_D53; end
/*latency*/ logic[6:0] _data_in_D55; always_ff @(posedge clk) begin _data_in_D55 <= _data_in_D54; end
/*latency*/ logic[6:0] _data_in_D56; always_ff @(posedge clk) begin _data_in_D56 <= _data_in_D55; end
/*latency*/ logic[6:0] _data_in_D57; always_ff @(posedge clk) begin _data_in_D57 <= _data_in_D56; end
/*latency*/ logic[6:0] _data_in_D58; always_ff @(posedge clk) begin _data_in_D58 <= _data_in_D57; end
/*latency*/ logic[6:0] _data_in_D59; always_ff @(posedge clk) begin _data_in_D59 <= _data_in_D58; end
/*latency*/ logic[6:0] _data_in_D60; always_ff @(posedge clk) begin _data_in_D60 <= _data_in_D59; end
/*latency*/ logic[6:0] _data_in_D61; always_ff @(posedge clk) begin _data_in_D61 <= _data_in_D60; end
/*latency*/ logic[6:0] _data_in_D62; always_ff @(posedge clk) begin _data_in_D62 <= _data_in_D61; end
/*latency*/ logic[6:0] _data_in_D63; always_ff @(posedge clk) begin _data_in_D63 <= _data_in_D62; end
/*latency*/ logic[6:0] _data_in_D64; always_ff @(posedge clk) begin _data_in_D64 <= _data_in_D63; end
/*latency*/ logic[6:0] _data_in_D65; always_ff @(posedge clk) begin _data_in_D65 <= _data_in_D64; end
/*latency*/ logic[6:0] _data_in_D66; always_ff @(posedge clk) begin _data_in_D66 <= _data_in_D65; end
/*latency*/ logic[6:0] _data_in_D67; always_ff @(posedge clk) begin _data_in_D67 <= _data_in_D66; end
/*latency*/ logic[6:0] _data_in_D68; always_ff @(posedge clk) begin _data_in_D68 <= _data_in_D67; end
/*latency*/ logic[6:0] _data_in_D69; always_ff @(posedge clk) begin _data_in_D69 <= _data_in_D68; end
/*latency*/ logic[6:0] _data_in_D70; always_ff @(posedge clk) begin _data_in_D70 <= _data_in_D69; end
/*latency*/ logic[6:0] _data_in_D71; always_ff @(posedge clk) begin _data_in_D71 <= _data_in_D70; end
/*latency*/ logic[6:0] _data_in_D72; always_ff @(posedge clk) begin _data_in_D72 <= _data_in_D71; end
/*latency*/ logic[6:0] _data_in_D73; always_ff @(posedge clk) begin _data_in_D73 <= _data_in_D72; end
/*latency*/ logic[6:0] _data_in_D74; always_ff @(posedge clk) begin _data_in_D74 <= _data_in_D73; end
/*latency*/ logic[6:0] _data_in_D75; always_ff @(posedge clk) begin _data_in_D75 <= _data_in_D74; end
/*latency*/ logic[6:0] _data_in_D76; always_ff @(posedge clk) begin _data_in_D76 <= _data_in_D75; end
/*latency*/ logic[6:0] _data_in_D77; always_ff @(posedge clk) begin _data_in_D77 <= _data_in_D76; end
/*latency*/ logic[6:0] _data_in_D78; always_ff @(posedge clk) begin _data_in_D78 <= _data_in_D77; end
/*latency*/ logic[6:0] _data_in_D79; always_ff @(posedge clk) begin _data_in_D79 <= _data_in_D78; end
/*latency*/ logic[6:0] _data_in_D80; always_ff @(posedge clk) begin _data_in_D80 <= _data_in_D79; end
/*latency*/ logic[6:0] _data_in_D81; always_ff @(posedge clk) begin _data_in_D81 <= _data_in_D80; end
/*latency*/ logic[6:0] _data_in_D82; always_ff @(posedge clk) begin _data_in_D82 <= _data_in_D81; end
/*latency*/ logic[6:0] _data_in_D83; always_ff @(posedge clk) begin _data_in_D83 <= _data_in_D82; end
/*latency*/ logic[6:0] _data_in_D84; always_ff @(posedge clk) begin _data_in_D84 <= _data_in_D83; end
/*latency*/ logic[6:0] _data_in_D85; always_ff @(posedge clk) begin _data_in_D85 <= _data_in_D84; end
/*latency*/ logic[6:0] _data_in_D86; always_ff @(posedge clk) begin _data_in_D86 <= _data_in_D85; end
/*latency*/ logic[6:0] _data_in_D87; always_ff @(posedge clk) begin _data_in_D87 <= _data_in_D86; end
/*latency*/ logic[6:0] _data_in_D88; always_ff @(posedge clk) begin _data_in_D88 <= _data_in_D87; end
/*latency*/ logic[6:0] _data_in_D89; always_ff @(posedge clk) begin _data_in_D89 <= _data_in_D88; end
/*latency*/ logic[6:0] _data_in_D90; always_ff @(posedge clk) begin _data_in_D90 <= _data_in_D89; end
/*latency*/ logic[6:0] _data_in_D91; always_ff @(posedge clk) begin _data_in_D91 <= _data_in_D90; end
/*latency*/ logic[6:0] _data_in_D92; always_ff @(posedge clk) begin _data_in_D92 <= _data_in_D91; end
/*latency*/ logic[6:0] _data_in_D93; always_ff @(posedge clk) begin _data_in_D93 <= _data_in_D92; end
/*latency*/ logic[6:0] _data_in_D94; always_ff @(posedge clk) begin _data_in_D94 <= _data_in_D93; end
/*latency*/ logic[6:0] _data_in_D95; always_ff @(posedge clk) begin _data_in_D95 <= _data_in_D94; end
/*latency*/ logic[6:0] _data_in_D96; always_ff @(posedge clk) begin _data_in_D96 <= _data_in_D95; end
/*latency*/ logic[6:0] _data_in_D97; always_ff @(posedge clk) begin _data_in_D97 <= _data_in_D96; end
/*latency*/ logic[6:0] _data_in_D98; always_ff @(posedge clk) begin _data_in_D98 <= _data_in_D97; end
/*latency*/ logic[6:0] _data_in_D99; always_ff @(posedge clk) begin _data_in_D99 <= _data_in_D98; end
/*latency*/ logic[6:0] _data_in_D100; always_ff @(posedge clk) begin _data_in_D100 <= _data_in_D99; end
/*latency*/ logic[6:0] _data_in_D101; always_ff @(posedge clk) begin _data_in_D101 <= _data_in_D100; end
/*latency*/ logic[6:0] _data_in_D102; always_ff @(posedge clk) begin _data_in_D102 <= _data_in_D101; end
/*latency*/ logic[6:0] _data_in_D103; always_ff @(posedge clk) begin _data_in_D103 <= _data_in_D102; end
/*latency*/ logic[6:0] _data_in_D104; always_ff @(posedge clk) begin _data_in_D104 <= _data_in_D103; end
/*latency*/ logic[6:0] _data_in_D105; always_ff @(posedge clk) begin _data_in_D105 <= _data_in_D104; end
/*latency*/ logic[6:0] _data_in_D106; always_ff @(posedge clk) begin _data_in_D106 <= _data_in_D105; end
/*latency*/ logic[6:0] _data_in_D107; always_ff @(posedge clk) begin _data_in_D107 <= _data_in_D106; end
/*latency*/ logic[6:0] _data_in_D108; always_ff @(posedge clk) begin _data_in_D108 <= _data_in_D107; end
/*latency*/ logic[6:0] _data_in_D109; always_ff @(posedge clk) begin _data_in_D109 <= _data_in_D108; end
/*latency*/ logic[6:0] _data_in_D110; always_ff @(posedge clk) begin _data_in_D110 <= _data_in_D109; end
/*latency*/ logic[6:0] _data_in_D111; always_ff @(posedge clk) begin _data_in_D111 <= _data_in_D110; end
/*latency*/ logic[6:0] _data_in_D112; always_ff @(posedge clk) begin _data_in_D112 <= _data_in_D111; end
/*latency*/ logic[6:0] _data_in_D113; always_ff @(posedge clk) begin _data_in_D113 <= _data_in_D112; end
/*latency*/ logic[6:0] _data_in_D114; always_ff @(posedge clk) begin _data_in_D114 <= _data_in_D113; end
/*latency*/ logic[6:0] _data_in_D115; always_ff @(posedge clk) begin _data_in_D115 <= _data_in_D114; end
/*latency*/ logic[6:0] _data_in_D116; always_ff @(posedge clk) begin _data_in_D116 <= _data_in_D115; end
/*latency*/ logic[6:0] _data_in_D117; always_ff @(posedge clk) begin _data_in_D117 <= _data_in_D116; end
/*latency*/ logic[6:0] _data_in_D118; always_ff @(posedge clk) begin _data_in_D118 <= _data_in_D117; end
/*latency*/ logic[6:0] _data_in_D119; always_ff @(posedge clk) begin _data_in_D119 <= _data_in_D118; end
/*latency*/ logic[6:0] _data_in_D120; always_ff @(posedge clk) begin _data_in_D120 <= _data_in_D119; end
/*latency*/ logic[6:0] _data_in_D121; always_ff @(posedge clk) begin _data_in_D121 <= _data_in_D120; end
/*latency*/ logic[6:0] _data_in_D122; always_ff @(posedge clk) begin _data_in_D122 <= _data_in_D121; end
/*latency*/ logic[6:0] _data_in_D123; always_ff @(posedge clk) begin _data_in_D123 <= _data_in_D122; end
/*latency*/ logic[6:0] _data_in_D124; always_ff @(posedge clk) begin _data_in_D124 <= _data_in_D123; end
/*latency*/ logic[6:0] _data_in_D125; always_ff @(posedge clk) begin _data_in_D125 <= _data_in_D124; end
/*latency*/ logic[6:0] _data_in_D126; always_ff @(posedge clk) begin _data_in_D126 <= _data_in_D125; end
/*latency*/ logic[6:0] _data_in_D127; always_ff @(posedge clk) begin _data_in_D127 <= _data_in_D126; end
/*latency*/ logic[6:0] _data_in_D128; always_ff @(posedge clk) begin _data_in_D128 <= _data_in_D127; end
/*latency*/ logic[6:0] _data_in_D129; always_ff @(posedge clk) begin _data_in_D129 <= _data_in_D128; end
/*latency*/ logic[6:0] _data_in_D130; always_ff @(posedge clk) begin _data_in_D130 <= _data_in_D129; end
/*latency*/ logic[6:0] _data_in_D131; always_ff @(posedge clk) begin _data_in_D131 <= _data_in_D130; end
/*latency*/ logic[6:0] _data_in_D132; always_ff @(posedge clk) begin _data_in_D132 <= _data_in_D131; end
/*latency*/ logic[6:0] _data_in_D133; always_ff @(posedge clk) begin _data_in_D133 <= _data_in_D132; end
/*latency*/ logic[6:0] _data_in_D134; always_ff @(posedge clk) begin _data_in_D134 <= _data_in_D133; end
/*latency*/ logic[6:0] _data_in_D135; always_ff @(posedge clk) begin _data_in_D135 <= _data_in_D134; end
/*latency*/ logic[6:0] _data_in_D136; always_ff @(posedge clk) begin _data_in_D136 <= _data_in_D135; end
/*latency*/ logic[6:0] _data_in_D137; always_ff @(posedge clk) begin _data_in_D137 <= _data_in_D136; end
/*latency*/ logic[6:0] _data_in_D138; always_ff @(posedge clk) begin _data_in_D138 <= _data_in_D137; end
/*latency*/ logic[6:0] _data_in_D139; always_ff @(posedge clk) begin _data_in_D139 <= _data_in_D138; end
/*latency*/ logic[6:0] _data_in_D140; always_ff @(posedge clk) begin _data_in_D140 <= _data_in_D139; end
/*latency*/ logic[6:0] _data_in_D141; always_ff @(posedge clk) begin _data_in_D141 <= _data_in_D140; end
/*latency*/ logic[6:0] _data_in_D142; always_ff @(posedge clk) begin _data_in_D142 <= _data_in_D141; end
/*latency*/ logic[6:0] _data_in_D143; always_ff @(posedge clk) begin _data_in_D143 <= _data_in_D142; end
/*latency*/ logic[6:0] _data_in_D144; always_ff @(posedge clk) begin _data_in_D144 <= _data_in_D143; end
/*latency*/ logic[6:0] _data_in_D145; always_ff @(posedge clk) begin _data_in_D145 <= _data_in_D144; end
/*latency*/ logic[6:0] _data_in_D146; always_ff @(posedge clk) begin _data_in_D146 <= _data_in_D145; end
/*latency*/ logic[6:0] _data_in_D147; always_ff @(posedge clk) begin _data_in_D147 <= _data_in_D146; end
/*latency*/ logic[6:0] _data_in_D148; always_ff @(posedge clk) begin _data_in_D148 <= _data_in_D147; end
/*latency*/ logic[6:0] _data_in_D149; always_ff @(posedge clk) begin _data_in_D149 <= _data_in_D148; end
/*latency*/ logic[6:0] _data_in_D150; always_ff @(posedge clk) begin _data_in_D150 <= _data_in_D149; end
/*latency*/ logic[6:0] _data_in_D151; always_ff @(posedge clk) begin _data_in_D151 <= _data_in_D150; end
/*latency*/ logic[6:0] _data_in_D152; always_ff @(posedge clk) begin _data_in_D152 <= _data_in_D151; end
/*latency*/ logic[6:0] _data_in_D153; always_ff @(posedge clk) begin _data_in_D153 <= _data_in_D152; end
/*latency*/ logic[6:0] _data_in_D154; always_ff @(posedge clk) begin _data_in_D154 <= _data_in_D153; end
/*latency*/ logic[6:0] _data_in_D155; always_ff @(posedge clk) begin _data_in_D155 <= _data_in_D154; end
/*latency*/ logic[6:0] _data_in_D156; always_ff @(posedge clk) begin _data_in_D156 <= _data_in_D155; end
/*latency*/ logic[6:0] _data_in_D157; always_ff @(posedge clk) begin _data_in_D157 <= _data_in_D156; end
/*latency*/ logic[6:0] _data_in_D158; always_ff @(posedge clk) begin _data_in_D158 <= _data_in_D157; end
/*latency*/ logic[6:0] _data_in_D159; always_ff @(posedge clk) begin _data_in_D159 <= _data_in_D158; end
/*latency*/ logic[6:0] _data_in_D160; always_ff @(posedge clk) begin _data_in_D160 <= _data_in_D159; end
/*latency*/ logic[6:0] _data_in_D161; always_ff @(posedge clk) begin _data_in_D161 <= _data_in_D160; end
/*latency*/ logic[6:0] _data_in_D162; always_ff @(posedge clk) begin _data_in_D162 <= _data_in_D161; end
/*latency*/ logic[6:0] _data_in_D163; always_ff @(posedge clk) begin _data_in_D163 <= _data_in_D162; end
/*latency*/ logic[6:0] _data_in_D164; always_ff @(posedge clk) begin _data_in_D164 <= _data_in_D163; end
/*latency*/ logic[6:0] _data_in_D165; always_ff @(posedge clk) begin _data_in_D165 <= _data_in_D164; end
/*latency*/ logic[6:0] _data_in_D166; always_ff @(posedge clk) begin _data_in_D166 <= _data_in_D165; end
/*latency*/ logic[6:0] _data_in_D167; always_ff @(posedge clk) begin _data_in_D167 <= _data_in_D166; end
/*latency*/ logic[6:0] _data_in_D168; always_ff @(posedge clk) begin _data_in_D168 <= _data_in_D167; end
/*latency*/ logic[6:0] _data_in_D169; always_ff @(posedge clk) begin _data_in_D169 <= _data_in_D168; end
/*latency*/ logic[6:0] _data_in_D170; always_ff @(posedge clk) begin _data_in_D170 <= _data_in_D169; end
/*latency*/ logic[6:0] _data_in_D171; always_ff @(posedge clk) begin _data_in_D171 <= _data_in_D170; end
/*latency*/ logic[6:0] _data_in_D172; always_ff @(posedge clk) begin _data_in_D172 <= _data_in_D171; end
/*latency*/ logic[6:0] _data_in_D173; always_ff @(posedge clk) begin _data_in_D173 <= _data_in_D172; end
/*latency*/ logic[6:0] _data_in_D174; always_ff @(posedge clk) begin _data_in_D174 <= _data_in_D173; end
/*latency*/ logic[6:0] _data_in_D175; always_ff @(posedge clk) begin _data_in_D175 <= _data_in_D174; end
/*latency*/ logic[6:0] _data_in_D176; always_ff @(posedge clk) begin _data_in_D176 <= _data_in_D175; end
/*latency*/ logic[6:0] _data_in_D177; always_ff @(posedge clk) begin _data_in_D177 <= _data_in_D176; end
/*latency*/ logic[6:0] _data_in_D178; always_ff @(posedge clk) begin _data_in_D178 <= _data_in_D177; end
/*latency*/ logic[6:0] _data_in_D179; always_ff @(posedge clk) begin _data_in_D179 <= _data_in_D178; end
/*latency*/ logic[6:0] _data_in_D180; always_ff @(posedge clk) begin _data_in_D180 <= _data_in_D179; end
/*latency*/ logic[6:0] _data_in_D181; always_ff @(posedge clk) begin _data_in_D181 <= _data_in_D180; end
/*latency*/ logic[6:0] _data_in_D182; always_ff @(posedge clk) begin _data_in_D182 <= _data_in_D181; end
/*latency*/ logic[6:0] _data_in_D183; always_ff @(posedge clk) begin _data_in_D183 <= _data_in_D182; end
/*latency*/ logic[6:0] _data_in_D184; always_ff @(posedge clk) begin _data_in_D184 <= _data_in_D183; end
/*latency*/ logic[6:0] _data_in_D185; always_ff @(posedge clk) begin _data_in_D185 <= _data_in_D184; end
/*latency*/ logic[6:0] _data_in_D186; always_ff @(posedge clk) begin _data_in_D186 <= _data_in_D185; end
/*latency*/ logic[6:0] _data_in_D187; always_ff @(posedge clk) begin _data_in_D187 <= _data_in_D186; end
/*latency*/ logic[6:0] _data_in_D188; always_ff @(posedge clk) begin _data_in_D188 <= _data_in_D187; end
/*latency*/ logic[6:0] _data_in_D189; always_ff @(posedge clk) begin _data_in_D189 <= _data_in_D188; end
/*latency*/ logic[6:0] _data_in_D190; always_ff @(posedge clk) begin _data_in_D190 <= _data_in_D189; end
/*latency*/ logic[6:0] _data_in_D191; always_ff @(posedge clk) begin _data_in_D191 <= _data_in_D190; end
/*latency*/ logic[6:0] _data_in_D192; always_ff @(posedge clk) begin _data_in_D192 <= _data_in_D191; end
/*latency*/ logic[6:0] _data_in_D193; always_ff @(posedge clk) begin _data_in_D193 <= _data_in_D192; end
/*latency*/ logic[6:0] _data_in_D194; always_ff @(posedge clk) begin _data_in_D194 <= _data_in_D193; end
/*latency*/ logic[6:0] _data_in_D195; always_ff @(posedge clk) begin _data_in_D195 <= _data_in_D194; end
/*latency*/ logic[6:0] _data_in_D196; always_ff @(posedge clk) begin _data_in_D196 <= _data_in_D195; end
/*latency*/ logic[6:0] _data_in_D197; always_ff @(posedge clk) begin _data_in_D197 <= _data_in_D196; end
/*latency*/ logic[6:0] _data_in_D198; always_ff @(posedge clk) begin _data_in_D198 <= _data_in_D197; end
/*latency*/ logic[6:0] _data_in_D199; always_ff @(posedge clk) begin _data_in_D199 <= _data_in_D198; end
/*latency*/ logic[6:0] _data_in_D200; always_ff @(posedge clk) begin _data_in_D200 <= _data_in_D199; end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	data_out = 7'dx;
	data_out = _data_in_D200;
end
endmodule

// matrix_vector_mul #()
module matrix_vector_mul(
	input clk,
	input wire[6:0] mat[3:0][5:0],
	input wire[6:0] vec[3:0],
	output /*mux_wire*/ logic[15:0] result[5:0]
);

/*mux_wire*/ logic[13:0] row_products[3:0];
wire[6:0] _1 = mat[0][0];
wire[6:0] _2 = vec[0];
wire[13:0] _3;
assign _3 = _1 * _2;
wire[6:0] _4 = mat[1][0];
wire[6:0] _5 = vec[1];
wire[13:0] _6;
assign _6 = _4 * _5;
wire[6:0] _7 = mat[2][0];
wire[6:0] _8 = vec[2];
wire[13:0] _9;
assign _9 = _7 * _8;
wire[6:0] _10 = mat[3][0];
wire[6:0] _11 = vec[3];
wire[13:0] _12;
assign _12 = _10 * _11;
wire[15:0] _14;
always_comb begin
	_14 = 0;
	for(int _v0 = 0; _v0 < 4; _v0 += 1) _14 += row_products[_v0];
end
/*mux_wire*/ logic[13:0] row_products_2[3:0];
wire[6:0] _15 = mat[0][1];
wire[6:0] _16 = vec[0];
wire[13:0] _17;
assign _17 = _15 * _16;
wire[6:0] _18 = mat[1][1];
wire[6:0] _19 = vec[1];
wire[13:0] _20;
assign _20 = _18 * _19;
wire[6:0] _21 = mat[2][1];
wire[6:0] _22 = vec[2];
wire[13:0] _23;
assign _23 = _21 * _22;
wire[6:0] _24 = mat[3][1];
wire[6:0] _25 = vec[3];
wire[13:0] _26;
assign _26 = _24 * _25;
wire[15:0] _28;
always_comb begin
	_28 = 0;
	for(int _v0 = 0; _v0 < 4; _v0 += 1) _28 += row_products_2[_v0];
end
/*mux_wire*/ logic[13:0] row_products_3[3:0];
wire[6:0] _29 = mat[0][2];
wire[6:0] _30 = vec[0];
wire[13:0] _31;
assign _31 = _29 * _30;
wire[6:0] _32 = mat[1][2];
wire[6:0] _33 = vec[1];
wire[13:0] _34;
assign _34 = _32 * _33;
wire[6:0] _35 = mat[2][2];
wire[6:0] _36 = vec[2];
wire[13:0] _37;
assign _37 = _35 * _36;
wire[6:0] _38 = mat[3][2];
wire[6:0] _39 = vec[3];
wire[13:0] _40;
assign _40 = _38 * _39;
wire[15:0] _42;
always_comb begin
	_42 = 0;
	for(int _v0 = 0; _v0 < 4; _v0 += 1) _42 += row_products_3[_v0];
end
/*mux_wire*/ logic[13:0] row_products_4[3:0];
wire[6:0] _43 = mat[0][3];
wire[6:0] _44 = vec[0];
wire[13:0] _45;
assign _45 = _43 * _44;
wire[6:0] _46 = mat[1][3];
wire[6:0] _47 = vec[1];
wire[13:0] _48;
assign _48 = _46 * _47;
wire[6:0] _49 = mat[2][3];
wire[6:0] _50 = vec[2];
wire[13:0] _51;
assign _51 = _49 * _50;
wire[6:0] _52 = mat[3][3];
wire[6:0] _53 = vec[3];
wire[13:0] _54;
assign _54 = _52 * _53;
wire[15:0] _56;
always_comb begin
	_56 = 0;
	for(int _v0 = 0; _v0 < 4; _v0 += 1) _56 += row_products_4[_v0];
end
/*mux_wire*/ logic[13:0] row_products_5[3:0];
wire[6:0] _57 = mat[0][4];
wire[6:0] _58 = vec[0];
wire[13:0] _59;
assign _59 = _57 * _58;
wire[6:0] _60 = mat[1][4];
wire[6:0] _61 = vec[1];
wire[13:0] _62;
assign _62 = _60 * _61;
wire[6:0] _63 = mat[2][4];
wire[6:0] _64 = vec[2];
wire[13:0] _65;
assign _65 = _63 * _64;
wire[6:0] _66 = mat[3][4];
wire[6:0] _67 = vec[3];
wire[13:0] _68;
assign _68 = _66 * _67;
wire[15:0] _70;
always_comb begin
	_70 = 0;
	for(int _v0 = 0; _v0 < 4; _v0 += 1) _70 += row_products_5[_v0];
end
/*mux_wire*/ logic[13:0] row_products_6[3:0];
wire[6:0] _71 = mat[0][5];
wire[6:0] _72 = vec[0];
wire[13:0] _73;
assign _73 = _71 * _72;
wire[6:0] _74 = mat[1][5];
wire[6:0] _75 = vec[1];
wire[13:0] _76;
assign _76 = _74 * _75;
wire[6:0] _77 = mat[2][5];
wire[6:0] _78 = vec[2];
wire[13:0] _79;
assign _79 = _77 * _78;
wire[6:0] _80 = mat[3][5];
wire[6:0] _81 = vec[3];
wire[13:0] _82;
assign _82 = _80 * _81;
wire[15:0] _84;
always_comb begin
	_84 = 0;
	for(int _v0 = 0; _v0 < 4; _v0 += 1) _84 += row_products_6[_v0];
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	result = '{16'dx, 16'dx, 16'dx, 16'dx, 16'dx, 16'dx};
	result[0] = _14;
	result[1] = _28;
	result[2] = _42;
	result[3] = _56;
	result[4] = _70;
	result[5] = _84;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	row_products = '{14'dx, 14'dx, 14'dx, 14'dx};
	row_products[0] = _3;
	row_products[1] = _6;
	row_products[2] = _9;
	row_products[3] = _12;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	row_products_2 = '{14'dx, 14'dx, 14'dx, 14'dx};
	row_products_2[0] = _17;
	row_products_2[1] = _20;
	row_products_2[2] = _23;
	row_products_2[3] = _26;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	row_products_3 = '{14'dx, 14'dx, 14'dx, 14'dx};
	row_products_3[0] = _31;
	row_products_3[1] = _34;
	row_products_3[2] = _37;
	row_products_3[3] = _40;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	row_products_4 = '{14'dx, 14'dx, 14'dx, 14'dx};
	row_products_4[0] = _45;
	row_products_4[1] = _48;
	row_products_4[2] = _51;
	row_products_4[3] = _54;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	row_products_5 = '{14'dx, 14'dx, 14'dx, 14'dx};
	row_products_5[0] = _59;
	row_products_5[1] = _62;
	row_products_5[2] = _65;
	row_products_5[3] = _68;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	row_products_6 = '{14'dx, 14'dx, 14'dx, 14'dx};
	row_products_6[0] = _73;
	row_products_6[1] = _76;
	row_products_6[2] = _79;
	row_products_6[3] = _82;
end
endmodule

// module_taking_time #()
module module_taking_time(
	input clk,
	input wire[6:0] i,
	output /*mux_wire*/ logic[6:0] o
);

/*latency*/ logic[6:0] _i_D1; always_ff @(posedge clk) begin _i_D1 <= i; end
/*latency*/ logic[6:0] _i_D2; always_ff @(posedge clk) begin _i_D2 <= _i_D1; end
/*latency*/ logic[6:0] _i_D3; always_ff @(posedge clk) begin _i_D3 <= _i_D2; end
/*latency*/ logic[6:0] _i_D4; always_ff @(posedge clk) begin _i_D4 <= _i_D3; end
/*latency*/ logic[6:0] _i_D5; always_ff @(posedge clk) begin _i_D5 <= _i_D4; end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 7'dx;
	o = _i_D5;
end
endmodule

// determinable_because_no_input_output_ports #()
module determinable_because_no_input_output_ports(
	input clk,
	input wire[6:0] a,
	output /*mux_wire*/ logic[7:0] x
);

/*latency*/ logic[6:0] _a_D1; always_ff @(posedge clk) begin _a_D1 <= a; end
/*latency*/ logic[6:0] _a_D2; always_ff @(posedge clk) begin _a_D2 <= _a_D1; end
/*latency*/ logic[6:0] _a_D3; always_ff @(posedge clk) begin _a_D3 <= _a_D2; end
/*mux_wire*/ logic[6:0] a_d;
/*mux_wire*/ logic[6:0] t;
/*latency*/ logic[6:0] _t_D2; always_ff @(posedge clk) begin _t_D2 <= t; end
/*mux_wire*/ logic[6:0] a_dd;
/*mux_wire*/ logic[6:0] t_d;
/*latency*/ logic[6:0] _t_d_D3; always_ff @(posedge clk) begin _t_d_D3 <= t_d; end
wire[7:0] _7;
assign _7 = _t_d_D3 + a_dd;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x = 8'dx;
	x = _7;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a_d = 7'dx;
	a_d = _a_D1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t = 7'dx;
	t = a_d;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a_dd = 7'dx;
	a_dd = _a_D3;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_d = 7'dx;
	t_d = _t_D2;
end
endmodule

// determinable_input_latency #()
module determinable_input_latency(
	input clk,
	input wire[6:0] a,
	input wire[6:0] b,
	output /*mux_wire*/ logic[8:0] x,
	output /*mux_wire*/ logic[7:0] y
);

/*latency*/ logic[6:0] _a_D1; always_ff @(posedge clk) begin _a_D1 <= a; end
/*latency*/ logic[6:0] _a_D2; always_ff @(posedge clk) begin _a_D2 <= _a_D1; end
/*mux_wire*/ logic[6:0] a_d;
/*mux_wire*/ logic[7:0] t;
/*latency*/ logic[7:0] _t_D2; always_ff @(posedge clk) begin _t_D2 <= t; end
wire[7:0] _4;
assign _4 = a_d + b;
/*mux_wire*/ logic[6:0] a_dd;
/*mux_wire*/ logic[7:0] t_d;
wire[8:0] _9;
assign _9 = t_d + a_dd;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x = 9'dx;
	x = _9;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 8'dx;
	y = t;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a_d = 7'dx;
	a_d = _a_D1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t = 8'dx;
	t = _4;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a_dd = 7'dx;
	a_dd = _a_D2;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_d = 8'dx;
	t_d = _t_D2;
end
endmodule

// specified_input_latency #()
module specified_input_latency(
	input clk,
	input wire[6:0] a,
	input wire[6:0] b,
	output /*mux_wire*/ logic[8:0] x,
	output /*mux_wire*/ logic[7:0] y
);

/*latency*/ logic[6:0] _a_D1; always_ff @(posedge clk) begin _a_D1 <= a; end
/*latency*/ logic[6:0] _a_D2; always_ff @(posedge clk) begin _a_D2 <= _a_D1; end
/*latency*/ logic[6:0] _a_D3; always_ff @(posedge clk) begin _a_D3 <= _a_D2; end
/*mux_wire*/ logic[6:0] a_d;
/*mux_wire*/ logic[7:0] t;
/*latency*/ logic[7:0] _t_D2; always_ff @(posedge clk) begin _t_D2 <= t; end
wire[7:0] _4;
assign _4 = a_d + b;
/*mux_wire*/ logic[6:0] a_dd;
/*mux_wire*/ logic[7:0] t_d;
/*latency*/ logic[7:0] _t_d_D3; always_ff @(posedge clk) begin _t_d_D3 <= t_d; end
wire[8:0] _9;
assign _9 = _t_d_D3 + a_dd;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	x = 9'dx;
	x = _9;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	y = 8'dx;
	y = t;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a_d = 7'dx;
	a_d = _a_D1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t = 8'dx;
	t = _4;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a_dd = 7'dx;
	a_dd = _a_D3;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_d = 8'dx;
	t_d = _t_D2;
end
endmodule

// test_single_wire #()
module test_single_wire(
	input clk,
	input wire[6:0] a,
	output /*mux_wire*/ logic[6:0] o
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 7'dx;
	o = a;
end
endmodule

// first_bit_idx_6 #()
module first_bit_idx_6(
	input clk,
	input wire[5:0] bits,
	output /*mux_wire*/ logic[2:0] first,
	output /*mux_wire*/ logic all_zeros
);

wire _1 = bits[0];
wire _3 = bits[1];
wire _5 = bits[2];
wire _7 = bits[3];
wire _9 = bits[4];
wire _11 = bits[5];
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	first = 3'dx;
	if(_1) first = 1'd0;
	if(!_1) if(_3) first = 1'd1;
	if(!_1) if(!_3) if(_5) first = 2'd2;
	if(!_1) if(!_3) if(!_5) if(_7) first = 2'd3;
	if(!_1) if(!_3) if(!_5) if(!_7) if(_9) first = 3'd4;
	if(!_1) if(!_3) if(!_5) if(!_7) if(!_9) if(_11) first = 3'd5;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	all_zeros = 1'bx;
	if(_1) all_zeros = 1'b0;
	if(!_1) if(_3) all_zeros = 1'b0;
	if(!_1) if(!_3) if(_5) all_zeros = 1'b0;
	if(!_1) if(!_3) if(!_5) if(_7) all_zeros = 1'b0;
	if(!_1) if(!_3) if(!_5) if(!_7) if(_9) all_zeros = 1'b0;
	if(!_1) if(!_3) if(!_5) if(!_7) if(!_9) if(_11) all_zeros = 1'b0;
	if(!_1) if(!_3) if(!_5) if(!_7) if(!_9) if(!_11) all_zeros = 1'b1;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	all_zeros = all_zeros;
end
endmodule

// add_indices_to_array #()
module add_indices_to_array(
	input clk,
	input wire[6:0] values[9:0],
	output /*mux_wire*/ logic[6:0] added_values[9:0]
);

/*mux_wire*/ logic[6:0] t;
wire[6:0] _1 = values[0];
wire[6:0] _3;
assign _3 = t + 1'd0;
/*mux_wire*/ logic[6:0] t_2;
wire[6:0] _4 = values[1];
wire[6:0] _6;
assign _6 = t_2 + 1'd1;
/*mux_wire*/ logic[6:0] t_3;
wire[6:0] _7 = values[2];
wire[6:0] _9;
assign _9 = t_3 + 2'd2;
/*mux_wire*/ logic[6:0] t_4;
wire[6:0] _10 = values[3];
wire[6:0] _12;
assign _12 = t_4 + 2'd3;
/*mux_wire*/ logic[6:0] t_5;
wire[6:0] _13 = values[4];
wire[6:0] _15;
assign _15 = t_5 + 3'd4;
/*mux_wire*/ logic[6:0] t_6;
wire[6:0] _16 = values[5];
wire[6:0] _18;
assign _18 = t_6 + 3'd5;
/*mux_wire*/ logic[6:0] t_7;
wire[6:0] _19 = values[6];
wire[6:0] _21;
assign _21 = t_7 + 3'd6;
/*mux_wire*/ logic[6:0] t_8;
wire[6:0] _22 = values[7];
wire[6:0] _24;
assign _24 = t_8 + 3'd7;
/*mux_wire*/ logic[6:0] t_9;
wire[6:0] _25 = values[8];
wire[6:0] _27;
assign _27 = t_9 + 4'd8;
/*mux_wire*/ logic[6:0] t_10;
wire[6:0] _28 = values[9];
wire[6:0] _30;
assign _30 = t_10 + 4'd9;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	added_values = '{7'dx, 7'dx, 7'dx, 7'dx, 7'dx, 7'dx, 7'dx, 7'dx, 7'dx, 7'dx};
	added_values[0] = _3;
	added_values[1] = _6;
	added_values[2] = _9;
	added_values[3] = _12;
	added_values[4] = _15;
	added_values[5] = _18;
	added_values[6] = _21;
	added_values[7] = _24;
	added_values[8] = _27;
	added_values[9] = _30;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t = 7'dx;
	t = _1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_2 = 7'dx;
	t_2 = _4;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_3 = 7'dx;
	t_3 = _7;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_4 = 7'dx;
	t_4 = _10;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_5 = 7'dx;
	t_5 = _13;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_6 = 7'dx;
	t_6 = _16;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_7 = 7'dx;
	t_7 = _19;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_8 = 7'dx;
	t_8 = _22;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_9 = 7'dx;
	t_9 = _25;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	t_10 = 7'dx;
	t_10 = _28;
end
endmodule

// Unpack4 #()
module Unpack4(
	input clk,
	input wire[6:0] packed_1[3:0],
	output /*mux_wire*/ logic[6:0] out_stream
);

/*state*/ logic[1:0] st = 2'd0;
/*state*/ logic[6:0] stored_packed[2:0];
wire _2;
assign _2 = st == 1'd0;
wire[6:0] _3 = packed_1[0];
wire[6:0] _4 = packed_1[1];
wire[6:0] _5 = packed_1[2];
wire[6:0] _6 = packed_1[3];
wire _10;
assign _10 = st == 1'd1;
wire[6:0] _11 = stored_packed[0];
wire _15;
assign _15 = st == 2'd2;
wire[6:0] _16 = stored_packed[1];
wire _20;
assign _20 = st == 2'd3;
wire[6:0] _21 = stored_packed[2];
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	out_stream = 7'dx;
	if(_2) out_stream = _3;
	if(!_2) if(_10) out_stream = _11;
	if(!_2) if(!_10) if(_15) out_stream = _16;
	if(!_2) if(!_10) if(!_15) if(_20) out_stream = _21;
end
always_ff @(posedge clk) begin
	if(_2) st <= 1'd1;
	if(!_2) if(_10) st <= 2'd2;
	if(!_2) if(!_10) if(_15) st <= 2'd3;
	if(!_2) if(!_10) if(!_15) if(_20) st <= 1'd0;
end
always_ff @(posedge clk) begin
	if(_2) stored_packed[0] <= _4;
	if(_2) stored_packed[1] <= _5;
	if(_2) stored_packed[2] <= _6;
end
endmodule

// blur #()
module blur(
	input clk,
	input wire[6:0] a,
	input wire done,
	output /*mux_wire*/ logic[7:0] result
);

/*state*/ logic working = 1'b0;
/*latency*/ logic _working_D1; always_ff @(posedge clk) begin _working_D1 <= working; end
/*latency*/ logic _working_D2; always_ff @(posedge clk) begin _working_D2 <= _working_D1; end
/*latency*/ logic _working_D3; always_ff @(posedge clk) begin _working_D3 <= _working_D2; end
/*state*/ logic[6:0] prev;
wire[7:0] _4;
assign _4 = prev + a;
/*latency*/ logic[7:0] __4_D1; always_ff @(posedge clk) begin __4_D1 <= _4; end
/*latency*/ logic[7:0] __4_D2; always_ff @(posedge clk) begin __4_D2 <= __4_D1; end
/*latency*/ logic[7:0] __4_D3; always_ff @(posedge clk) begin __4_D3 <= __4_D2; end
wire _7;
assign _7 = !done;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	result = 8'dx;
	if(_working_D3) result = __4_D3;
end
always_ff @(posedge clk) begin
	working <= _7;
end
always_ff @(posedge clk) begin
	prev <= a;
end
endmodule

// Accumulator #()
module Accumulator(
	input clk,
	input wire[6:0] term,
	input wire done,
	output /*mux_wire*/ logic[6:0] total
);

/*latency*/ logic _done_D1; always_ff @(posedge clk) begin _done_D1 <= done; end
/*state*/ logic[6:0] tot = 7'd0;
/*mux_wire*/ logic[6:0] new_tot;
/*latency*/ logic[6:0] _new_tot_D1; always_ff @(posedge clk) begin _new_tot_D1 <= new_tot; end
wire[7:0] _3;
assign _3 = tot + term;
wire[6:0] _5;
assign _5 = _3 - ((_3 >= 100) ? 100 : 0); // == mod 100
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	total = 7'dx;
	if(_done_D1) total = _new_tot_D1;
end
always_ff @(posedge clk) begin
	if(done) tot <= 1'd0;
	if(!done) tot <= new_tot;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	new_tot = 7'dx;
	new_tot = _5;
end
endmodule

// Tree_Multiply #()
module Tree_Multiply(
	input clk,
	input wire[6:0] values[3:0],
	output /*mux_wire*/ logic[26:0] total
);

/*mux_wire*/ logic[13:0] a;
wire[6:0] _1 = values[0];
wire[6:0] _2 = values[1];
wire[13:0] _3;
assign _3 = _1 * _2;
/*latency*/ logic[13:0] __3_D1; always_ff @(posedge clk) begin __3_D1 <= _3; end
/*mux_wire*/ logic[13:0] b;
wire[6:0] _4 = values[2];
wire[6:0] _5 = values[3];
wire[13:0] _6;
assign _6 = _4 * _5;
/*latency*/ logic[13:0] __6_D1; always_ff @(posedge clk) begin __6_D1 <= _6; end
wire[26:0] _9;
assign _9 = a * b;
/*latency*/ logic[26:0] __9_D2; always_ff @(posedge clk) begin __9_D2 <= _9; end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	total = 27'dx;
	total = __9_D2;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a = 14'dx;
	a = __3_D1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	b = 14'dx;
	b = __6_D1;
end
endmodule

// blur2 #()
module blur2(
	input clk,
	input wire[6:0] data,
	input wire first,
	output /*mux_wire*/ logic[6:0] blurred
);

/*state*/ logic[6:0] prev;
wire _2;
assign _2 = !first;
wire[7:0] _5;
assign _5 = data + prev;
wire[6:0] _7;
assign _7 = _5 - ((_5 >= 100) ? 100 : 0); // == mod 100
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	blurred = 7'dx;
	if(_2) blurred = _7;
end
always_ff @(posedge clk) begin
	prev <= data;
end
endmodule

// fibonnaci #()
module fibonnaci(
	input clk,
	output /*mux_wire*/ logic[6:0] num
);

/*state*/ logic[6:0] cur;
/*state*/ logic[6:0] prev;
wire[7:0] _5;
assign _5 = cur + prev;
wire[6:0] _7;
assign _7 = _5 - ((_5 >= 100) ? 100 : 0); // == mod 100
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	num = 7'dx;
	num = _7;
end
always_ff @(posedge clk) begin
	cur <= 1'd1;
	cur <= num;
end
always_ff @(posedge clk) begin
	prev <= 1'd0;
	prev <= cur;
end
endmodule

// test_pow17 #()
module test_pow17(
	input clk
);

/*mux_wire*/ logic[112:0] a;
/*mux_wire*/ logic[6:0] _pow17_i;
wire[112:0] _pow17_o;
pow17 pow17(
	.clk(clk),
	.i(_pow17_i),
	.o(_pow17_o)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	a = 113'dx;
	a = _pow17_o;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_pow17_i = 7'dx;
	_pow17_i = 2'd2;
end
endmodule

// pow17 #()
module pow17(
	input clk,
	input wire[6:0] i,
	output /*mux_wire*/ logic[112:0] o
);

/*latency*/ logic[6:0] _i_D1; always_ff @(posedge clk) begin _i_D1 <= i; end
/*latency*/ logic[6:0] _i_D2; always_ff @(posedge clk) begin _i_D2 <= _i_D1; end
/*mux_wire*/ logic[13:0] i2;
wire[13:0] _3;
assign _3 = i * i;
/*mux_wire*/ logic[26:0] i4;
wire[26:0] _6;
assign _6 = i2 * i2;
/*latency*/ logic[26:0] __6_D1; always_ff @(posedge clk) begin __6_D1 <= _6; end
/*mux_wire*/ logic[53:0] i8;
wire[53:0] _9;
assign _9 = i4 * i4;
/*mux_wire*/ logic[106:0] i16;
wire[106:0] _12;
assign _12 = i8 * i8;
/*latency*/ logic[106:0] __12_D2; always_ff @(posedge clk) begin __12_D2 <= _12; end
wire[112:0] _15;
assign _15 = i16 * _i_D2;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 113'dx;
	o = _15;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	i2 = 14'dx;
	i2 = _3;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	i4 = 27'dx;
	i4 = __6_D1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	i8 = 54'dx;
	i8 = _9;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	i16 = 107'dx;
	i16 = __12_D2;
end
endmodule

// multiply_add #()
module multiply_add(
	input clk,
	input wire[6:0] a,
	input wire[6:0] b,
	input wire[6:0] c,
	output /*mux_wire*/ logic[13:0] total
);

/*mux_wire*/ logic[13:0] tmp;
wire[13:0] _3;
assign _3 = a * b;
/*latency*/ logic[13:0] __3_D1; always_ff @(posedge clk) begin __3_D1 <= _3; end
wire[13:0] _6;
assign _6 = tmp + c;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	total = 14'dx;
	total = _6;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	tmp = 14'dx;
	tmp = __3_D1;
end
endmodule

// example_md #()
module example_md(
	input clk,
	input wire[6:0] factors[3:0],
	input wire[6:0] add_to,
	output /*mux_wire*/ logic[26:0] product,
	output /*mux_wire*/ logic[26:0] total
);

/*mux_wire*/ logic[13:0] mul0;
wire[6:0] _1 = factors[0];
wire[6:0] _2 = factors[1];
wire[13:0] _3;
assign _3 = _1 * _2;
/*latency*/ logic[13:0] __3_D1; always_ff @(posedge clk) begin __3_D1 <= _3; end
/*mux_wire*/ logic[13:0] mul1;
wire[6:0] _4 = factors[2];
wire[6:0] _5 = factors[3];
wire[13:0] _6;
assign _6 = _4 * _5;
/*latency*/ logic[13:0] __6_D1; always_ff @(posedge clk) begin __6_D1 <= _6; end
wire[26:0] _9;
assign _9 = mul0 * mul1;
/*latency*/ logic[26:0] __9_D2; always_ff @(posedge clk) begin __9_D2 <= _9; end
wire[26:0] _12;
assign _12 = product + add_to;
/*latency*/ logic[26:0] __12_D3; always_ff @(posedge clk) begin __12_D3 <= _12; end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	product = 27'dx;
	product = __9_D2;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	total = 27'dx;
	total = __12_D3;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	mul0 = 14'dx;
	mul0 = __3_D1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	mul1 = 14'dx;
	mul1 = __6_D1;
end
endmodule

// BoolToInt #()
module BoolToInt(
	input clk,
	input wire i,
	output /*mux_wire*/ logic[0:0] o
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 1'dx;
	if(i) o = 1'd1;
	if(!i) o = 1'd0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	o = o;
end
endmodule

// IntToBool #()
module IntToBool(
	input clk,
	input wire[0:0] i,
	output /*mux_wire*/ logic o
);

wire _3;
assign _3 = i == 1'd1;
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	o = 1'bx;
	o = _3;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	o = o;
end
endmodule

// TreeAdd #(WIDTH: 5, FROM: 3, TO: 4)
module TreeAdd_WIDTH_5_FROM_3_TO_4(
	input clk,
	input wire[1:0] values[4:0],
	output /*mux_wire*/ logic[3:0] total
);

genvar _g0;
/*mux_wire*/ logic[2:0] left_total;
/*latency*/ logic[2:0] _left_total_D2; always_ff @(posedge clk) begin _left_total_D2 <= left_total; end
wire[1:0] _1[1:0];
generate
for(_g0 = 0; _g0 < 2; _g0 = _g0 + 1) begin
assign _1[_g0] = values[_g0];
end
endgenerate
/*mux_wire*/ logic[1:0] _TreeAdd_values[1:0];
wire[2:0] _TreeAdd_total;
/*mux_wire*/ logic[3:0] right_total;
wire[1:0] _2[2:0];
generate
for(_g0 = 0; _g0 < 3; _g0 = _g0 + 1) begin
assign _2[_g0] = values[2 + _g0];
end
endgenerate
/*mux_wire*/ logic[1:0] _TreeAdd_2_values[2:0];
wire[3:0] _TreeAdd_2_total;
wire[3:0] _5;
assign _5 = _left_total_D2 + right_total;
/*latency*/ logic[3:0] __5_D3; always_ff @(posedge clk) begin __5_D3 <= _5; end
TreeAdd_WIDTH_2_FROM_3_TO_4 TreeAdd(
	.clk(clk),
	.values(_TreeAdd_values),
	.total(_TreeAdd_total)
);
TreeAdd_WIDTH_3_FROM_3_TO_4 TreeAdd_2(
	.clk(clk),
	.values(_TreeAdd_2_values),
	.total(_TreeAdd_2_total)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	total = 4'dx;
	total = __5_D3;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	left_total = 3'dx;
	left_total = _TreeAdd_total;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_TreeAdd_values = '{2'dx, 2'dx};
	for(int _v0 = 0; _v0 < 2; _v0 = _v0 + 1) begin
_TreeAdd_values[_v0] = _1[_v0];
end
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	right_total = 4'dx;
	right_total = _TreeAdd_2_total;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_TreeAdd_2_values = '{2'dx, 2'dx, 2'dx};
	for(int _v0 = 0; _v0 < 3; _v0 = _v0 + 1) begin
_TreeAdd_2_values[_v0] = _2[_v0];
end
end
endmodule

// TreeAdd #(WIDTH: 3, FROM: 3, TO: 4)
module TreeAdd_WIDTH_3_FROM_3_TO_4(
	input clk,
	input wire[1:0] values[2:0],
	output /*mux_wire*/ logic[3:0] total
);

genvar _g0;
/*mux_wire*/ logic[1:0] left_total;
/*latency*/ logic[1:0] _left_total_D1; always_ff @(posedge clk) begin _left_total_D1 <= left_total; end
wire[1:0] _1[0:0];
generate
for(_g0 = 0; _g0 < 1; _g0 = _g0 + 1) begin
assign _1[_g0] = values[_g0];
end
endgenerate
/*mux_wire*/ logic[1:0] _TreeAdd_values[0:0];
wire[1:0] _TreeAdd_total;
/*mux_wire*/ logic[2:0] right_total;
wire[1:0] _2[1:0];
generate
for(_g0 = 0; _g0 < 2; _g0 = _g0 + 1) begin
assign _2[_g0] = values[1 + _g0];
end
endgenerate
/*mux_wire*/ logic[1:0] _TreeAdd_2_values[1:0];
wire[2:0] _TreeAdd_2_total;
wire[3:0] _5;
assign _5 = _left_total_D1 + right_total;
/*latency*/ logic[3:0] __5_D2; always_ff @(posedge clk) begin __5_D2 <= _5; end
TreeAdd_WIDTH_1_FROM_3_TO_4 TreeAdd(
	.clk(clk),
	.values(_TreeAdd_values),
	.total(_TreeAdd_total)
);
TreeAdd_WIDTH_2_FROM_3_TO_4 TreeAdd_2(
	.clk(clk),
	.values(_TreeAdd_2_values),
	.total(_TreeAdd_2_total)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	total = 4'dx;
	total = __5_D2;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	left_total = 2'dx;
	left_total = _TreeAdd_total;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_TreeAdd_values = '{2'dx};
	for(int _v0 = 0; _v0 < 1; _v0 = _v0 + 1) begin
_TreeAdd_values[_v0] = _1[_v0];
end
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	right_total = 3'dx;
	right_total = _TreeAdd_2_total;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_TreeAdd_2_values = '{2'dx, 2'dx};
	for(int _v0 = 0; _v0 < 2; _v0 = _v0 + 1) begin
_TreeAdd_2_values[_v0] = _2[_v0];
end
end
endmodule

// TreeAdd #(WIDTH: 2, FROM: 3, TO: 4)
module TreeAdd_WIDTH_2_FROM_3_TO_4(
	input clk,
	input wire[1:0] values[1:0],
	output /*mux_wire*/ logic[2:0] total
);

genvar _g0;
/*mux_wire*/ logic[1:0] left_total;
wire[1:0] _1[0:0];
generate
for(_g0 = 0; _g0 < 1; _g0 = _g0 + 1) begin
assign _1[_g0] = values[_g0];
end
endgenerate
/*mux_wire*/ logic[1:0] _TreeAdd_values[0:0];
wire[1:0] _TreeAdd_total;
/*mux_wire*/ logic[1:0] right_total;
wire[1:0] _2[0:0];
generate
for(_g0 = 0; _g0 < 1; _g0 = _g0 + 1) begin
assign _2[_g0] = values[1 + _g0];
end
endgenerate
/*mux_wire*/ logic[1:0] _TreeAdd_2_values[0:0];
wire[1:0] _TreeAdd_2_total;
wire[2:0] _5;
assign _5 = left_total + right_total;
/*latency*/ logic[2:0] __5_D1; always_ff @(posedge clk) begin __5_D1 <= _5; end
TreeAdd_WIDTH_1_FROM_3_TO_4 TreeAdd(
	.clk(clk),
	.values(_TreeAdd_values),
	.total(_TreeAdd_total)
);
TreeAdd_WIDTH_1_FROM_3_TO_4 TreeAdd_2(
	.clk(clk),
	.values(_TreeAdd_2_values),
	.total(_TreeAdd_2_total)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	total = 3'dx;
	total = __5_D1;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	left_total = 2'dx;
	left_total = _TreeAdd_total;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_TreeAdd_values = '{2'dx};
	for(int _v0 = 0; _v0 < 1; _v0 = _v0 + 1) begin
_TreeAdd_values[_v0] = _1[_v0];
end
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	right_total = 2'dx;
	right_total = _TreeAdd_2_total;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_TreeAdd_2_values = '{2'dx};
	for(int _v0 = 0; _v0 < 1; _v0 = _v0 + 1) begin
_TreeAdd_2_values[_v0] = _2[_v0];
end
end
endmodule

// TreeAdd #(WIDTH: 1, FROM: 3, TO: 4)
module TreeAdd_WIDTH_1_FROM_3_TO_4(
	input clk,
	input wire[1:0] values[0:0],
	output /*mux_wire*/ logic[1:0] total
);

wire[1:0] _1 = values[0];
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	total = 2'dx;
	total = _1;
end
endmodule

// Repeat #(T: type bool #(), SIZE: 3)
module Repeat_T_type_bool_SIZE_3(
	input clk,
	input wire v,
	output /*mux_wire*/ logic[2:0] result
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	result = 3'bxxx;
	result[0] = v;
	result[1] = v;
	result[2] = v;
end
endmodule

// Repeat #(T: type bool #(), SIZE: 2)
module Repeat_T_type_bool_SIZE_2(
	input clk,
	input wire v,
	output /*mux_wire*/ logic[1:0] result
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	result = 2'bxx;
	result[0] = v;
	result[1] = v;
end
endmodule

// Repeat #(T: type bool #(), SIZE: 1)
module Repeat_T_type_bool_SIZE_1(
	input clk,
	input wire v,
	output /*mux_wire*/ logic[0:0] result
);

always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	result = 1'bx;
	result[0] = v;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	result = result;
end
endmodule

// transmute #(T1: type int #(FROM: 0, TO: 255)[2], T2: type int #(FROM: 0, TO: 65536))
module transmute_T1_type_int_FROM_0_TO_255_2_T2_type_int_FROM_0_TO_65536(
	input clk,
	input wire[7:0] a[1:0],
	output /*mux_wire*/ logic[15:0] b
);

/*mux_wire*/ logic[15:0] as_bits;
/*mux_wire*/ logic[7:0] _transmute_to_bits_value[1:0];
wire[15:0] _transmute_to_bits_bits;
/*mux_wire*/ logic[15:0] _transmute_from_bits_bits;
wire[15:0] _transmute_from_bits_value;
transmute_to_bits_T_type_int_FROM_0_TO_255_2 transmute_to_bits(
	.clk(clk),
	.value(_transmute_to_bits_value),
	.bits(_transmute_to_bits_bits)
);
transmute_from_bits_T_type_int_FROM_0_TO_65536 transmute_from_bits(
	.clk(clk),
	.bits(_transmute_from_bits_bits),
	.value(_transmute_from_bits_value)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	b = 16'dx;
	b = _transmute_from_bits_value;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	as_bits = 16'bxxxxxxxxxxxxxxxx;
	as_bits = _transmute_to_bits_bits;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_transmute_to_bits_value = '{8'dx, 8'dx};
	for(int _v0 = 0; _v0 < 2; _v0 = _v0 + 1) begin
_transmute_to_bits_value[_v0] = a[_v0];
end
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_transmute_from_bits_bits = 16'bxxxxxxxxxxxxxxxx;
	_transmute_from_bits_bits = as_bits;
end
endmodule

// transmute_from_bits #(T: type int #(FROM: 0, TO: 65536))
module transmute_from_bits_T_type_int_FROM_0_TO_65536(
	input clk,
	input wire[15:0] bits,
	output /*mux_wire*/ logic[15:0] value
);

assign value = bits;
endmodule

// transmute_to_bits #(T: type int #(FROM: 0, TO: 255)[2])
module transmute_to_bits_T_type_int_FROM_0_TO_255_2(
	input clk,
	input wire[7:0] value[1:0],
	output /*mux_wire*/ logic[15:0] bits
);

genvar _g0;
generate
for(_g0 = 0; _g0 < 2; _g0 = _g0 + 1) begin
assign bits[(_g0) * 8 +: 8] = value[_g0];
end
endgenerate
endmodule

// transmute_to_bits #(T: type bool #()[60])
module transmute_to_bits_T_type_bool_60(
	input clk,
	input wire[59:0] value,
	output /*mux_wire*/ logic[59:0] bits
);

assign bits = value;
endmodule

// UIntToBits #(NUM_BITS: 0)
module UIntToBits_NUM_BITS_0(
	input clk
	// (zero sized) input value
	// (zero sized) output bits
);

endmodule

// CrossActionNoData #()
module CrossActionNoData(
	input in_clk,
	input wire in,
	output /*mux_wire*/ logic out
);

/*mux_wire*/ logic _cross_valid_in;
wire _cross_valid_out;
CrossDomain_T_type_bool cross_valid(
	.in_clk(in_clk),
	.in(_cross_valid_in),
	.out(_cross_valid_out)
);
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	_cross_valid_in = 1'bx;
	if(in) _cross_valid_in = 1'b1;
	if(!in) _cross_valid_in = 1'b0;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	_cross_valid_in = _cross_valid_in;
end
always_comb begin
	// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches
	out = 1'bx;
	out = 1'b0;
	if(_cross_valid_out) out = 1'b1;
	// PATCH Vivado 23.1 Simulator Bug: 1-bit Conditional Assigns become don't care
	out = out;
end
endmodule

// CrossDomain #(T: type bool #())
module CrossDomain_T_type_bool(
	input in_clk,
	input wire in,
	output /*mux_wire*/ logic out
);

	assign out = in;
endmodule

