<div align="center">

# ruverta <!-- omit in toc -->

**Rust で Verilog を書くための簡単なライブラリ**

[English](README.md) | 日本語

</div>

SystemVerilog の簡単なサブセットのみをサポートしています。

- 変数：`logic`のみ生成可能です。`reg` と `wire` はありません。
- 組合回路：`always_comb`のみ生成可能です。 `assign`はありません。
- 順序回路：`always_ff`のみ生成可能です。`always`はありません。

**目次**

- [インストール](#インストール)
- [基本 API](#基本-api)
  - [入出力ポート](#入出力ポート)
  - [パラメタ](#パラメタ)
  - [ワイヤ](#ワイヤ)
  - [インスタンス](#インスタンス)
  - [組み合わせ回路](#組み合わせ回路)
  - [順序回路](#順序回路)
  - [Verilog の生成](#verilog-の生成)
- [拡張 API](#拡張-api)
  - [DFF](#dff)
  - [Comb](#comb)
  - [FSM](#fsm)
  - [Stream](#stream)
  - [FIFO](#fifo)
- [Bus API](#bus-api)
- [Test](#test)

## インストール

```
$ cargo add --git "https://github.com/kanade-k-1228/ruverta.git"
```

or

```
[dependencies]
ruverta = { git = "https://github.com/kanade-k-1228/ruverta.git" }
```

## 基本 API

`Module::new(name)` でモジュールを作成し、メソッドチェーンで要素を追加します。最後に `.verilog()` メソッドで Verilog コードを生成します。

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

### 入出力ポート

- `.input(name, width)`
- `.output(name, width)`
- `.inout(name, width)`

### パラメタ

- `.param(name, default_value)`
- `.lparam(name, value)`

### ワイヤ

- `.logic(name, bit, len)`

### インスタンス

- `.instant(inst: Instant)`

### 組み合わせ回路

- `.always_comb(stmt: Stmt)`

`Stmt` は文を表すクラスです。

### 順序回路

- `.always_ff(Sens, Stmt)`

`Sens` はセンシティビティリストを表すクラスです。

- `.posedge(wire_name)`
- `.negedge(wire_name)`
- `.bothedge(wire_name)`

### Verilog の生成

`.verilog()` で verilog を生成します。`Vec<String>` を返すので `.join("\n")` で結合してください。

> API の設計はわりと雑なので、リクエストあったらなんでもどうぞ～

## 拡張 API

Module のビルダメソッドを拡張して、さまざまな回路を簡単に構築できるようにします。

|                   | Rust                         | Verilog                              | Test                                   |
| ----------------- | ---------------------------- | ------------------------------------ | -------------------------------------- |
| [DFF](#dff)       | [dff.sv](tests/dff.rs)       | [dff.sv](tests/verilog/dff.sv)       | [dff_tb.sv](tests/verilog/dff_tb.sv)   |
| [Comb](#comb)     | [comb.rs](tests/comb.rs)     | [comb.sv](tests/verilog/comb.sv)     | [comb_tb.sv](tests/verilog/comb_tb.sv) |
| [FSM](#fsm)       | [fsm.rs](tests/fsm.rs)       | [fsm.sv](tests/verilog/fsm.sv)       | [fsm_tb.sv](tests/verilog/fsm_tb.sv)   |
| [Stream](#stream) | [stream.rs](tests/stream.rs) | [stream.sv](tests/verilog/stream.sv) |                                        |
| [FIFO](#fifo)     | [fifo.rs](tests/fifo.rs)     | [fifo.sv](tests/verilog/fifo.sv)     |                                        |

### DFF

順序回路を実装する場合 `always_ff` ではなく、`sync_ff` / `async_ff` を使うことを推奨します。

DFF には、クロックとリセットの設定によって何パターンかの使い方があります。

- clock edge: posedge / negedge / bothedge
- reset edge: positive / negative
- reset timing: sync / async

いまのところ、次のパターンのみに対応しています。

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

組合回路を実装する場合 `always_comb` ではなく、`comb` を使うことを推奨します。

default を必ず要求するため、場合分けの漏れがありません。

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

状態変数が１つのステートマシンを構築します。

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
| PicoSlave    |                                              |                                                      |                                                            |

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

`tests/` 以下にテストがあります。

```bash
$ cargo test
```

を実行すると `tests/verilog/` 以下に sv ファイルが出力されます。

make を実行すると gtkwave が立ち上がり見えます。

```bash
ruverta/tests/verilog$ make ???
```

??? はテストケースの名前です。
