module fwft_fifo_test;

////////////////////////////////////////////////////////////
// Clock / Reset
////////////////////////////////////////////////////////////

logic clk = 0;
logic rst;

always #5 clk = !clk;

////////////////////////////////////////////////////////////
// SUS Interface Mapping
////////////////////////////////////////////////////////////

logic try_push;
logic try_pop;

logic push_success;
logic [31:0] push_v;

logic pop_available;
logic [31:0] pop_data;
wire pop = try_pop & pop_available;

TestFWFT dut (
    .clk,
    .rst,
    .try_push,
    .pop,
    .push_success,
    .push_v,
    .pop_available,
    .pop_data
);

////////////////////////////////////////////////////////////
// Reference Model
////////////////////////////////////////////////////////////

int unsigned ref_q[$];

int push_cnt;
int pop_cnt;

////////////////////////////////////////////////////////////
// Liveness Tracking (Push -> pop_available)
////////////////////////////////////////////////////////////

// Track pending pushes waiting to become visible
int pending_visibility[$]; // stores age counters

////////////////////////////////////////////////////////////
// Scoreboard + Properties
////////////////////////////////////////////////////////////

always @(posedge clk) begin
    if (rst) begin
        ref_q.delete();
        pending_visibility.delete();
        push_cnt <= 0;
        pop_cnt  <= 0;
    end
    else begin

        //--------------------------------------------------
        // PUSH TRACKING
        //--------------------------------------------------
        if (push_success) begin
            ref_q.push_back(push_v);
            pending_visibility.push_back(0);
            push_cnt++;
        end

        //--------------------------------------------------
        // EVENTUAL VISIBILITY PROPERTY (<=10 cycles)
        //--------------------------------------------------
        for (int i = 0; i < pending_visibility.size(); i++) begin
            pending_visibility[i]++;
            if (pending_visibility[i] > 10) begin
                $fatal(1,
                    "FWFT liveness failure: pushed data not visible within 10 cycles (time %t)",
                    $time
                );
            end
        end

        // Once data becomes visible at front, remove tracker
        if (pop_available && pending_visibility.size() > 0)
            pending_visibility.pop_front();

        //--------------------------------------------------
        // DATA CORRECTNESS WHEN VISIBLE
        //--------------------------------------------------
        if (pop_available) begin
            if (ref_q.size() == 0)
                $fatal(1, "FWFT exposed data while empty");

            if (pop_data !== ref_q[0]) begin
                $fatal(1,
                    "FWFT mismatch expected=%0d got=%0d time=%t",
                    ref_q[0], pop_data, $time
                );
            end
        end

        //--------------------------------------------------
        // DATA STABILITY UNTIL pop
        //--------------------------------------------------
        if (pop_available && !$past(pop_available)) begin
            // capture new visible word
        end

        if (pop_available && $past(pop_available) && !$past(pop)) begin
            if (pop_data !== $past(pop_data)) begin
                $fatal(1,
                    "FWFT stability violation: data changed before pop at %t",
                    $time
                );
            end
        end

        //--------------------------------------------------
        // pop CONSUMPTION
        //--------------------------------------------------
        if (pop) begin
            ref_q.pop_front();
            pop_cnt++;
        end

    end
end

////////////////////////////////////////////////////////////
// Protocol Assertions
////////////////////////////////////////////////////////////

// push_success only if try_push
always @(posedge clk) begin
    if (!rst && push_success && !try_push)
        $fatal(1,"push_success without try_push");
end

////////////////////////////////////////////////////////////
// Test Tasks
////////////////////////////////////////////////////////////

task idle_cycles(int n);
    try_push <= 0;
    try_pop <= 0;
    repeat(n) @(posedge clk);
endtask

task trickle_push(int n, int gap=2);
    for (int i=0;i<n;i++) begin
        try_push <= 1;
        try_pop <= 0;
        @(posedge clk);

        try_push <= 0;
        idle_cycles(gap);
    end
endtask

task drain_all();
    try_push <= 0;

    while (ref_q.size()!=0) begin
        try_pop <= pop_available;
        @(posedge clk);
        try_pop <= 0;
    end
endtask

task flood_push(int max=200);
    int stall=0;

    try_push <= 1;
    try_pop <= 0;

    repeat(max) begin
        @(posedge clk);

        if (!push_success)
            stall++;
        else
            stall=0;

        if (stall>5)
            break;
    end

    try_push <= 0;
endtask

task steady_stream(int cycles);
    try_push <= 1;

    repeat(cycles) begin
        try_pop <= pop_available;
        @(posedge clk);
        try_pop <= 0;
    end

    try_push <= 0;
endtask

task random_stress(int cycles);
    for (int i=0;i<cycles;i++) begin
        try_push <= $urandom_range(0,1);

        if (pop_available)
            try_pop <= $urandom_range(0,1);
        else
            try_pop <= 0;

        @(posedge clk);
        try_pop <= 0;
    end

    try_push <= 0;
endtask

////////////////////////////////////////////////////////////
// TEST SEQUENCE
////////////////////////////////////////////////////////////

initial begin

    //------------------------------------------------------
    // RESET
    //------------------------------------------------------
    rst <= 1;
    idle_cycles(5);
    rst <= 0;
    idle_cycles(5);

    //------------------------------------------------------
    // Empty stability
    //------------------------------------------------------
    repeat(5) begin
        try_pop <= 1;
        @(posedge clk);
        try_pop <= 0;
    end

    //------------------------------------------------------
    // Trickle push
    //------------------------------------------------------
    trickle_push(10,2);

    //------------------------------------------------------
    // Drain
    //------------------------------------------------------
    drain_all();

    //------------------------------------------------------
    // Flood + blocked try_pop
    //------------------------------------------------------
    flood_push();

    idle_cycles(5);

    //------------------------------------------------------
    // Full drain
    //------------------------------------------------------
    drain_all();

    //------------------------------------------------------
    // Streaming
    //------------------------------------------------------
    steady_stream(150);

    //------------------------------------------------------
    // Oscillation regime
    //------------------------------------------------------
    trickle_push(25,1);
    drain_all();

    //------------------------------------------------------
    // Random stress
    //------------------------------------------------------
    random_stress(1500);
    drain_all();

    //------------------------------------------------------
    // Final check
    //------------------------------------------------------
    if (ref_q.size()!=0)
        $fatal(1,"Reference queue not empty");

    $display("======================================");
    $display("FWFT FIFO TEST PASSED");
    $display("Push=%0d try_pop=%0d", push_cnt, pop_cnt);
    $display("======================================");

    $finish;
end

endmodule
