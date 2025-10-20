<div align="center">

# ruverta <!-- omit in toc -->

**Rust で Verilog を書くための簡単なライブラリ**

[English](README.md) | 日本語

</div>

## Ruverta とは <!-- omit in toc -->

Ruverta (/rʊˈvɛrtə/) は Rust で IP ジェネレータを簡単に作るためのライブラリです。

- **柔軟性が高い** : SystemVerilog のパラメタを用いたモジュールの抽象化は柔軟性が低いです。Rust + Ruverta を使っては柔軟性の高い IP を作成しましょう。
- **最小限の文法** : SystemVerilog のシンプルなサブセットのみをサポートしており、ほとんどの場合に十分です。
  - 変数：`logic`のみ使用可能です。`reg` と `wire` はありません。
  - 組合回路：`always_comb`のみ使用可能です。 `assign`はありません。
  - 順序回路：`always_ff`のみ使用可能です。`always`はありません。
- **人間に優しい** : ビルダー API は人間工学的に使いやすいように設計されています。また生成される SystemVerilog コードも人間にとって読みやすいものになっています。大量の無意味な変数名に悩まされる必要はありません。

## 目次 <!-- omit in toc -->

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
  - [StateMachine](#statemachine)
  - [Stream](#stream)
  - [FIFO](#fifo)
- [Bus API](#bus-api)
- [Test](#test)

## インストール

```
$ cargo add ruverta
```

## 基本 API

`Module::new(name)` でモジュールを作成し、メソッドチェーンで要素を追加します。最後に `.verilog()` メソッドで Verilog コードを生成します。

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

## 拡張 API

Module のビルダメソッドを拡張して、さまざまな回路を簡単に構築できるようにします。

|                               | Rust                                       | Verilog                                            | Test                                                     |
| ----------------------------- | ------------------------------------------ | -------------------------------------------------- | -------------------------------------------------------- |
| [DFF](#dff)                   | [dff.sv](tests/dff.rs)                     | [dff.sv](tests/verilog/dff.sv)                     | [dff_tb.sv](tests/verilog/dff_tb.sv)                     |
| [Comb](#comb)                 | [comb.rs](tests/comb.rs)                   | [comb.sv](tests/verilog/comb.sv)                   | [comb_tb.sv](tests/verilog/comb_tb.sv)                   |
| [StateMachine](#statemachine) | [state_machine.rs](tests/state_machine.rs) | [state_machine.sv](tests/verilog/state_machine.sv) | [state_machine_tb.sv](tests/verilog/state_machine_tb.sv) |
| [Stream](#stream)             | [stream.rs](tests/stream.rs)               | [stream.sv](tests/verilog/stream.sv)               |                                                          |
| [FIFO](#fifo)                 | [fifo.rs](tests/fifo.rs)                   | [fifo.sv](tests/verilog/fifo.sv)                   |                                                          |

### DFF

順序回路を実装する場合 `always_ff` ではなく、`DFF` 拡張を使うことを推奨します。

DFF には、クロックとリセットの設定によって何パターンかの使い方があります。

- clock edge: posedge / negedge / bothedge
- reset edge: positive / negative
- reset timing: sync / async

いまのところ、次のパターンのみに対応しています。

|              | clock edge | reset logic | reset timing |
| ------------ | ---------- | ----------- | ------------ |
| `DFF::sync`  | posedge    | negative    | sync         |
| `DFF::async` | posedge    | negative    | async        |

クロックとリセット信号はモジュールのデフォルトクロック・リセットから取得されます。

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

組合回路を実装する場合 `always_comb` ではなく、`Comb` 拡張を使うことを推奨します。

default を必ず要求するため、場合分けの漏れがありません。

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

状態変数が１つのステートマシンを構築します。

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
| PicoSlave    |                                              |                                                      |                                                            |

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
