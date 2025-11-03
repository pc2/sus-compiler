# Simulate with Vivado

create_project test_signed_ints /dev/shm/test_signed_ints -part xc7vx485tffg1157-1

add_files harness.sv
add_files signed_int_codegen.sv

set_property top signed_ints_test [get_filesets sim_1]
set_property top_lib xil_defaultlib [get_filesets sim_1]
launch_simulation
