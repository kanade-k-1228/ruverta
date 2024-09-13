module fsm (
    input logic clk,
    input logic rstn,
    input logic in0,
    input logic in1
);
  logic state;
  localparam INIT = 0;
  localparam RUNNING = 1;
  always_ff @(posedge clk) begin
    if (!rstn) state <= 0;
    else begin
      case (state)
        INIT: begin
          if (in0 == 1) state <= RUNNING;
          else state <= INIT;
        end
        RUNNING: begin
          if (in1 == 1) state <= INIT;
          else state <= RUNNING;
        end
      endcase
    end
  end
endmodule
