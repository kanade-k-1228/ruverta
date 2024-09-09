module basic #(
    parameter BIT = 8
) (
    input  logic       clk,
    input  logic       rstn,
    input  logic [7:0] in0,
    input  logic [7:0] in1,
    output logic [7:0] out
);
  logic [7:0] tmp;
  always_comb tmp = in0 + in1;
  always_ff @(posedge clk) out <= tmp;
  always_comb begin
    case (hoge)
      0: ;
      1: ;
    endcase
  end
endmodule
