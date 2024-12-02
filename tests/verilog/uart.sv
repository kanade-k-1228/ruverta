module uart (
    inout  logic        clk,
    input  logic        rstn,
    output logic        tx,
    input  logic        rx,
    input  logic [ 7:0] cbus_awaddr,
    input  logic        cbus_awvalid,
    output logic        cbus_awready,
    input  logic [31:0] cbus_wdata,
    input  logic [ 3:0] cbus_wstrb,
    input  logic        cbus_wvalid,
    output logic        cbus_wready,
    output logic [ 1:0] cbus_bresp,
    output logic        cbus_bvalid,
    input  logic        cbus_bready,
    input  logic [ 7:0] cbus_araddr,
    input  logic        cbus_arvalid,
    output logic        cbus_arready,
    output logic [31:0] cbus_rdata,
    output logic [ 1:0] cbus_rresp,
    output logic        cbus_rvalid,
    input  logic        cbus_rready
);
  logic [31:0] div;
  logic [ 7:0] tx_data;
  logic [ 7:0] rx_data;
  always_ff @(posedge clk) begin
    if (!rstn) begin
      div <= 0;
      tx_data <= 0;
    end else begin
      if (cbus_wvalid && cbus_awvalid) begin
        case (cbus_awaddr)
          0: div <= cbus_wdata[31:0];
          1: tx_data <= cbus_wdata[7:0];
          default: ;
        endcase
      end
    end
  end
  always_ff @(posedge clk) begin
    if (!rstn) cbus_rdata <= 0;
    else begin
      if (cbus_arvalid) begin
        case (cbus_araddr)
          0: cbus_rdata[31:0] <= div;
          1: cbus_rdata[7:0] <= tx_data;
          2: cbus_rdata[7:0] <= rx_data;
          default: cbus_rdata <= 0;
        endcase
      end
    end
  end
  always_ff @(posedge clk) begin
    if (!rstn) begin
      cbus_awready <= 0;
      cbus_wready  <= 0;
      cbus_bvalid  <= 0;
      cbus_arready <= 0;
      cbus_rvalid  <= 0;
      cbus_bresp   <= 0;
      cbus_rresp   <= 0;
    end else begin
      cbus_awready <= cbus_awvalid && !cbus_awready;
      cbus_wready  <= cbus_wvalid && !cbus_wready;
      cbus_bvalid  <= cbus_awready && cbus_wready && !cbus_bvalid;
      cbus_arready <= cbus_arvalid && !cbus_arready;
      cbus_rvalid  <= cbus_arvalid && !cbus_arready;
      if (cbus_bvalid && cbus_bready) cbus_bvalid <= 0;
      if (cbus_rvalid && cbus_rready) cbus_rvalid <= 0;
    end
  end
  always_ff @(posedge clk or negedge rstn) begin
    if (!rstn) begin
      buffer <= 0;
      cnt <= 0;
    end else begin
      buffer <= a;
    end
  end
endmodule
