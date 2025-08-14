
// Data is returned 2 cycles after. 
module RawROM #(parameter int WIDTH = 0, parameter int DEPTH = 0, parameter string MEMORY_FILE = "none") (
    input logic clk,
    input logic read,
    input logic[$clog2(DEPTH)-1:0] index,
    output logic[WIDTH-1:0] output_bits
);
    // xpm_memory_sprom: Single Port ROM
    // Xilinx Parameterized Macro, version 2024.2
    
    xpm_memory_sprom #(
       .ADDR_WIDTH_A($clog2(DEPTH)),              // DECIMAL
       .AUTO_SLEEP_TIME(0),           // DECIMAL
       .CASCADE_HEIGHT(0),            // DECIMAL
       .ECC_BIT_RANGE("7:0"),         // String
       .ECC_MODE("no_ecc"),           // String
       .ECC_TYPE("none"),             // String
       .IGNORE_INIT_SYNTH(0),         // DECIMAL
       .MEMORY_INIT_FILE(MEMORY_FILE),     // String
       .MEMORY_INIT_PARAM("0"),       // String
       .MEMORY_OPTIMIZATION("true"),  // String
       .MEMORY_PRIMITIVE("auto"),     // String
       .MEMORY_SIZE(DEPTH * WIDTH),            // DECIMAL
       .MESSAGE_CONTROL(0),           // DECIMAL
       .RAM_DECOMP("auto"),           // String
       .READ_DATA_WIDTH_A(WIDTH),        // DECIMAL
       .READ_LATENCY_A(2),            // DECIMAL
       .READ_RESET_VALUE_A("0"),      // String
       .RST_MODE_A("SYNC"),           // String
       .SIM_ASSERT_CHK(0),            // DECIMAL; 0=disable simulation messages, 1=enable simulation messages
       .USE_MEM_INIT(1),              // DECIMAL
       .USE_MEM_INIT_MMI(0),          // DECIMAL
       .WAKEUP_TIME("disable_sleep")  // String
    ) xpm_memory_sprom_inst (
       .dbiterra(),             // 1-bit output: Leave open.
       .douta(output_bits),                   // READ_DATA_WIDTH_A-bit output: Data output for port A read operations.
       .sbiterra(),             // 1-bit output: Leave open.
       .addra(index),                   // ADDR_WIDTH_A-bit input: Address for port A read operations.
       .clka(clk),                     // 1-bit input: Clock signal for port A.
       .ena(read),                       // 1-bit input: Memory enable signal for port A. Must be high on clock
                                        // cycles when read operations are initiated. Pipelined internally.
    
       .injectdbiterra(1'b0), // 1-bit input: Do not change from the provided value.
       .injectsbiterra(1'b0), // 1-bit input: Do not change from the provided value.
       .regcea(1'b1),                 // 1-bit input: Do not change from the provided value.
       .rsta(1'b0),                     // 1-bit input: Reset signal for the final port A output register stage.
                                        // Synchronously resets output port douta to the value specified by
                                        // parameter READ_RESET_VALUE_A.
    
       .sleep(1'b0)                    // 1-bit input: sleep signal to enable the dynamic power saving feature.
    );
    
    // End of xpm_memory_sprom_inst instantiation
endmodule

module RawRAM #(parameter int WIDTH = 8, parameter int DEPTH = 32, parameter string MEMORY_FILE = "none")(
    input logic clk,
    input logic write,
    input logic[$clog2(DEPTH)-1:0] addra,
    input logic[WIDTH-1:0] dina,
    input logic read,
    input logic[$clog2(DEPTH)-1:0] addrb,
    output logic[WIDTH-1:0] doutb
);

// xpm_memory_sdpram: Simple Dual Port RAM
// Xilinx Parameterized Macro, version 2025.1

xpm_memory_sdpram #(
   .ADDR_WIDTH_A($clog2(DEPTH)),               // DECIMAL
   .ADDR_WIDTH_B($clog2(DEPTH)),               // DECIMAL
   .AUTO_SLEEP_TIME(0),            // DECIMAL
   .BYTE_WRITE_WIDTH_A(WIDTH),        // DECIMAL
   .CASCADE_HEIGHT(0),             // DECIMAL
   .CLOCKING_MODE("common_clock"), // String
   .ECC_BIT_RANGE("7:0"),          // String
   .ECC_MODE("no_ecc"),            // String
   .ECC_TYPE("none"),              // String
   .IGNORE_INIT_SYNTH(0),          // DECIMAL
   .MEMORY_INIT_FILE(MEMORY_FILE),      // String
   .MEMORY_INIT_PARAM("0"),        // String
   .MEMORY_OPTIMIZATION("true"),   // String
   .MEMORY_PRIMITIVE("auto"),      // String
   .MEMORY_SIZE(WIDTH * DEPTH),             // DECIMAL
   .MESSAGE_CONTROL(0),            // DECIMAL
   .RAM_DECOMP("auto"),            // String
   .READ_DATA_WIDTH_B(WIDTH),         // DECIMAL
   .READ_LATENCY_B(2),             // DECIMAL
   .READ_RESET_VALUE_B("0"),       // String
   .RST_MODE_A("SYNC"),            // String
   .RST_MODE_B("SYNC"),            // String
   .SIM_ASSERT_CHK(0),             // DECIMAL; 0=disable simulation messages, 1=enable simulation messages
   .USE_EMBEDDED_CONSTRAINT(0),    // DECIMAL
   .USE_MEM_INIT(1),               // DECIMAL
   .USE_MEM_INIT_MMI(0),           // DECIMAL
   .WAKEUP_TIME("disable_sleep"),  // String
   .WRITE_DATA_WIDTH_A(WIDTH),        // DECIMAL
   .WRITE_MODE_B("no_change"),     // String
   .WRITE_PROTECT(1)               // DECIMAL
)
xpm_memory_sdpram_inst (
   .dbiterrb(),             // 1-bit output: Status signal to indicate double bit error occurrence on the data output of port B.
   .doutb(doutb),                   // READ_DATA_WIDTH_B-bit output: Data output for port B read operations.
   .sbiterrb(),             // 1-bit output: Status signal to indicate single bit error occurrence on the data output of port B.
   .addra(addra),                   // ADDR_WIDTH_A-bit input: Address for port A write operations.
   .addrb(addrb),                   // ADDR_WIDTH_B-bit input: Address for port B read operations.
   .clka(clk),                     // 1-bit input: Clock signal for port A. Also clocks port B when parameter CLOCKING_MODE is "common_clock".
   .clkb(clk),                     // 1-bit input: Clock signal for port B when parameter CLOCKING_MODE is "independent_clock". Unused when
                                    // parameter CLOCKING_MODE is "common_clock".

   .dina(dina),                     // WRITE_DATA_WIDTH_A-bit input: Data input for port A write operations.
   .ena(write),                       // 1-bit input: Memory enable signal for port A. Must be high on clock cycles when write operations are
                                    // initiated. Pipelined internally.

   .enb(read),                       // 1-bit input: Memory enable signal for port B. Must be high on clock cycles when read operations are
                                    // initiated. Pipelined internally.

   .injectdbiterra(1'b0), // 1-bit input: Controls double bit error injection on input data when ECC enabled (Error injection capability
                                    // is not available in "decode_only" mode).

   .injectsbiterra(1'b0), // 1-bit input: Controls single bit error injection on input data when ECC enabled (Error injection capability
                                    // is not available in "decode_only" mode).

   .regceb(1'b1),                 // 1-bit input: Clock Enable for the last register stage on the output data path.
   .rstb(1'b0),                     // 1-bit input: Reset signal for the final port B output register stage. Synchronously resets output port
                                    // doutb to the value specified by parameter READ_RESET_VALUE_B.

   .sleep(1'b0),                   // 1-bit input: sleep signal to enable the dynamic power saving feature.
   .wea()                        // WRITE_DATA_WIDTH_A/BYTE_WRITE_WIDTH_A-bit input: Write enable vector for port A input data port dina. 1 bit
                                    // wide when word-wide writes are used. In byte-wide write configurations, each bit controls the writing one
                                    // byte of dina to address addra. For example, to synchronously write only bits [15-8] of dina when
                                    // WRITE_DATA_WIDTH_A is 32, wea would be 4'b0010.

);

// End of xpm_memory_sdpram_inst instantiation


endmodule


module RawFIFO #(parameter int WIDTH = 8, parameter int DEPTH = 32, parameter int MAY_PUSH_LATENCY = 5, parameter int EXTRA_IN_FLIGHT = 0)(
    input logic clk,
    input logic rst,
    output logic may_push,
    input logic push,
    input logic[WIDTH-1:0] push_data,
    output logic may_pop,
    input logic pop,
    output logic[WIDTH-1:0] pop_data
);

// xpm_fifo_sync: Synchronous FIFO
// Xilinx Parameterized Macro, version 2025.1

logic empty;
assign may_pop = !empty;
logic prog_full;
assign may_push = !prog_full;

// Minus one because READ_MODE == "std", if it were fallthrough this wouldn't be
localparam PROG_FULL_THRESH = DEPTH - MAY_PUSH_LATENCY - EXTRA_IN_FLIGHT - 1;

xpm_fifo_sync #(
   .CASCADE_HEIGHT(0),            // DECIMAL
   .DOUT_RESET_VALUE("0"),        // String
   .ECC_MODE("no_ecc"),           // String
   .EN_SIM_ASSERT_ERR("error"), // String
   .FIFO_MEMORY_TYPE("auto"),     // String
   .FIFO_READ_LATENCY(2),         // DECIMAL
   .FIFO_WRITE_DEPTH(DEPTH),       // DECIMAL
   .FULL_RESET_VALUE(0),          // DECIMAL
   .PROG_EMPTY_THRESH(0),        // DECIMAL
   .PROG_FULL_THRESH(PROG_FULL_THRESH),         // DECIMAL
   .RD_DATA_COUNT_WIDTH(1),       // DECIMAL
   .READ_DATA_WIDTH(WIDTH),          // DECIMAL
   .READ_MODE("std"),             // String
   .SIM_ASSERT_CHK(1),            // DECIMAL; 0=disable simulation messages, 1=enable simulation messages
   .USE_ADV_FEATURES("0002"),     // String - Only enable prog_full
   .WAKEUP_TIME(0),               // DECIMAL
   .WRITE_DATA_WIDTH(WIDTH),         // DECIMAL
   .WR_DATA_COUNT_WIDTH(1)        // DECIMAL
)
xpm_fifo_sync_inst (
   .almost_empty(),   // 1-bit output: Almost Empty : When asserted, this signal indicates that only one more read can be performed
                                  // before the FIFO goes to empty.

   .almost_full(),     // 1-bit output: Almost Full: When asserted, this signal indicates that only one more write can be performed
                                  // before the FIFO is full.

   .data_valid(),       // 1-bit output: Read Data Valid: When asserted, this signal indicates that valid data is available on the
                                  // output bus (dout).

   .dbiterr(),             // 1-bit output: Double Bit Error: Indicates that the ECC decoder detected a double-bit error and data in the
                                  // FIFO core is corrupted.

   .dout(pop_data),                   // READ_DATA_WIDTH-bit output: Read Data: The output data bus is driven when reading the FIFO.
   .empty(empty),                 // 1-bit output: Empty Flag: When asserted, this signal indicates that the FIFO is empty. Read requests are
                                  // ignored when the FIFO is empty, initiating a read while empty is not destructive to the FIFO.

   .full(),                   // 1-bit output: Full Flag: When asserted, this signal indicates that the FIFO is full. Write requests are
                                  // ignored when the FIFO is full, initiating a write when the FIFO is full is not destructive to the contents of
                                  // the FIFO.

   .overflow(),           // 1-bit output: Overflow: This signal indicates that a write request (wren) during the prior clock cycle was
                                  // rejected, because the FIFO is full. Overflowing the FIFO is not destructive to the contents of the FIFO.

   .prog_empty(),       // 1-bit output: Programmable Empty: This signal is asserted when the number of words in the FIFO is less than
                                  // or equal to the programmable empty threshold value. It is de-asserted when the number of words in the FIFO
                                  // exceeds the programmable empty threshold value.

   .prog_full(prog_full),         // 1-bit output: Programmable Full: This signal is asserted when the number of words in the FIFO is greater than
                                  // or equal to the programmable full threshold value. It is de-asserted when the number of words in the FIFO is
                                  // less than the programmable full threshold value.

   .rd_data_count(), // RD_DATA_COUNT_WIDTH-bit output: Read Data Count: This bus indicates the number of words read from the FIFO.
   .rd_rst_busy(),     // 1-bit output: Read Reset Busy: Active-High indicator that the FIFO read domain is currently in a reset state.
   .sbiterr(),             // 1-bit output: Single Bit Error: Indicates that the ECC decoder detected and fixed a single-bit error.
   .underflow(),         // 1-bit output: Underflow: Indicates that the read request (rd_en) during the previous clock cycle was rejected
                                  // because the FIFO is empty. Under flowing the FIFO is not destructive to the FIFO.

   .wr_ack(),               // 1-bit output: Write Acknowledge: This signal indicates that a write request (wr_en) during the prior clock
                                  // cycle is succeeded.

   .wr_data_count(), // WR_DATA_COUNT_WIDTH-bit output: Write Data Count: This bus indicates the number of words written into the
                                  // FIFO.

   .wr_rst_busy(),     // 1-bit output: Write Reset Busy: Active-High indicator that the FIFO write domain is currently in a reset
                                  // state.

   .din(push_data),                     // WRITE_DATA_WIDTH-bit input: Write Data: The input data bus used when writing the FIFO.
   .injectdbiterr(), // 1-bit input: Double Bit Error Injection: Injects a double bit error if the ECC feature is used on block RAMs
                                  // or UltraRAM macros.

   .injectsbiterr(), // 1-bit input: Single Bit Error Injection: Injects a single bit error if the ECC feature is used on block RAMs
                                  // or UltraRAM macros.

   .rd_en(pop),                 // 1-bit input: Read Enable: If the FIFO is not empty, asserting this signal causes data (on dout) to be read
                                  // from the FIFO. Must be held active-low when rd_rst_busy is active high.

   .rst(rst),                     // 1-bit input: Reset: Must be synchronous to wr_clk. The clock(s) can be unstable at the time of applying
                                  // reset, but reset must be released only after the clock(s) is/are stable.

   .sleep(1'b0),                 // 1-bit input: Dynamic power saving- If sleep is High, the memory/fifo block is in power saving mode.
   .wr_clk(clk),               // 1-bit input: Write clock: Used for write operation. wr_clk must be a free running clock.
   .wr_en(push)                  // 1-bit input: Write Enable: If the FIFO is not full, asserting this signal causes data (on din) to be written
                                  // to the FIFO Must be held active-low when rst or wr_rst_busy or rd_rst_busy is active high

);

// End of xpm_fifo_sync_inst instantiation
endmodule
