# Check for error regressions on git commit
cargo build &&
./target/debug/sus_compiler test.sus --ci --nocolor 1> test.sus_output.txt 2> test.sus_errors.txt &&
echo "SUS Error Regression test Finished"
