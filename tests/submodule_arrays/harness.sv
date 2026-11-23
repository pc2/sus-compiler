`timescale 1ns / 1ps

// Testbenches for submodule_array_tests.sus
//
// All testbenches drive their inputs on the falling clock edge, and check on the falling edge too,
// such that everything is stable around the rising edge where the DUT registers update.

////////////////////////////////////////////////////////////
// TestFIFOArray: random pushes and pops on 4 FIFOs through runtime indices.
// Every FIFO must keep its own order, and data may never end up in another FIFO.
////////////////////////////////////////////////////////////
module fifo_array_test(output logic done);

localparam int NUM_FIFOS = 4;

logic clk = 0;
always #5 clk = !clk;

logic rst = 0;
logic try_push = 0;
logic [1:0] push_idx = 0;
logic [15:0] push_v = 0;
logic push_success;

logic try_pop = 0;
logic [1:0] pop_idx = 0;
logic pop_success;
logic [15:0] pop_v;

TestFIFOArray dut(
    .clk,
    .rst,
    .push_success,
    .try_push,
    .push_idx,
    .push_v,
    .pop_success,
    .pop_v,
    .try_pop,
    .pop_idx
);

// Scoreboard
logic [15:0] ref_q[NUM_FIFOS][$];
int unsigned push_cnt[NUM_FIFOS];
int unsigned pop_cnt[NUM_FIFOS];
int unsigned refused_push_cnt = 0;

// pop_success and pop_v arrive one cycle after try_pop
logic [1:0] pop_idx_d;
logic [15:0] expected;

always @(posedge clk) begin
    pop_idx_d <= pop_idx;
    if (rst) begin
        for (int i = 0; i < NUM_FIFOS; i++) begin
            ref_q[i].delete();
            push_cnt[i] = 0;
            pop_cnt[i] = 0;
        end
    end else begin
        if (try_push && !push_success) refused_push_cnt++;
        if (push_success) begin
            if (!try_push) $fatal(1, "push_success without try_push at time %t", $time);
            ref_q[push_idx].push_back(push_v);
            push_cnt[push_idx]++;
        end
        if (pop_success) begin
            if (ref_q[pop_idx_d].size() == 0) begin
                $fatal(1, "POP from empty FIFO %0d at time %t", pop_idx_d, $time);
            end
            expected = ref_q[pop_idx_d].pop_front();
            if (expected !== pop_v) begin
                $fatal(1, "FIFO %0d popped %0d, expected %0d at time %t", pop_idx_d, pop_v, expected, $time);
            end
            pop_cnt[pop_idx_d]++;
        end
    end
end

task automatic random_cycles(int cycles, int push_percent, int pop_percent);
    repeat (cycles) begin
        @(negedge clk);
        try_push = ($urandom_range(99) < push_percent);
        push_idx = 2'($urandom_range(NUM_FIFOS - 1));
        push_v = 16'($urandom);
        try_pop = ($urandom_range(99) < pop_percent);
        pop_idx = 2'($urandom_range(NUM_FIFOS - 1));
    end
endtask

initial begin
    done = 0;
    @(negedge clk);
    rst = 1;
    @(negedge clk);
    rst = 0;

    random_cycles(2000, 80, 20); // Fills up, exercises may_push backpressure per FIFO
    random_cycles(2000, 20, 80); // Drains, exercises may_pop per FIFO
    random_cycles(2000, 50, 50);

    // Only FIFO 2 gets pushed, the others must stay as they are
    repeat (100) begin
        @(negedge clk);
        try_push = 1;
        push_idx = 2;
        push_v = 16'($urandom);
        try_pop = 0;
    end

    // Drain everything. Pushes need a few cycles to arrive, so keep popping for a while
    try_push = 0;
    for (int i = 0; i < 400; i++) begin
        @(negedge clk);
        try_pop = 1;
        pop_idx = 2'(i % NUM_FIFOS);
    end
    @(negedge clk);
    try_pop = 0;
    repeat (4) @(negedge clk);

    for (int i = 0; i < NUM_FIFOS; i++) begin
        if (ref_q[i].size() != 0) $fatal(1, "FIFO %0d still has %0d elements after draining", i, ref_q[i].size());
        if (push_cnt[i] == 0) $fatal(1, "FIFO %0d was never pushed to", i);
        if (push_cnt[i] != pop_cnt[i]) $fatal(1, "FIFO %0d: %0d pushes but %0d pops", i, push_cnt[i], pop_cnt[i]);
        $display("  FIFO %0d: %0d pushes, %0d pops", i, push_cnt[i], pop_cnt[i]);
    end
    if (refused_push_cnt == 0) $fatal(1, "may_push backpressure was never exercised");
    $display("fifo_array_test passed (%0d pushes refused by backpressure)", refused_push_cnt);
    done = 1;
end

endmodule

////////////////////////////////////////////////////////////
// TestAccumulatorGrid: a 2x3 grid of accumulators.
// Runtime [row][col] indexing, compile-time row with runtime column, and compile-time reads of all elements
////////////////////////////////////////////////////////////
module accumulator_grid_test(output logic done);

localparam int ROWS = 2;
localparam int COLS = 3;

logic clk = 0;
always #5 clk = !clk;

logic clear = 0;
logic add = 0;
logic [0:0] row = 0;
logic [1:0] col = 0;
logic [7:0] v = 0;
logic add_to_last_row = 0;
logic [1:0] last_row_col = 0;
logic [7:0] last_row_v = 0;
logic [0:0] read_row = 0;
logic [1:0] read_col = 0;
logic [31:0] total;
logic [31:0] all_totals[0:ROWS-1][0:COLS-1];

TestAccumulatorGrid dut(
    .clk,
    .clear,
    .add,
    .row,
    .col,
    .v,
    .add_to_last_row,
    .last_row_col,
    .last_row_v,
    .read_row,
    .read_col,
    .total,
    .all_totals
);

// Reference model. Accumulators start at 0 through their `initial` value
logic [31:0] model[ROWS][COLS] = '{default: 0};
int unsigned num_checks = 0;

always @(posedge clk) begin
    if (add) model[row][col] = model[row][col] + 32'(v);
    if (add_to_last_row) model[ROWS-1][last_row_col] = model[ROWS-1][last_row_col] + 32'(last_row_v);
    if (clear) begin
        for (int r = 0; r < ROWS; r++) for (int c = 0; c < COLS; c++) model[r][c] = 0;
    end
end

task automatic check();
    for (int r = 0; r < ROWS; r++) begin
        for (int c = 0; c < COLS; c++) begin
            if (all_totals[r][c] !== model[r][c]) begin
                $fatal(1, "all_totals[%0d][%0d] is %0d, expected %0d at time %t", r, c, all_totals[r][c], model[r][c], $time);
            end
        end
    end
    if (total !== model[read_row][read_col]) begin
        $fatal(1, "total (accs[%0d][%0d]) is %0d, expected %0d at time %t", read_row, read_col, total, model[read_row][read_col], $time);
    end
    num_checks++;
endtask

initial begin
    done = 0;
    repeat (5000) begin
        @(negedge clk);
        check();
        clear = ($urandom_range(199) == 0);
        add = ($urandom_range(1) == 0);
        row = 1'($urandom_range(ROWS - 1));
        col = 2'($urandom_range(COLS - 1));
        v = 8'($urandom);
        add_to_last_row = ($urandom_range(2) == 0);
        last_row_col = 2'($urandom_range(COLS - 1));
        last_row_v = 8'($urandom);
        // Two actions writing to the same accumulator in the same cycle is not allowed
        if (add && row == 1'(ROWS - 1) && col == last_row_col) add_to_last_row = 0;
        read_row = 1'($urandom_range(ROWS - 1));
        read_col = 2'($urandom_range(COLS - 1));
    end
    @(negedge clk);
    check();
    $display("accumulator_grid_test passed (%0d checks)", num_checks);
    done = 1;
end

endmodule

////////////////////////////////////////////////////////////
// TestTriggerArray: every element echoes what it was sent 2 cycles later, through its trigger.
////////////////////////////////////////////////////////////
module trigger_array_test(output logic done);

localparam int NUM = 4;

logic clk = 0;
always #5 clk = !clk;

logic send = 0;
logic [1:0] idx = 0;
logic [7:0] v = 0;
logic send_to_3 = 0;
logic [7:0] v3 = 0;
logic [3:0] echoed;
logic [7:0] echoed_v[0:NUM-1];

TestTriggerArray dut(
    .clk,
    .send,
    .idx,
    .v,
    .send_to_3,
    .v3,
    .echoed,
    .echoed_v
);

// Reference model: a 2 stage pipeline per element
logic [NUM-1:0] exp_valid[2] = '{default: 0};
logic [7:0] exp_v[2][NUM];
int unsigned num_echoes = 0;

always @(posedge clk) begin
    exp_valid[1] = exp_valid[0];
    exp_v[1] = exp_v[0];
    exp_valid[0] = 0;
    if (send) begin
        exp_valid[0][idx] = 1;
        exp_v[0][idx] = v;
    end
    if (send_to_3) begin
        exp_valid[0][3] = 1;
        exp_v[0][3] = v3;
    end
end

task automatic check();
    if (echoed !== exp_valid[1]) begin
        $fatal(1, "echoed is %b, expected %b at time %t", echoed, exp_valid[1], $time);
    end
    for (int i = 0; i < NUM; i++) begin
        if (echoed[i]) begin
            if (echoed_v[i] !== exp_v[1][i]) begin
                $fatal(1, "echoed_v[%0d] is %0d, expected %0d at time %t", i, echoed_v[i], exp_v[1][i], $time);
            end
            num_echoes++;
        end
    end
endtask

initial begin
    done = 0;
    repeat (5000) begin
        @(negedge clk);
        check();
        send = ($urandom_range(1) == 0);
        idx = 2'($urandom_range(NUM - 1));
        v = 8'($urandom);
        send_to_3 = ($urandom_range(2) == 0);
        v3 = 8'($urandom);
        // Two calls to the same element in the same cycle is not allowed
        if (send && idx == 3) send_to_3 = 0;
    end
    @(negedge clk);
    send = 0;
    send_to_3 = 0;
    repeat (3) begin
        check();
        @(negedge clk);
    end
    if (num_echoes == 0) $fatal(1, "No echoes were seen");
    $display("trigger_array_test passed (%0d echoes)", num_echoes);
    done = 1;
end

endmodule

////////////////////////////////////////////////////////////
// TestSplitCascade: 4 separate AddOne submodules in a chain. The output is i + 4, 4 cycles later
////////////////////////////////////////////////////////////
module split_cascade_test(output logic done);

localparam int STAGES = 4;

logic clk = 0;
always #5 clk = !clk;

logic [7:0] i = 0;
logic [7:0] o;

TestSplitCascade dut(
    .clk,
    .i,
    .o
);

// Reference model: the inputs of the last STAGES cycles
logic [7:0] history[STAGES];
int unsigned cycle = 0;
int unsigned num_checks = 0;

always @(posedge clk) begin
    for (int s = STAGES - 1; s > 0; s--) history[s] = history[s - 1];
    history[0] = i;
    cycle++;
end

initial begin
    done = 0;
    repeat (2000) begin
        @(negedge clk);
        if (cycle >= STAGES) begin
            if (o !== 8'(history[STAGES - 1] + STAGES)) begin
                $fatal(1, "o is %0d, expected %0d + %0d at time %t", o, history[STAGES - 1], STAGES, $time);
            end
            num_checks++;
        end
        i = 8'($urandom);
    end
    $display("split_cascade_test passed (%0d checks)", num_checks);
    done = 1;
end

endmodule

////////////////////////////////////////////////////////////
// TestSplitAccumulatorRows: both rows are separate submodule arrays, and may be added to in the same cycle
////////////////////////////////////////////////////////////
module split_accumulator_rows_test(output logic done);

localparam int ROWS = 2;
localparam int COLS = 3;

logic clk = 0;
always #5 clk = !clk;

logic add_row_0 = 0;
logic [1:0] col_0 = 0;
logic [7:0] v_0 = 0;
logic add_row_1 = 0;
logic [1:0] col_1 = 0;
logic [7:0] v_1 = 0;
logic [31:0] all_totals[0:ROWS-1][0:COLS-1];

TestSplitAccumulatorRows dut(
    .clk,
    .add_row_0,
    .col_0,
    .v_0,
    .add_row_1,
    .col_1,
    .v_1,
    .all_totals
);

logic [31:0] model[ROWS][COLS] = '{default: 0};
int unsigned num_checks = 0;

always @(posedge clk) begin
    if (add_row_0) model[0][col_0] = model[0][col_0] + 32'(v_0);
    if (add_row_1) model[1][col_1] = model[1][col_1] + 32'(v_1);
end

task automatic check();
    for (int r = 0; r < ROWS; r++) begin
        for (int c = 0; c < COLS; c++) begin
            if (all_totals[r][c] !== model[r][c]) begin
                $fatal(1, "all_totals[%0d][%0d] is %0d, expected %0d at time %t", r, c, all_totals[r][c], model[r][c], $time);
            end
        end
    end
    num_checks++;
endtask

initial begin
    done = 0;
    repeat (5000) begin
        @(negedge clk);
        check();
        add_row_0 = ($urandom_range(1) == 0);
        col_0 = 2'($urandom_range(COLS - 1));
        v_0 = 8'($urandom);
        add_row_1 = ($urandom_range(1) == 0);
        col_1 = 2'($urandom_range(COLS - 1));
        v_1 = 8'($urandom);
    end
    @(negedge clk);
    check();
    $display("split_accumulator_rows_test passed (%0d checks)", num_checks);
    done = 1;
end

endmodule

////////////////////////////////////////////////////////////
// Top
////////////////////////////////////////////////////////////
module submodule_array_tests;

logic fifo_array_done, accumulator_grid_done, trigger_array_done;
logic split_cascade_done, split_accumulator_rows_done;

fifo_array_test fifo_array(.done(fifo_array_done));
accumulator_grid_test accumulator_grid(.done(accumulator_grid_done));
trigger_array_test trigger_array(.done(trigger_array_done));
split_cascade_test split_cascade(.done(split_cascade_done));
split_accumulator_rows_test split_accumulator_rows(.done(split_accumulator_rows_done));

initial begin
    $display("Starting submodule array tests");
    wait (fifo_array_done && accumulator_grid_done && trigger_array_done);
    wait (split_cascade_done && split_accumulator_rows_done);
    $display("All tests passed");
    $finish();
end

initial begin
    #1000000;
    $fatal(1, "Timeout");
end

endmodule
