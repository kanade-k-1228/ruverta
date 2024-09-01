<div align="center">

# ruverta

**Rust to Verilog: Very Simple Verilog Builder**

English | [日本語](./doc/README_JP.md)

</div>

Only support tiny subset of sv.

- Only `logic`: no `reg` or `wire`
- Only `always_ff` : no `always`
- Only `always_comb` : no `assign`

## Install

```
$ cargo add --git "https://github.com/kanade-k-1228/ruverta.git"
```

or

```
[dependencies]
ruverta = { git = "https://github.com/kanade-k-1228/ruverta.git" }
```

## Basic module builder

<table><tr><th>Rust</th><th>SystemVerilog</th></tr><tr><td>

```rust
use ruverta::{Module, Sens, Stmt};
fn test_module() {
    let m = Module::new("test_module")
        .param("BIT", Some("8"))
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 8)
        .input("in1", 8)
        .output("out", 8)
        .always_comb(Stmt::assign("out", "in0 + in1"))
        .always_ff(
            Sens::new().posedge("clk"),
            Stmt::begin().add(Stmt::assign("a", "b")).end(),
        );
    println!("{}", m.verilog().join("\n"));
}
```

</td><td>

```systemverilog
module test_module #(
  parameter BIT = 8
) (
  input  logic        clk,
  input  logic        rstn,
  input  logic [ 7:0] in0,
  input  logic [ 7:0] in1,
  output logic [ 7:0] out
);
  always_comb
    out = in0 + in1;
  always_ff @(posedge clk)
    begin
      a <= b;
    end
endmodule;
```

</td></tr></table>

## Advanced builder

You can build some circuit easily.

- DFF: Setup clock / reset
- 

### Common Clock & Reset

You can write `always_ff` slight easily.

<table><tr><th>Rust</th><th>SystemVerilog</th></tr><tr><td>

```rust
fn test_sm(){
    let mut m = Module::new("test_mod")
      .param("BIT", Some("8"))
      .input("clk", 1)
      .input("rstn", 1)
      .input("in0", 8)
      .input("in1", 8)
      .output("out", 8)
      .sync("clk", "rstn",
        Dff::new("clk", "rstn").
          .stmt("out <= in0 + in1;")
      );
    println!("{}", m.verilog().join("\n"));
}
```

</td><td>

```verilog
```

</td></tr></table>

### CSR Bus

<table><tr><th>Rust</th><th>SystemVerilog</th></tr><tr><td>

```rust
fn test_csr() {
    let regmap = csr::RegMap::new("cbus", 32)
        .read_only("ctrl", 8, 2)
        .read_only("buf", 8, 1)
        .trigger("start");
    let module = module::Module::new("test_csr")
        .regmap(&regmap)
        .regio(&regmap);
    println!("{}", module.verilog().join("\n"))
}
```

</td><td>

```verilog
module test_csr #(
) (
  input  logic [ 6:0] cbus_awaddr,
  input  logic        cbus_awvalid,
  output logic        cbus_awready,
  input  logic [31:0] cbus_wdata,
  input  logic [ 3:0] cbus_wstrb,
  input  logic        cbus_wvalid,
  output logic        cbus_wready,
  output logic [ 1:0] cbus_bresp,
  output logic        cbus_bvalid,
  input  logic        cbus_bready,
  input  logic [ 6:0] cbus_araddr,
  input  logic        cbus_arvalid,
  output logic        cbus_arready,
  output logic [31:0] cbus_rdata,
  output logic        cbus_rvalid,
  input  logic        cbus_rready,
  input  logic [ 7:0] ro_ctrl,
  input  logic [ 7:0] ro_buf,
  output logic        tw_start_trig,
  input  logic        tw_start_resp
);
  logic [ 7:0] ro_ctrl[ 7:0];
  logic [ 7:0] ro_buf;
  logic        tw_start_trig;
  logic        tw_start_resp;
endmodule;
```

</td></tr></table>

### Combinational Circuit

<table><tr><th>Rust</th><th>SystemVerilog</th></tr><tr><td>

```rust
let mut module = Module::new("sample_mod");
module.input("in0", 0, 0);
module.input("in1", 0, 0);

let mut comb = Comb::new(
vec!["in0", "in1"],
vec![
    Wire{name: "out0", default: 0},
    Wire{name: "out1", default: 1},
]);
comb.case()
comb.default()

Comb::build(module, comb);
```

</td><td>

```verilog
```

</td></tr></table>

### State Machine

<table><tr><th>Rust</th><th>SystemVerilog</th></tr><tr><td>

```rust
```

</td><td>

```verilog
```

</td></tr></table>

### And More Builders!

<table><tr><th>Rust</th><th>SystemVerilog</th></tr><tr><td>

```rust
```

</td><td>

```verilog
```

</td></tr></table>
