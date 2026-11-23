set -e

cargo run submodule_array_tests.sus -o codegen.sv --top TestFIFOArray --top TestAccumulatorGrid --top TestTriggerArray --top TestSplitCascade --top TestSplitAccumulatorRows

vivado -mode batch -source sim.tcl
