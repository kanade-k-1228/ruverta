pub mod module;
pub mod regmap;
pub mod traits;

use module::{AlwaysFF, Module};
use traits::Verilog;
#[test]
fn hoge() {
    let mut m = Module::new("hoge");
    m.inout("hoge", 1);
    m.always_ff({
        let mut a = AlwaysFF::new();
        a.posedge("clk");
        a.stmt("if (!rstn) begin");
        a.stmt("end else begin");
        a.stmt("end");
        a
    });
    println!("{}", m.verilog().join("\n"));
}
