# Simulate with Vivado

create_project -force fifo_tests /tmp/fifo_tests -part xc7vx485tffg1157-1

add_files harness_fifo.sv
add_files harness_fwft.sv
add_files codegen.sv

set_property top fifo_tests [get_filesets sim_1]
set_property top_lib xil_defaultlib [get_filesets sim_1]
launch_simulation
