set -e

cargo run submodule_array_tests.sus -o codegen.sv --top TestFIFOArray --top TestAccumulatorGrid --top TestTriggerArray

verilator --binary --timing -Wno-fatal --top-module submodule_array_tests --Mdir /tmp/submodule_array_tests_verilator harness.sv codegen.sv
/tmp/submodule_array_tests_verilator/Vsubmodule_array_tests
