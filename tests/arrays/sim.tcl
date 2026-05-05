# Simulate with Vivado

create_project array_tests /tmp/array_tests -part xc7vx485tffg1157-1 -force

add_files harness.sv
add_files codegen.sv

set_property top array_tests [get_filesets sim_1]
set_property top_lib xil_defaultlib [get_filesets sim_1]
launch_simulation
