module comb
(
  input  logic        clk,
  input  logic        rstn,
  input  logic        in0,
  input  logic        in1,
  output logic        out0,
  output logic        out1
)
;
  always_comb
    begin
      if (in0==0)
        begin
          out0 = 0;
          out1 = 1;
        end
      else
        begin
          out0 = in0;
          out1 = in1;
        end
    end
endmodule