set -e

cargo run array_tests.sus -o codegen.sv --top AllTests

vivado -mode batch -source sim.tcl
