
# 1-bit Conditional Assigns become don't care
## Affects: Vivado 23.1, Vivado 23.2
## Fixed in: Vivado 24.2
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

The code generator may generate 1-bit integers, and combinatorial conditional assignments with these integers do not work. (They remain don't-care)
