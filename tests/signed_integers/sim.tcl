# Simulate with Vivado

create_project test_signed_ints /tmp/test_signed_ints -part xc7vx485tffg1157-1

add_files harness.sv
add_files codegen.sv

set_property top signed_ints_test [get_filesets sim_1]
set_property top_lib xil_defaultlib [get_filesets sim_1]
launch_simulation
