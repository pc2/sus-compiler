set -e

cargo run submodule_array_tests.sus -o codegen.sv --top TestFIFOArray --top TestAccumulatorGrid --top TestTriggerArray

vivado -mode batch -source sim.tcl
