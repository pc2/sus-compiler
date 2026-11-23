# Simulate with Vivado

create_project -force submodule_array_tests /tmp/submodule_array_tests -part xc7vx485tffg1157-1

add_files harness.sv
add_files codegen.sv

set_property top submodule_array_tests [get_filesets sim_1]
set_property top_lib xil_defaultlib [get_filesets sim_1]
launch_simulation
run all
