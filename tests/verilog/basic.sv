module basic #(
    parameter BIT = 8
) (
    input  logic       clk,
    input  logic       rstn,
    input  logic [7:0] in0,
    input  logic [7:0] in1,
    output logic [7:0] out
);
  always_comb out = in0 + in1;
  always_ff @(posedge clk) begin
    a <= b;
  end
  always_comb begin
    case (hoge)
      default: a <= b;
    endcase
  end
endmodule
