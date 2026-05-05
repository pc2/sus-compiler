
module array_tests;

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

module assert_eq_bools#(parameter string ID = "...", parameter integer BITWIDTH) (
    input logic clk,
    input logic activate,
    input logic[31:0] idx,
    input logic[BITWIDTH-1:0] found,
    input logic[BITWIDTH-1:0] expected
);

always @(posedge clk) begin
    if(activate & (found != expected)) begin
        $display("Incorrect Value for %s idx %d: %d != %d ???", ID, idx, found, expected);
    end
end

endmodule

