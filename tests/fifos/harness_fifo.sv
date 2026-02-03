module fifo_test;

////////////////////////////////////////////////////////////
// Clock / Reset
////////////////////////////////////////////////////////////

logic clk = 0;
logic rst;

always #5 clk = !clk;

////////////////////////////////////////////////////////////
// DUT interface signals
////////////////////////////////////////////////////////////

logic try_push = 0;
logic try_pop  = 0;

logic push_success;
logic [31:0] push_v;

logic pop_success;
logic [31:0] pop_v;

TestFIFO dut(
    .clk,
    .rst,
    .try_push,
    .try_pop,
    .push_success,
    .pop_success,
    .push_v,
    .pop_v
);

////////////////////////////////////////////////////////////
// Reference Model / Scoreboard
////////////////////////////////////////////////////////////

int unsigned ref_q[$];        // expected FIFO content
int unsigned push_cnt = 0;
int unsigned pop_cnt  = 0;

////////////////////////////////////////////////////////////
// Scoreboard logic
////////////////////////////////////////////////////////////

logic[31:0] expected;
always @(posedge clk) begin
    if (rst) begin
        ref_q.delete();
        push_cnt <= 0;
        pop_cnt  <= 0;
    end
    else begin
        // Track successful pushes
        if (push_success) begin
            ref_q.push_back(push_v);
            push_cnt++;
        end

        // Track successful pops + check order
        if (pop_success) begin
            if (ref_q.size() == 0) begin
                $fatal(1, "POP from empty FIFO at time %t", $time);
            end

            expected = ref_q.pop_front();

            if (expected !== pop_v) begin
                $fatal(1,
                    "FIFO order violation. Expected %0d got %0d at %t",
                    expected, pop_v, $time
                );
            end

            pop_cnt++;
        end
    end
end

////////////////////////////////////////////////////////////
// Convenience Tasks
////////////////////////////////////////////////////////////

task automatic idle_cycles(int n);
    try_push <= 0;
    try_pop  <= 0;
    repeat(n) @(posedge clk);
endtask

//----------------------------------------------------------
// Trickle push: push with gaps
//----------------------------------------------------------
task automatic trickle_push(int count, int gap = 2);
    for (int i = 0; i < count; i++) begin
        try_push <= 1;
        try_pop  <= 0;
        @(posedge clk);

        try_push <= 0;
        idle_cycles(gap);
    end
endtask

//----------------------------------------------------------
// Drain FIFO fully
//----------------------------------------------------------
task automatic drain_all();
    try_push <= 0;

    // continue until scoreboard empty and no pop happens
    while (ref_q.size() != 0) begin
        try_pop <= 1;
        @(posedge clk);
    end

    try_pop <= 0;
endtask

//----------------------------------------------------------
// Flood until FIFO refuses pushes
//----------------------------------------------------------
task automatic flood_push(int max_cycles = 200);
    int stall_cycles = 0;

    try_pop  <= 0;
    try_push <= 1;

    repeat(max_cycles) begin
        @(posedge clk);

        if (!push_success)
            stall_cycles++;
        else
            stall_cycles = 0;

        // assume full once push blocked for several cycles
        if (stall_cycles > 5)
            break;
    end

    try_push <= 0;
endtask

//----------------------------------------------------------
// Push Wall with trickle pop - Test the "bursting" effect when near the fill level of the fifo
//----------------------------------------------------------
task automatic push_wall_with_trickle_pop(int num_pops);
    try_push <= 1;
    try_pop <= 0;
    
    repeat(num_pops) begin
        repeat(9) @(posedge clk);
        try_pop <= 1;
        @(posedge clk);
        try_pop <= 0;
    end

    try_push <= 0;
    try_pop  <= 0;
endtask

//----------------------------------------------------------
// Steady simultaneous push + pop
//----------------------------------------------------------
task automatic steady_stream(int cycles);
    try_push <= 1;
    try_pop  <= 1;

    repeat(cycles) @(posedge clk);

    try_push <= 0;
    try_pop  <= 0;
endtask

//----------------------------------------------------------
// Random stress
//----------------------------------------------------------
task automatic random_stress(int cycles);
    for (int i = 0; i < cycles; i++) begin
        try_push <= $urandom_range(0,1);
        try_pop  <= $urandom_range(0,1);
        @(posedge clk);
    end

    try_push <= 0;
    try_pop  <= 0;
endtask

////////////////////////////////////////////////////////////
// Assertions / Protocol Checks
////////////////////////////////////////////////////////////

// Push success must correspond to try_push
always @(posedge clk) begin
    if (!rst && push_success && !try_push)
        $fatal(1, "push_success without try_push at %t", $time);
end

// Pop success must correspond to try_pop
always @(posedge clk) begin
    if (!rst && pop_success && !try_pop)
        $fatal(1, "pop_success without try_pop at %t", $time);
end

////////////////////////////////////////////////////////////
// Test Sequence
////////////////////////////////////////////////////////////

initial begin
    ////////////////////////////////////////////////////////
    // Reset
    ////////////////////////////////////////////////////////

    rst <= 1;
    idle_cycles(5);
    rst <= 0;
    idle_cycles(5);

    $display("---- TEST: pop from empty protection ----");
    try_pop <= 1;
    idle_cycles(5);
    try_pop <= 0;

    ////////////////////////////////////////////////////////
    // Trickle write from empty
    ////////////////////////////////////////////////////////

    $display("---- TEST: trickle push ----");
    trickle_push(10, 2);

    ////////////////////////////////////////////////////////
    // Blocked pop side (fill without popping)
    ////////////////////////////////////////////////////////

    $display("---- TEST: flood until full ----");
    flood_push();

    idle_cycles(5);

    ////////////////////////////////////////////////////////
    // Drain fully
    ////////////////////////////////////////////////////////

    $display("---- TEST: drain after full ----");
    drain_all();

    idle_cycles(5);

    ////////////////////////////////////////////////////////
    // Drain fully
    ////////////////////////////////////////////////////////

    $display("---- TEST: push wall with trickle pop ----");
    drain_all();
    push_wall_with_trickle_pop(30);

    idle_cycles(5);

    ////////////////////////////////////////////////////////
    // Alternating push/pop
    ////////////////////////////////////////////////////////

    $display("---- TEST: steady push+pop ----");
    steady_stream(100);

    ////////////////////////////////////////////////////////
    // Partial fill + partial drain
    ////////////////////////////////////////////////////////

    $display("---- TEST: partial regimes ----");
    trickle_push(20, 1);
    drain_all();

    ////////////////////////////////////////////////////////
    // Random stress
    ////////////////////////////////////////////////////////

    $display("---- TEST: randomized stress ----");
    random_stress(200);
    drain_all();

    ////////////////////////////////////////////////////////
    // Final checks
    ////////////////////////////////////////////////////////

    if (ref_q.size() != 0)
        $fatal(1, "Reference queue not empty at end");

    $display("======================================");
    $display("FIFO TEST PASSED");
    $display("Pushes: %0d Pops: %0d", push_cnt, pop_cnt);
    $display("======================================");

    $finish;
end

endmodule
