module fsm #(
) (
    input logic clk,
    input logic rstn,
    input logic hoge
);
  logic init;
  always_ff @(posedge clk) begin
    if (!rstn) begin
    end else begin
    end
  end
endmodule
