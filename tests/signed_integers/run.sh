set -e

cargo run signed_int_tests.sus -o codegen.sv --top AllTests

vivado -mode batch -source sim.tcl
