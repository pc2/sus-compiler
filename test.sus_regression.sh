# Check for error regressions on git commit
cargo build &&
./target/debug/sus_compiler test.sus platform/xilinx/xpm.sus --ci --nocolor -o test.sus_codegen.sv 2> test.sus_errors.txt &&
echo "SUS Error Regression test Finished"
