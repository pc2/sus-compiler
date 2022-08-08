
/*
LEVEL 1 (Data Loss): 
	- No data loss
	- No new invalid data
	
	-> every read must correspond to data destruction
	-> data destruction must happen together with a read
*/

/*
LEVEL 2 (Temporal safety):
	- Operations may only happen on data of the same 'time slice' within a stream
*/

/*
LEVEL 3 (Channel splits):
	- Channel splits
	- Channel merges
*/

clocked module basicPipe: 
	in integer a'x,
	in integer b'x,
	in integer c'x+1 -> 
	out integer result'x+2 {
	
	reg ab'x+1 = D(a+b);
	reg result'x+2 = ab * c;
}

clocked module skidBuffer 
    in bool validIn'(a+1),
	in T dataIn'(a+1),
    out bool ready'(a+1) -> 
    in bool readyO'a,
	out bool validO,
    out T dataO:

reg readyOD'(a+1) = D(readyO);
ready = readyOD;

cond reg dataD'(a+2 if !readyOD'(a+1));
if(!readyOD) {
	dataD = D(dataIn);
}

dataO = readyOD'(a+1) ? dataIn'(a+1) : dataD'(a+2 & !readyOD'(a+1)); // Is this ok?

endmodule

clocked module skidBuffer 
    in bool dataIn'(a+1),
    out bool ready'(a+1) -> 
    in bool readyO'a, 
    out bool dataO:

reg readyOD'(a+1) = D(readyO);
ready = readyOD;

cond reg dataD'(a+2 if !readyOD'(a+1));
if(!readyOD) {
	dataD = D(dataIn);
}

dataO = readyOD'(a+1) ? dataIn'(a+1) : dataD'(a+2 & !readyOD'(a+1)); // Is this ok?

endmodule



module skid_buffer #(parameter WIDTH = 8) (
    input clk,
    input rst,
    input[WIDTH-1:0] i_data,
    input i_valid,
    output o_ready,

    output[WIDTH-1:0] o_data,
    output o_valid,
    input i_ready
)

reg ready_rg;
reg[WIDTH-1:0] data_rg;
reg bypass_rg;

always @(posedge clk) begin
    if(rst) begin
        ready_rg <= 0;
        data_rg <= 0;
        bypass_rg <= 1;
    end else begin
        if(bypass_rg) begin
            if(!i_ready && i_valid && ready_rg) begin
                ready_rg <= 0;
                data_rg <= i_data;
                bypass_rg <= 0;
            end else begin
				ready_rg <= 1;
			end
        end else begin
            if(i_ready) begin
                ready_rg <= 1;
                bypass_rg <= 1;
            end
        end
    end
end

assign o_ready = ready_rg;
assign o_data = bypass_rg ? i_data : data_rg;
assign o_valid = bypass_rg ? (i_valid && ready_rg) : 1;

endmodule
