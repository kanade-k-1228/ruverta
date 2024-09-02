module test_dff #(
  parameter BIT = 8
) (
  input  logic        clk,
  input  logic        rstn,
  input  logic [ 7:0] in0,
  input  logic [ 7:0] in1,
  output logic [ 7:0] out
);
  always_ff @(posedge clk)
    begin
      if (!rstn)
        begin
          out <= 0;
        end
      else
        begin
          out <= in0 + in1;
        end
    end
endmodule