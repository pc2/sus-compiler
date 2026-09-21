// Testbench for module MultiReduction #(T: type bool #()[32], NEUTRAL_ELEMENT: 32'b00000000000000000000000000000000, OPERATOR_LATENCY: 7, DATA_IN_LATENCY: 2, MAX_DATAS: 64)
module MultiReduction_T_type_bool_32_NEUTRAL_ELEMENT_32_b00000000000000000000000000000000_OPERATOR_LATENCY_7_DATA_IN_LATENCY_2_MAX_DATAS_64_tb;
	// Clocks
	logic clk = 0;
	initial #0 forever #5 clk = !clk;

	// Ports
	// {clk} output bool #() reduction_operator'2
	wire reduction_operator;
	// {clk} output bool #()[32] op_left'2
	wire[31:0] op_left;
	// {clk} output bool #()[32] op_right'2
	wire[31:0] op_right;
	// {clk} input bool #()[32] op_result'9
	logic[31:0] op_result;
	// {clk} output bool #() may_push'0
	wire may_push;
	// {clk} input bool #() push'0
	logic push;
	// {clk} input bool #()[32] value_to_reduce'2
	logic[31:0] value_to_reduce;
	// {clk} input bool #() last_in_wave'0
	logic last_in_wave;
	// {clk} input bool #() finish_reduction'0
	logic finish_reduction;
	// {clk} output bool #()[32] reduced_data'9
	wire[31:0] reduced_data;
	// {clk} input bool #() rst'0
	logic rst;
	
    localparam int DATA_IN_LATENCY = 2;
    localparam int OPERATOR_LATENCY = 7;
    localparam int MAX_DATAS = 64;

	// DUT
	MultiReduction_T_type_bool_32_NEUTRAL_ELEMENT_32_b00000000000000000000000000000000_OPERATOR_LATENCY_7_DATA_IN_LATENCY_2_MAX_DATAS_64 dut(
		.clk(clk),
		.reduction_operator(reduction_operator),
		.op_left(op_left),
		.op_right(op_right),
		.op_result(op_result),
		.may_push(may_push),
		.push(push),
		.value_to_reduce(value_to_reduce),
		.last_in_wave(last_in_wave),
		.finish_reduction(finish_reduction),
		.reduced_data(reduced_data),
		.rst(rst)
	);

    logic[31:0] operator_pipeline_stages[OPERATOR_LATENCY];
    assign op_result = operator_pipeline_stages[OPERATOR_LATENCY-1];
    always @(posedge clk) begin
        if(reduction_operator) begin
            operator_pipeline_stages[0] <= op_left + op_right;
        end
        for(int i = 1; i < OPERATOR_LATENCY; i++) begin
            operator_pipeline_stages[i] <= operator_pipeline_stages[i-1];
        end
    end

    typedef struct {
        logic is_used;
        logic[31:0] expected_reduced_data;
    } ExpectedResult;
    logic[31:0] expected_result_maker;
    ExpectedResult expected_results_pipeline[OPERATOR_LATENCY+DATA_IN_LATENCY];
    always @(posedge clk) begin
        expected_results_pipeline[0].is_used <= push & finish_reduction;
        if(push & finish_reduction) begin
            expected_results_pipeline[0].expected_reduced_data <= expected_result_maker;
        end
        for(int i = 1; i < OPERATOR_LATENCY+DATA_IN_LATENCY; i++) begin
            expected_results_pipeline[i] <= expected_results_pipeline[i-1];
        end
    end
    ExpectedResult expected_result;
    assign expected_result = expected_results_pipeline[OPERATOR_LATENCY+DATA_IN_LATENCY-1];
    always @(posedge clk) begin
        if(expected_result.is_used) begin
            if(expected_result.expected_reduced_data == reduced_data) begin
                $display("Correct value. Expected %d and found %d",
                    expected_result.expected_reduced_data, reduced_data
                );
            end else begin
                $fatal("Incorrect value. Expected %d but found %d",
                    expected_result.expected_reduced_data, reduced_data
                );
            end
        end
    end

    int current_reduction_idx = 0;
    logic[31:0] expected_reductions[MAX_DATAS];
    initial begin
        for(int i = 0; i < 64; i++) begin
            expected_reductions[i] = 0;
        end
    end

    initial push = 0;
	logic[31:0] _value_to_reduce_Pre2;
	logic[31:0] _value_to_reduce_Pre1;
	always_ff @(posedge clk) begin _value_to_reduce_Pre1 <= _value_to_reduce_Pre2; end
    always_ff @(posedge clk) begin value_to_reduce <= _value_to_reduce_Pre1; end
	
	
	logic push_pending = 0;
	assign push = may_push && push_pending;
	int cur_arbitrary_value = 100;
    task push_d(logic last_in_wave_v, logic finish_reduction_v);
        automatic logic[31:0] value_to_push = cur_arbitrary_value++;
        
        _value_to_reduce_Pre2 <= value_to_push;
        last_in_wave <= last_in_wave_v;
        finish_reduction <= finish_reduction_v;
        push_pending <= 1;
        
        expected_reductions[current_reduction_idx] += value_to_push;
        if(finish_reduction_v) begin
            expected_result_maker <= expected_reductions[current_reduction_idx];
            expected_reductions[current_reduction_idx] = 0;
        end
        current_reduction_idx++;
		if(last_in_wave_v) begin
			current_reduction_idx = 0;
		end

        do begin
            @(posedge clk);
        end while(!push);

        push_pending <= 0;
    endtask

    task push_wave(int wave_length, int num_reductions);
        for(int reduction_step = 0; reduction_step < num_reductions; reduction_step++) begin
            for(int wave_elem = 0; wave_elem < wave_length; wave_elem++) begin
                push_d(wave_elem == wave_length - 1, reduction_step == num_reductions - 1);
            end
        end
    endtask

	initial begin
        rst <= 1;
        repeat(10) @(posedge clk);

        rst <= 0;
        repeat(10) @(posedge clk);
        
        push_wave(3, 2);
        repeat(10) @(posedge clk);
        repeat(10) @(posedge clk);
        
        for(int wave_length = 1; wave_length <= MAX_DATAS; wave_length++) begin
            push_wave(wave_length, 1);
            push_wave(wave_length, 1);
            push_wave(wave_length, 1);
            push_wave(wave_length, 2);
            push_wave(wave_length, 2);
            push_wave(wave_length, 2);
            push_wave(wave_length, 5);
            push_wave(wave_length, 5);
            repeat(10) @(posedge clk);
        end

        $finish();
	end
endmodule // MultiReduction_T_type_bool_32_NEUTRAL_ELEMENT_32_b00000000000000000000000000000000_OPERATOR_LATENCY_7_DATA_IN_LATENCY_2_MAX_DATAS_64_tb

