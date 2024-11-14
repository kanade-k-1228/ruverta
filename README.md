<div align="center">

# ruverta <!-- omit in toc -->

**Rust to Verilog: Very Simple Verilog Builder**

English | [日本語](README_JP.md)

</div>

Supports only a simple subset of SystemVerilog.

- Variables: Only `logic` is available. No `reg` or `wire`.
- Combinational circuits: Only `always_comb` is available. No `assign`.
- Sequential circuits: Only `always_ff` is available. No `always`.

**Table of Contents**

- [Installation](#installation)
- [Basic API](#basic-api)
  - [module](#module)
  - [always\_comb](#always_comb)
  - [always\_ff](#always_ff)
  - [Generate](#generate)
- [Extended API](#extended-api)
  - [DFF](#dff)
  - [Comb](#comb)
  - [FSM](#fsm)
  - [RegMap](#regmap)
  - [Stream](#stream)
  - [FIFO](#fifo)
- [Test](#test)

## Installation

```
$ cargo add --git "https://github.com/kanade-k-1228/ruverta.git"
```

or

```
[dependencies]
ruverta = { git = "https://github.com/kanade-k-1228/ruverta.git" }
```

## Basic API

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

### module

Create a module with Module::new(name).

Add parameters and input/output ports with the following methods.

- `.param(name, default_value)`
- `.input(name, width)`
- `.output(name, width)`
- `.inout(name, width)`

### always_comb

Create statements with Stmt.

### always_ff

Create a sensitivity list with `Sens::new()` and add wires to monitor with the following methods.

- `.posedge(wire_name)`
- `.negedge(wire_name)`
- `.bothedge(wire_name)`

### Generate

Generate verilog code with `.verilog()`. Since it returns `Vec<String>`, use `.join("\n")`.

> The API design is quite rough, so feel free to request anything~

## Extended API

You can build some circuit easily.

|                   | Rust                         | Verilog                              | Test                                       |
| ----------------- | ---------------------------- | ------------------------------------ | ------------------------------------------ |
| [DFF](#dff)       | [dff.sv](tests/dff.rs)       | [dff.sv](tests/verilog/dff.sv)       | [dff_tb.sv](tests/verilog/dff_tb.sv)       |
| [Comb](#comb)     | [comb.rs](tests/comb.rs)     | [comb.sv](tests/verilog/comb.sv)     | [comb_tb.sv](tests/verilog/comb_tb.sv)     |
| [FSM](#fsm)       | [fsm.rs](tests/fsm.rs)       | [fsm.sv](tests/verilog/fsm.sv)       | [fsm_tb.sv](tests/verilog/fsm_tb.sv)       |
| [RegMap](#regmap) | [regmap.rs](tests/regmap.rs) | [regmap.sv](tests/verilog/regmap.sv) | [regmap_tb.sv](tests/verilog/regmap_tb.sv) |
| [Stream](#stream) | [stream.rs](tests/stream.rs) | [stream.sv](tests/verilog/stream.sv) |                                            |
| [FIFO](#fifo)     | [fifo.rs](tests/fifo.rs)     | [fifo.sv](tests/verilog/fifo.sv)     |                                            |

### DFF

When implementing sequential circuits, it is recommended to use `sync_ff` / `async_ff` api instead of `always_ff`.

DFF has several usage patterns depending on the clock and reset settings.

- clock edge: posedge / negedge / bothedge
- reset edge: positive / negative
- reset timing: sync / async

Currently, only the following patterns are supported.

|            | clock edge | reset logic | reset timing |
| ---------- | ---------- | ----------- | ------------ |
| `sync_ff`  | posedge    | negative    | sync         |
| `async_ff` | posedge    | negative    | async        |

```rust
Module::new(name)
    .input("clk", 1)
    .input("rstn", 1)
    .input("in0", 8)
    .input("in1", 8)
    .output("out", 8)
    .sync_ff(
        "clk",
        "rstn",
        Stmt::begin().assign("out", "0").end(),
        Stmt::begin().assign("out", "in0 + in1").end(),
    );
```

### Comb

When implementing combinational circuits, it is recommended to use `comb` instead of `always_comb`.

Since it always requires a default, there are no omissions in case distinctions.

```rust
Module::new(name)
    .input("clk", 1)
    .input("rstn", 1)
    .input("hoge", 1)
    .comb(
        Comb::new()
            .input("in0")
            .input("in1")
            .output("out0")
            .output("out1")
            .case("in0==0", "out0=0", "out1=0")
            .default("0", "1"),
    );
```

### FSM

Construct a state machine with a single state variable.

```rust
Module::new(name)
    .input("clk", 1)
    .input("rstn", 1)
    .input("hoge", 1)
    .sync_fsm(
        FSM::new("init", "clk", "rstn")
            .state("init")
            .jump("hoge == 1", "fuga")
            .r#else("init")
            .state("fuga")
            .jump("hoge == 0", "init")
            .r#else("fuga"),
    );
```

### RegMap

```rust
Module::new(name)
    .input("clk", 1)
    .input("rstn", 1)
    .regmap(
        "clk",
        "rstn",
        RegMap::new("cbus", 32)
            .read_write("csr_rw0", 8, 1)
            .read_write("csr_rw1", 8, 1)
            .read_only("csr_ro", 8, 1)
            .trigger("csr_tw"),
    );
```

### Stream

### FIFO

## Test

Tests are located under tests.

```bash
$ cargo test
```

will output sv files under `tests/verilog/`.

Running make will launch gtkwave.

```bash
ruverta/tests/verilog$ make ???
```

??? is the name of the test case.
