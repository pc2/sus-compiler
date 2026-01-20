
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

# XRT DRC violation on empty modules. 
## Affects: XRT 2.16

When an empty SystemVerilog module is code-generated, XRT produces an unneccecary error. See [https://github.com/pc2/sus-compiler/issues/155](https://github.com/pc2/sus-compiler/issues/155). 

`input.sus`
```
module empty_repeat {
    bool[32][0] b = Repeat(32'bBEEPB00P)
}
```

`codegen.sv`
```
// Repeat #(T: type bool #()[32], SIZE: 0)
module Repeat_T_type_bool_32_SIZE_0(
	input clk,
	input wire[31:0] v
	// (zero sized) output result
);

endmodule
```

`xrt_error.log`
```
....
[10:07:32] Run vpl: Step synth: Started
[10:08:04] Block-level synthesis in progress, 0 of 1 jobs complete, 1 job running.
[10:08:34] Block-level synthesis in progress, 1 of 1 jobs complete, 0 jobs running.
[10:09:05] Top-level synthesis in progress.
[10:09:37] Run vpl: Step synth: Completed
[10:09:37] Run vpl: Step impl: Started
[10:15:40] Run vpl: Step impl: Failed
[10:15:40] Finished 2nd of 6 tasks (FPGA linking synthesized kernels to platform). Elapsed time: 00h 10m 21s

[10:15:40] Starting logic optimization..
[10:15:41] Run vpl: FINISHED. Run Status: impl ERROR

===>The following messages were generated while processing /scratch/pc2-mitarbeiter/lennart/debug_johannes_bug/_x/link/vivado/vpl/prj/prj.runs/impl_1 :
ERROR: [VPL INBB-3] Black Box Instances: Cell 'level0_i/ulp/reproduce_inst/inst/ctrl/Repeat_5' of type 'reproduce_inst_0_Repeat_T_type_bool_32_SIZE_0' has undefined contents and is considered a black box.  The contents of this cell must be defined for opt_design to complete successfully.
ERROR: [VPL 4-78] Error(s) found during DRC. Opt_design not run.

===>The following messages were generated while  creating FPGA bitstream. Log file: /scratch/pc2-mitarbeiter/lennart/debug_johannes_bug/_x/link/vivado/vpl/runme.log :
ERROR: [VPL 12-13638] Failed runs(s) : 'impl_1'
ERROR: [VPL 60-773] In '/scratch/pc2-mitarbeiter/lennart/debug_johannes_bug/_x/link/vivado/vpl/runme.log', caught Tcl error:  ERROR: [Common 17-39] 'wait_on_runs' failed due to earlier errors.
WARNING: [VPL 60-732] Link warning: No monitor points found for BD automation.
ERROR: [VPL 60-704] Integration error, Failed to complete hardware generation. The run name is 'impl_1'. An error stack with function names and arguments may be available in the 'vivado.log'.
ERROR: [VPL 60-1328] Vpl run 'vpl' failed
ERROR: [VPL 60-806] Failed to finish platform linker
INFO: [v++ 60-1442] [10:15:43] Run run_link: Step vpl: Failed
Time (s): cpu = 00:00:06 ; elapsed = 00:10:25 . Memory (MB): peak = 478.949 ; gain = 0.000 ; free physical = 190520 ; free virtual = 419532
ERROR: [v++ 60-661] v++ link run 'run_link' failed
ERROR: [v++ 60-626] Kernel link failed to complete
ERROR: [v++ 60-703] Failed to finish linking
INFO: [v++ 60-1653] Closing dispatch client.
```

It is solved by emitting a do-nothing content for empty modules. We have chosen `initial begin end`. So the new codegen is:
```
// Repeat #(T: type bool #()[32], SIZE: 0)
module Repeat_T_type_bool_32_SIZE_0(
	input clk,
	input wire[31:0] v
	// (zero sized) output result
);

initial begin end

endmodule
```
