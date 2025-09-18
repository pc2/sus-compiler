

module PopCount_Step3_to_2 #(parameter int WIDTH = 100) (
    input logic[WIDTH-1:0] bits,
    output logic[WIDTH / 3 + WIDTH % 3 - 1:0] reduced_low,
    output logic[WIDTH / 3 - 1:0] reduced_high
);

generate
// Full adder expressed in two separate parts
//assign reduced_low[WIDTH / 3 - 1 : 0] = bits[0 +: WIDTH / 3] ^ bits[WIDTH / 3 +: WIDTH / 3] ^ bits[WIDTH / 3 * 2 +: WIDTH / 3];
//assign reduced_high = (bits[0 +: WIDTH / 3] & bits[WIDTH / 3 +: WIDTH / 3]) 
//    | (bits[0 +: WIDTH / 3] & bits[WIDTH / 3 * 2 +: WIDTH / 3])
//    | (bits[WIDTH / 3 +: WIDTH / 3] & bits[WIDTH / 3 * 2 +: WIDTH / 3]);

// The order here is important, since we're recursively stacking modules on top of one another. By iterating from the front we limit the maximum tree depth
genvar i;
for(i = 0; i < WIDTH / 3; i = i + 1) begin
    logic[1:0] count;
    assign count = bits[i * 3] + bits[i * 3 + 1] + bits[i * 3 + 2];
    assign reduced_low[i] = count[0];
    assign reduced_high[i] = count[1];
end

if(WIDTH % 3 != 0) begin
    assign reduced_low[WIDTH / 3 +: WIDTH % 3] = bits[WIDTH / 3 * 3 +: WIDTH % 3];
end
endgenerate

endmodule

module PopCount_FullyReduce_Recurse #(parameter int LOW_C = 20, parameter int HIGH_C = 20) (
    input logic[LOW_C-1:0] low_bits,
    input logic[HIGH_C-1:0] high_bits,
    output logic[HIGH_C + LOW_C / 2 - 1 : 0] reduced_high,
    output logic lowest_result
);

generate
    if(LOW_C == 1) begin
        assign lowest_result = low_bits[0];
        assign reduced_high = high_bits;
    end else if(LOW_C == 2) begin
        assign lowest_result = low_bits[0] ^ low_bits[1];
        assign reduced_high = {high_bits, low_bits[0] & low_bits[1]};
    end else begin
        localparam NEW_LOW_C = LOW_C / 3 + LOW_C % 3;
        localparam NEW_HIGH_C = HIGH_C + LOW_C / 3;
        logic[NEW_LOW_C - 1:0] step_reduced_low;
        logic[LOW_C / 3 - 1:0] step_reduced_high;
        PopCount_Step3_to_2 #(.WIDTH(LOW_C)) step (
            .bits(low_bits),
            .reduced_low(step_reduced_low),
            .reduced_high(step_reduced_high)
        );

        logic[NEW_HIGH_C - 1 : 0] total_high_bits;
        assign total_high_bits = {high_bits, step_reduced_high};
        PopCount_FullyReduce_Recurse #(.LOW_C(NEW_LOW_C), .HIGH_C(NEW_HIGH_C)) recurse(
            .low_bits(step_reduced_low),
            .high_bits(total_high_bits),
            .reduced_high(reduced_high),
            .lowest_result(lowest_result)
        );
    end
endgenerate

endmodule

/*
Vivado report for WIDTH = 100:

8. Primitives
-------------

+----------+------+---------------------+
| Ref Name | Used | Functional Category |
+----------+------+---------------------+
| INBUF    |  100 |                 I/O |
| IBUFCTRL |  100 |              Others |
| LUT3     |   72 |                 CLB |
| LUT5     |   59 |                 CLB |
| LUT6     |   38 |                 CLB |
| OBUF     |    7 |                 I/O |
| LUT4     |    2 |                 CLB |
+----------+------+---------------------+

118 LUT total

The version with the big parallel XORs:
8. Primitives
-------------

+----------+------+---------------------+
| Ref Name | Used | Functional Category |
+----------+------+---------------------+
| INBUF    |  100 |                 I/O |
| IBUFCTRL |  100 |              Others |
| LUT5     |   72 |                 CLB |
| LUT3     |   71 |                 CLB |
| LUT6     |   28 |                 CLB |
| OBUF     |    7 |                 I/O |
| LUT4     |    4 |                 CLB |
| LUT2     |    1 |                 CLB |
+----------+------+---------------------+

117 LUT total - 7 Levels of Logic


*/
module PopCount_native #(parameter int WIDTH = 100) (
    input logic[WIDTH-1:0] bits,
    output logic[$clog2(WIDTH)-1:0] count
);

generate
    if(WIDTH == 1) begin
        assign count = bits;
    end else if(WIDTH == 2) begin
        assign count = bits[0] + bits[1];
    end else if(WIDTH == 3) begin
        assign count = bits[0] + bits[1] + bits[2];
    end else begin
        localparam LOW_C = WIDTH / 3 + WIDTH % 3;
        localparam HIGH_C = WIDTH / 3;
        logic[LOW_C - 1:0] reduced_low;
        logic[HIGH_C - 1:0] reduced_high;
        PopCount_Step3_to_2 #(.WIDTH(WIDTH)) step (
            .bits(bits),
            .reduced_low(reduced_low),
            .reduced_high(reduced_high)
        );

        logic[WIDTH/2-1:0] total_high_bits;
        PopCount_FullyReduce_Recurse #(.LOW_C(LOW_C), .HIGH_C(HIGH_C)) recurse(
            .low_bits(reduced_low),
            .high_bits(reduced_high),
            .reduced_high(total_high_bits),
            .lowest_result(count[0])
        );

        PopCount_native #(.WIDTH(WIDTH / 2)) popcnt_highs (
            .bits(total_high_bits),
            .count(count[$clog2(WIDTH)-1:1])
        );
    end
endgenerate

/*
8. Primitives
-------------

+----------+------+---------------------+
| Ref Name | Used | Functional Category |
+----------+------+---------------------+
| INBUF    |  100 |                 I/O |
| IBUFCTRL |  100 |              Others |
| LUT3     |   80 |                 CLB |
| LUT5     |   59 |                 CLB |
| LUT6     |   46 |                 CLB |
| OBUF     |    7 |                 I/O |
| LUT4     |    5 |                 CLB |
+----------+------+---------------------+

125 LUTs total
7 layers of logic
*/
module PopCount_native_naive #(parameter int WIDTH = 100) (
    input logic[WIDTH-1:0] bits,
    output logic[$clog2(WIDTH)-1:0] count
);

always_comb begin
    count = 0;
    for (int i = 0; i < WIDTH; i = i + 1) begin
        count = count + bits[i];
    end
end

endmodule



endmodule

// Simulation Testbench
module tb;
    parameter int WIDTH = 100;

    logic [WIDTH-1:0] bits;
    logic [$clog2(WIDTH+1)-1:0] dut_count;
    int ref_count;

    // DUT
    PopCount_native #(WIDTH) dut (
        .bits(bits),
        .count(dut_count)
    );

    initial begin
        int num_tests = 1000;
        int errors = 0;

        for (int i = 0; i < num_tests; i++) begin
            bits = $urandom();
            // for wider than 32 bits, combine multiple urandoms
            for (int j = 32; j < WIDTH; j += 32)
                bits[j +: 32] = $urandom();

            ref_count = $countones(bits);
            #1; // let DUT settle

            if (dut_count !== ref_count) begin
                $error("Mismatch at iter=%0d: bits=%h DUT=%0d REF=%0d",
                       i, bits, dut_count, ref_count);
                errors++;
            end
        end

        if (errors == 0)
            $display("All %0d tests passed ✅", num_tests);
        else
            $display("%0d errors found ❌", errors);

        $finish;
    end
endmodule
