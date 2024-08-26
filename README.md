# ruverta

Rust to Verilog: Very Simple Verilog Builder

Only support tiny subset of sv.

## Basic module builder

```rust:
let mut module = Module::new("sample_mod");
module.input("in0", 4);
module.output("out0", 4);
module.inout("inout0", 4);
module.param("AWIDTH", "$clog2(LEN)");
```

## Advanced builder

You can build some well known circuits easily.

### AXI Lite Bus

```rust:
fn test_axi_lite() {
    let mut module = Module::new("sample_mod");
    let mut axil = AXILite::new("mbus0", 32);
    axil.reg(Reg::ReadWrite, "ctrl", 2);
    axil.vec(Reg::ReadOnly, "buf", 8);
    AXILite::build(&mut module, &axil);
    println!("{}", module.verilog())
}
```

### Combinational Circuit

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

### State Machine
