`timescale 1ns / 1ps

module tb;

  initial begin
    $dumpfile("fsm.vcd");
    $dumpvars(0, dut);
    #10000;
    $finish;
  end

  // --------------------------------------------------------------------------
  // Clock and Reset

  logic clk;
  logic rstn;

  initial begin
    clk = 0;
    forever #5 clk = ~clk;
  end

  initial begin
    rstn = 1'b0;
    #100;
    rstn = 1'b1;
  end

  // --------------------------------------------------------------------------

  // verilator lint_off WIDTHTRUNC
  fsm dut (.*);
  // verilator lint_on WIDTHTRUNC

  // --------------------------------------------------------------------------

  logic in0, in1, out;

  initial begin
    in0 = 0;
    forever #13 in0 = ~in0;
  end

  initial begin
    in1 = 0;
    forever #17 in1 = ~in1;
  end

endmodule
