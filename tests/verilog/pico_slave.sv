module pico_slave (
    input  logic        clk,
    input  logic        rstn,
    input  logic        valid,
    input  logic        ready,
    input  logic [ 3:0] wstrb,
    input  logic [ 7:0] addr,
    input  logic [31:0] wdata,
    output logic [31:0] rdata
);
  logic [7:0] csr_rw      [3:0];
  logic [7:0] csr_ro;
  logic       csr_tw_trig;
  logic       csr_tw_resp;
  always_ff @(posedge clk) begin
    if (!rstn) begin
      csr_rw[0]   <= 0;
      csr_rw[1]   <= 0;
      csr_rw[2]   <= 0;
      csr_rw[3]   <= 0;
      csr_tw_trig <= 0;
    end else begin
      case (addr)
        0: csr_rw[0] <= wdata[7:0];
        1: csr_rw[1] <= wdata[7:0];
        2: csr_rw[2] <= wdata[7:0];
        3: csr_rw[3] <= wdata[7:0];
        5: csr_tw_trig <= wdata[0:0];
        default: ;
      endcase
    end
  end
  always_comb begin
    case (addr)
      0: rdata[7:0] = csr_rw[0];
      1: rdata[7:0] = csr_rw[1];
      2: rdata[7:0] = csr_rw[2];
      3: rdata[7:0] = csr_rw[3];
      4: rdata[7:0] = csr_ro;
      5: rdata[0:0] = csr_tw_resp;
      default: rdata = 0;
    endcase
  end
endmodule
