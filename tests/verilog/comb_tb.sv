`timescale 1ns / 1ps

module tb;

  initial begin
    $dumpfile("comb.vcd");
    $dumpvars(0, dut);
    #50000;
    $finish;
  end

  // --------------------------------------------------------------------------

  // Clock and Reset
  logic clk;
  logic rstn;
  logic in0;
  logic in1;
  logic out0;
  logic out1;

  // verilator lint_off WIDTHTRUNC
  comb dut (.*);
  // verilator lint_on WIDTHTRUNC

  // --------------------------------------------------------------------------

  // Clock generation
  initial begin
    clk = 0;
    forever #5 clk = ~clk;
  end

  // Reset task
  task reset_dut;
    begin
      rstn = 1'b0;
      #100;
      rstn = 1'b1;
    end
  endtask

  // --------------------------------------------------------------------------

  initial begin
    #10000;
    $finish;
  end

  // Stimulus
  initial begin
    // Dump waveform
    $dumpfile("comb.vcd");
    $dumpvars(0, tb_comb);

    // Initialize inputs
    in0 = 0;
    in1 = 0;

    // Apply reset
    reset_dut();

    // Test cases
    #10;
    in0 = 0;
    in1 = 1;  // Expect out0 = 0, out1 = 1
    #10;
    in0 = 1;
    in1 = 0;  // Expect out0 = 1, out1 = 0
    #10;
    in0 = 1;
    in1 = 1;  // Expect out0 = 1, out1 = 1

    // End simulation
    #50;
    $finish;
  end

endmodule
