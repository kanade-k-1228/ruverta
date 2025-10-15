module blink (
    input  logic clk,
    input  logic rst,
    output logic led
);
  logic [23:0] cnt;
  always_ff @(posedge clk) begin
    if (!rstn) cnt <= 0;
    else cnt <= cnt + 1;
  end
  always_comb led = cnt[23];
endmodule
