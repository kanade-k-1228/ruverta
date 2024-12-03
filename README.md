<div align="center">

# ruverta <!-- omit in toc -->

**Rust to Verilog: Very Simple Verilog Builder**

English | [日本語](README_JP.md)

</div>

## What is ruverta for? <!-- omit in toc -->

Ruverta is a library for easily creating IP generators in Rust.

- **Flexible Generation** : The abstraction of modules using SystemVerilog parameters is not very flexible. Create highly flexible IPs using Rust + Ruverta.
- **Minimalist Syntax** : Supports only simple subset of SystemVerilog which is enough for most cases.
  - Variables: Only `logic` is available. No `reg` or `wire`.
  - Combinational circuits: Only `always_comb` is available. No `assign`.
  - Sequential circuits: Only `always_ff` is available. No `always`.
- **Human Friendly** : Builder API is designed to be easy to use. Additionally, the generated SystemVerilog code is readable. You don't have to struggle with a bunch of meaningless variable names.

## Table of Contents <!-- omit in toc -->

- [Installation](#installation)
- [Basic API](#basic-api)
  - [Input/Output Ports](#inputoutput-ports)
  - [Parameters](#parameters)
  - [Wires](#wires)
  - [Instances](#instances)
  - [Combinational Circuits](#combinational-circuits)
  - [Sequential Circuits](#sequential-circuits)
  - [Verilog Generation](#verilog-generation)
- [Extended API](#extended-api)
  - [DFF](#dff)
  - [Comb](#comb)
  - [FSM](#fsm)
  - [Stream](#stream)
  - [FIFO](#fifo)
- [Bus API](#bus-api)
- [Test](#test)

## Installation

```
$ cargo add ruverta
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

### Input/Output Ports

- `.input(name, width)`
- `.output(name, width)`
- `.inout(name, width)`

### Parameters

- `.param(name, default_value)`
- `.lparam(name, value)`

### Wires

- `.logic(name, bit, len)`

### Instances

- `.instant(inst: Instant)`

### Combinational Circuits

- `.always_comb(stmt: Stmt)`

`Stmt` is a class representing a statement.

### Sequential Circuits

- `.always_ff(Sens, Stmt)`

`Sens` is a class representing a sensitivity list.

- `.posedge(wire_name)`
- `.negedge(wire_name)`
- `.bothedge(wire_name)`

### Verilog Generation

Generate Verilog with `.verilog()`. Since it returns `Vec<String>`, use `.join("\n")` to concatenate.

## Extended API

Extend the builder methods of Module to easily construct various circuits.

|                               | Rust                                         | Verilog                                              | Test                                                       |
| ----------------------------- | -------------------------------------------- | ---------------------------------------------------- | ---------------------------------------------------------- |
| [DFF](#dff)                   | [dff.sv](tests/dff.rs)                       | [dff.sv](tests/verilog/dff.sv)                       | [dff_tb.sv](tests/verilog/dff_tb.sv)                       |
| [Comb](#comb)                 | [comb.rs](tests/comb.rs)                     | [comb.sv](tests/verilog/comb.sv)                     | [comb_tb.sv](tests/verilog/comb_tb.sv)                     |
| [FSM](#fsm)                   | [fsm.rs](tests/fsm.rs)                       | [fsm.sv](tests/verilog/fsm.sv)                       | [fsm_tb.sv](tests/verilog/fsm_tb.sv)                       |
| [AXILiteSlave](#axiliteslave) | [axi_lite_slave.rs](tests/axi_lite_slave.rs) | [axi_lite_slave.sv](tests/verilog/axi_lite_slave.sv) | [axi_lite_slave_tb.sv](tests/verilog/axi_lite_slave_tb.sv) |
| [PicoSlave](#picoslave)       |                                              |                                                      |                                                            |
| [Stream](#stream)             | [stream.rs](tests/stream.rs)                 | [stream.sv](tests/verilog/stream.sv)                 |                                                            |
| [FIFO](#fifo)                 | [fifo.rs](tests/fifo.rs)                     | [fifo.sv](tests/verilog/fifo.sv)                     |                                                            |

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

### Stream

### FIFO

## Bus API

|              | Rust                                         | Verilog                                              | Test                                                       |
| ------------ | -------------------------------------------- | ---------------------------------------------------- | ---------------------------------------------------------- |
| AXILiteSlave | [axi_lite_slave.rs](tests/axi_lite_slave.rs) | [axi_lite_slave.sv](tests/verilog/axi_lite_slave.sv) | [axi_lite_slave_tb.sv](tests/verilog/axi_lite_slave_tb.sv) |
| PicoSlave    | [pico_slave.rs](tests/pico_slave.rs)         |                                                      |                                                            |

```rust
Module::new(name)
  .input("clk", 1)
  .input("rstn", 1)
  .axi_lite_slave(
    "clk",
    "rstn",
    AXILiteSlave::new(
      "cbus",
      MMap::new(32, 32)
        .read_write("csr_rw", 8, 2)
        .read_only("csr_ro", 8, 1)
        .trigger("csr_tw"),
      ),
  );
```

- AXI Lite Slave
- Pico Slave

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
