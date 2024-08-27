# ruverta

Rust to Verilog: Very Simple Verilog Builder

Only support tiny subset of sv.

- Only `logic`: no `reg` or `wire`
- Only `always_ff` : no `always`
- Only `always_comb` : no `assign`

## Basic module builder

<div style="display: flex; width: 100%;">
<div style="width: 50%; padding-right: 10px;">

```rust:
fn test_module() {
    let mut m = Module {
        name: format!("test_mod"),
        ports: vec![],
        params: vec![],
        blocks: vec![],
    };
    m.param("BIT", Some("8"));
    m.input("clk", 1);
    m.input("rstn", 1);
    m.input("in0", 8);
    m.input("in1", 8);
    m.output("out", 8);
    m.always_comb({
        let mut a = AlwaysComb::new();
        a.stmt("out = in0 + in1;");
        a
    });
    println!("{}", m.verilog().join("\n"));
}
```

</div>
<div style="width: 50%; padding-left: 10px;">

```verilog:
module test_mod #(
  parameter BIT = 8
) (
  input  logic        clk,
  input  logic        rstn,
  input  logic [ 7:0] in0,
  input  logic [ 7:0] in1,
  output logic [ 7:0] out
);
  always_comb begin
    out = in0 + in1;
  end
endmodule;
```

</div>
</div>

## Advanced builder

You can build some circuit easily.

### Common Clock & Reset

You can write `always_ff` easily.

<div style="display: flex; width: 100%;">
<div style="width: 50%; padding-right: 10px;">

```rust:
fn test_sm(){
    let mut m = Module {
        name: format!("test_mod"),
        ports: vec![],
        params: vec![],
        blocks: vec![],
    };
    m.param("BIT", Some("8"));
    m.input("clk", 1);
    m.input("rstn", 1);
    m.input("in0", 8);
    m.input("in1", 8);
    m.output("out", 8);
    m.sync("clk", "rstn", {
        let mut a = Dff::new();
        a.stmt("out <= in0 + in1;");
        a
    });
    println!("{}", m.verilog().join("\n"));
}
```

</div>
<div style="width: 50%; padding-left: 10px;">

```verilog:
```

</div>
</div>

### CSR Bus

<div style="display: flex; width: 100%;">
<div style="width: 50%; padding-right: 10px;">

```rust:
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

</div>
<div style="width: 50%; padding-left: 10px;">

```verilog:
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

</div>
</div>

### Combinational Circuit

<div style="display: flex; width: 100%;">
<div style="width: 50%; padding-right: 10px;">

```rust:
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

</div>
<div style="width: 50%; padding-left: 10px;">

```verilog:
```

</div>
</div>

### State Machine

### And More Builders!

<div style="display: flex; width: 100%;">
<div style="width: 50%; padding-right: 10px;">

```rust:
```

</div>
<div style="width: 50%; padding-left: 10px;">

```verilog:
```

</div>
</div>
