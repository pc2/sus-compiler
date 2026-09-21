# Simulate with Vivado

start_gui

create_project -force sus_test /tmp/sus_test -part xcvc1902-vsva2197-2MP-e-S

add_files testbench.sv
add_files codegen.sv

update_compile_order -fileset sources_1

set_property top MultiReduction_T_type_bool_32_NEUTRAL_ELEMENT_32_b00000000000000000000000000000000_OPERATOR_LATENCY_7_DATA_IN_LATENCY_2_MAX_DATAS_64_tb [get_filesets sim_1]

# set_property top fp_reduce_tests [get_filesets sim_1]
# set_property top_lib xil_defaultlib [get_filesets sim_1]

launch_simulation
