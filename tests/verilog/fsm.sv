module fsm #(
) (
    input logic clk,
    input logic rstn
);
  logic state;
  localparam init = 0;
  localparam fuga = 1;
  always_ff @(posedge clk) begin
    if (!rstn) begin
      state <= 0;
    end else begin
      case (state)
        init: begin
        end
        init: begin
          if (hoge == 1) state <= fuga;
        end
        fuga: begin
          if (hoge == 0) state <= init;
        end
      endcase
    end
  end
endmodule
