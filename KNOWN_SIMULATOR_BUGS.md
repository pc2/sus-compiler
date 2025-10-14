
# 1-bit Conditional Assigns become don't care
## Affects: Vivado 23.1, Vivado 23.2
## Fixed in: Vivado 24.1
## Workaround in: SUS 0.3.4 (crate::codegen::patches::patch_combinatorial_write_one_bit_dont_care) #127
For `logic[0:0]`:
```sv
module sim_reproduce_bug;

logic[0:0] write_to1;
logic[1:0] write_to2;
logic condition;
logic[7:0] v;

always_comb begin
    write_to1 = 1'bx;
    if(condition) write_to1 = v;
end
always_comb begin
    write_to2 = 2'bx;
    if(condition) write_to2 = v;
end

initial begin
    v = 8'b00000000;
    condition <= 0;
    #10
    condition <= 1;
    #10
    condition <= 0;
    #10
    $finish();
end

endmodule
```

<img width="521" height="158" alt="image" src="https://github.com/user-attachments/assets/61a133f4-fc6b-4e67-b163-447cd0cdbdb7" />

The workaround used is to simply add a `write_to1 = write_to1;` at the end of such always_comb blocks. 

For `logic`:
```sv
module sim_reproduce_bug2;

logic clk = 0;
always #5 clk = !clk;

logic write_to1;
logic[0:0] write_to1_arr;
logic write_to1_fix;
logic[0:0] write_to1_arr_fix;
logic condition;
logic v = 1;

always_comb begin
    write_to1 = 1'bx;
    if(condition) write_to1 = v;
end
always_comb begin
    write_to1_arr = 1'bx;
    if(condition) write_to1_arr = v;
end
always_comb begin
    write_to1_fix = 1'bx;
    if(condition) write_to1_fix = v;
    
    write_to1_fix = write_to1_fix; // Workaround!
end
always_comb begin
    write_to1_arr_fix = 1'bx;
    if(condition) write_to1_arr_fix = v;
    
    write_to1_arr_fix = write_to1_arr_fix; // Workaround!
end

initial begin
    condition = 0;
    #10
    condition = 1;
    #10
    condition = 0;
    #10
    $finish();
end

endmodule
```

<img width="658" height="225" alt="Image" src="https://github.com/user-attachments/assets/c38beea5-04ef-4321-a52e-c16155a70721" />

The code generator may generate 1-bit integers, and combinatorial conditional assignments with these integers do not work. (They remain don't-care)
