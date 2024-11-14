`timescale 1ns / 1ps

module tb;

  initial begin
    $dumpfile("axilite.vcd");
    $dumpvars(0, dut);
    #50000;
    $finish;
  end

  // --------------------------------------------------------------------------

  // Clock and Reset
  logic        clk;
  logic        rstn;

  // AXI-Lite Bus
  logic [31:0] cbus_awaddr;
  logic [ 2:0] cbus_awprot;
  logic        cbus_awvalid;
  logic        cbus_awready;

  logic [31:0] cbus_wdata;
  logic [ 3:0] cbus_wstrb;
  logic        cbus_wvalid;
  logic        cbus_wready;

  logic [ 1:0] cbus_bresp;
  logic        cbus_bvalid;
  logic        cbus_bready;

  logic [31:0] cbus_araddr;
  logic [ 2:0] cbus_arprot;
  logic        cbus_arvalid;
  logic        cbus_arready;

  logic [31:0] cbus_rdata;
  logic [ 1:0] cbus_rresp;
  logic        cbus_rvalid;
  logic        cbus_rready;

  // verilator lint_off WIDTHTRUNC
  axilite dut (.*);
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
  // AXI Lite

  task axil_init();
    begin
      cbus_awaddr  = 0;
      cbus_awprot  = 3'b000;
      cbus_awvalid = 0;
      cbus_wdata   = 0;
      cbus_wstrb   = 4'b0000;
      cbus_wvalid  = 0;
      cbus_bready  = 0;
      cbus_araddr  = 0;
      cbus_arprot  = 3'b000;
      cbus_arvalid = 0;
      cbus_rready  = 0;
    end
  endtask

  task axil_write(input [31:0] addr, input [31:0] data);
    begin
      cbus_awaddr  = addr;
      cbus_awvalid = 1;
      cbus_wdata   = data;
      cbus_wstrb   = 4'b1111;
      cbus_wvalid  = 1;
      cbus_bready  = 1;
      wait (cbus_awready);
      wait (cbus_wready);
      wait (cbus_bvalid);
      cbus_awvalid = 0;
      cbus_wvalid  = 0;
      wait (!cbus_bvalid);
      cbus_bready = 0;
    end
  endtask

  task axil_read(input [31:0] addr, output [31:0] data);
    begin
      cbus_araddr  = addr;
      cbus_arvalid = 1;
      cbus_rready  = 1;
      wait (cbus_arready);
      wait (cbus_rvalid);
      cbus_arvalid = 0;
      data = cbus_rdata;
      wait (!cbus_rvalid);
      cbus_rready = 0;
    end
  endtask

  // --------------------------------------------------------------------------

  logic [31:0] tmp_rdata;

  initial begin
    #10000;
    $finish;
  end

  initial begin
    clk  = 0;
    rstn = 1;
    axil_init();
    reset_dut();

    // Writing to all registers

    axil_write(0, 32'hFFFF_FFFF);
    axil_write(1, 32'hFFFF_FFFF);
    axil_write(2, 32'hFFFF_FFFF);
    axil_write(3, 32'hFFFF_FFFF);

    // Reading from all registers

    axil_read(0, tmp_rdata);
    axil_read(1, tmp_rdata);
    axil_read(2, tmp_rdata);
    axil_read(3, tmp_rdata);

    #20;
    $finish;
  end

endmodule
