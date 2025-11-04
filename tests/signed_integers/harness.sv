
module signed_ints_test;

logic clk = 0;

initial begin
    $display("Starting tests");
    #5 clk = 1; // Only one clock pulse to run the asserts
    #5 clk = 0;
    $display("All tests passed");
    $finish();
end

AllTests test_harness(.clk);
//TestSpecificModulos test_specific(.clk);

endmodule

module assert_eq#(parameter string ID = "...") (
    input logic clk,
    input logic assert_eq,
    input logic signed[31:0] l,
    input logic signed[31:0] r,
    input logic signed[31:0] found,
    input logic signed[31:0] expected
);

always @(posedge clk) begin
    if(assert_eq & (found != expected)) begin
        $display("Incorrect Value %d %s %d = %d ???   ---    expected %d", l, ID, r, found, expected);
    end
end

endmodule

module test_modulo;
    logic signed[10:0] a = -50;
    logic[10:0] modulus = 7;
    logic signed[10:0] modulus_signed = 7;
    
    wire[10:0] result_cst = a % 7;
    wire[10:0] result_ucst = a % 4'd7;
    wire[10:0] result_mod = a % modulus;
    wire signed[10:0] result_mod_signed = a % modulus_signed;
    
    initial repeat(100) #10 a = a + 1;
    
    wire signed[10:0] pos_pos = 5 % 3;
    wire signed[10:0] pos_neg = 5 % (-3);
    wire signed[10:0] neg_pos = (-5) % 3;
    wire signed[10:0] neg_neg = (-5) % (-3);
endmodule



module TestShifts;
    logic[10:0] v = 100;
    logic signed[10:0] v_signed = 100;
      
    logic[4:0] shift = 12;
    logic[30:0] v_shifted = v << shift;
    logic signed[30:0] v_signed_shifted = v_signed << shift;
    
endmodule


