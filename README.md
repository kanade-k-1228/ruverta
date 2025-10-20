<div align="center">

# ruverta <!-- omit in toc -->

**Rust to Verilog: Very Simple Verilog Builder**

English | [日本語](README.ja.md)

</div>

## What is ruverta for? <!-- omit in toc -->

Ruverta (/rʊˈvɛrtə/) is a library for easily creating IP generators in Rust.

- **Flexible Generation** : The abstraction of modules using SystemVerilog parameters is not very flexible. Create highly flexible IPs using Rust + Ruverta.
- **Minimalist Syntax** : Supports only simple subset of SystemVerilog which is enough for most cases.
  - Variables: Only `logic` is available. No `reg` or `wire`.
  - Combinational circuits: Only `always_comb` is available. No `assign`.
  - Sequential circuits: Only `always_ff` is available. No `always`.
- **Human Friendly** : Builder API is designed to be easy to use. Additionally, the generated SystemVerilog code is readable. You don't have to struggle with a bunch of meaningless variable names.

## Table of Contents <!-- omit in toc -->

- [Crash Course: Blink](#crash-course-blink)
- [Features](#features)
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
  - [StateMachine](#statemachine)
  - [Stream](#stream)
  - [FIFO](#fifo)
- [Bus API](#bus-api)
- [Test](#test)

## Crash Course: Blink

1. Init rust project and add ruverta

```bash
$ cargo init
$ cargo add ruverta -F module
```

2. Write code

Parameter "div"

```rust
use ruverta::{ext::DFF, module::Module, stmt::Stmt};
fn main(){
  let div: usize = 24;
  let module = Module::new("blink", "clk", "rstn")
        .logic("cnt", div, 1)
        .add(DFF::sync(Stmt::assign("cnt", "0"), Stmt::assign("cnt", "cnt + 1")))
        .input("clk", 1)
        .input("rst", 1)
        .output("led", 1)
        .always_comb(Stmt::assign("led", format!("cnt[{}]", div - 1)));
  println!("{}", module.verilog().join("\n"));
}
```

3. Generate Verilog

```bash
$ cargo run > blink.sv
```

```systemverilog
module blink (
    input  logic clk,
    input  logic rst,
    output logic led
);
  logic [23:0] cnt;
  always_ff @(posedge clk) begin
    if (!rstn) cnt <= 0;
    else cnt <= cnt + 1;
  end
  always_comb led = cnt[23];
endmodule
```

## Features

Ruverta is aimed at generating branch modules.

![](./doc/mod.drawio.svg)

- core: Only wrapper of system verilog.
- atom: Support API to generate single clock domain module.
- cros: Add support to generate cross clock domain module.
- top: Support API to generate top module. Multiple clocks and resets can be specified.

## Basic API

<table><tr><th>Rust</th><th>SystemVerilog</th></tr><tr><td>

```rust
use ruverta::{module::{Module, Sens}, stmt::Stmt};
fn test_module() {
    let m = Module::new("test_module", "clk", "rstn")
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
| [StateMachine](#statemachine) | [state_machine.rs](tests/state_machine.rs)   | [state_machine.sv](tests/verilog/state_machine.sv)   | [state_machine_tb.sv](tests/verilog/state_machine_tb.sv)   |
| [AXILiteSlave](#axiliteslave) | [axi_lite_slave.rs](tests/axi_lite_slave.rs) | [axi_lite_slave.sv](tests/verilog/axi_lite_slave.sv) | [axi_lite_slave_tb.sv](tests/verilog/axi_lite_slave_tb.sv) |
| [PicoSlave](#picoslave)       |                                              |                                                      |                                                            |
| [Stream](#stream)             | [stream.rs](tests/stream.rs)                 | [stream.sv](tests/verilog/stream.sv)                 |                                                            |
| [FIFO](#fifo)                 | [fifo.rs](tests/fifo.rs)                     | [fifo.sv](tests/verilog/fifo.sv)                     |                                                            |

### DFF

When implementing sequential circuits, it is recommended to use the `DFF` extension instead of `always_ff`.

DFF has several usage patterns depending on the clock and reset settings.

- clock edge: posedge / negedge / bothedge
- reset edge: positive / negative
- reset timing: sync / async

Currently, only the following patterns are supported.

|              | clock edge | reset logic | reset timing |
| ------------ | ---------- | ----------- | ------------ |
| `DFF::sync`  | posedge    | negative    | sync         |
| `DFF::async` | posedge    | negative    | async        |

Clock and reset signals are taken from the module's default clock and reset.

```rust
use ruverta::{ext::DFF, module::Module, stmt::Stmt};

Module::new("example", "clk", "rstn")
    .input("clk", 1)
    .input("rstn", 1)
    .input("in0", 8)
    .input("in1", 8)
    .output("out", 8)
    .add(DFF::sync(
        Stmt::begin().assign("out", "0").end(),
        Stmt::begin().assign("out", "in0 + in1").end(),
    ));
```

### Comb

When implementing combinational circuits, it is recommended to use the `Comb` extension instead of `always_comb`.

Since it always requires a default, there are no omissions in case distinctions.

```rust
use ruverta::{ext::Comb, module::Module};

Module::new("example", "clk", "rstn")
    .input("clk", 1)
    .input("rstn", 1)
    .input("in0", 1)
    .input("in1", 1)
    .output("out0", 1)
    .output("out1", 1)
    .add(
        Comb::new()
            .input("in0")
            .input("in1")
            .output("out0")
            .output("out1")
            .case("in0==0", vec!["0", "1"])
            .default(vec!["in0", "in1"]),
    );
```

### StateMachine

Construct a state machine with a single state variable.

```rust
use ruverta::{ext::StateMachine, module::Module};

const INIT: &str = "INIT";
const FUGA: &str = "FUGA";

Module::new("example", "clk", "rstn")
    .input("clk", 1)
    .input("rstn", 1)
    .input("hoge", 1)
    .add(
        StateMachine::new("state")
            .state(INIT)
            .jump("hoge == 1", FUGA)
            .r#else(INIT)
            .state(FUGA)
            .jump("hoge == 0", INIT)
            .r#else(FUGA),
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
use ruverta::{bus::{AXILiteSlave, RegList}, module::Module};

Module::new("example", "clk", "rstn")
  .input("clk", 1)
  .input("rstn", 1)
  .add(AXILiteSlave::new(
    Some("cbus"),
    "clk",
    "rstn",
    RegList::new()
      .read_write("csr_rw", 8, 4)
      .read_only("csr_ro", 8, 1)
      .trigger("csr_tw")
      .allocate_greedy(32, 8),
  ));
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
