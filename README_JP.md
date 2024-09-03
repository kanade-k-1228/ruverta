<div align="center">

# ruverta

**Rust で Verilog を書くための簡単なライブラリ**

[English](README.md) | 日本語

</div>

SystemVerilogの簡単なサブセットのみをサポートしています。

- 変数：`logic`のみ使用可能です。`reg` と `wire` はありません。
- 組合回路：`always_comb`のみ使用可能です。 `assign`はありません。
- 順序回路：`always_ff`のみ使用可能です。`always`はありません。

## インストール

```
$ cargo add --git "https://github.com/kanade-k-1228/ruverta.git"
```

or

```
[dependencies]
ruverta = { git = "https://github.com/kanade-k-1228/ruverta.git" }
```

## 使い方

メソッドチェーンを用いてモジュールを作成します。

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

### モジュール

`Module::new()` でモジュールを作成します。

パラメタや入出力ポートを以下のメソッドで追加します。

- `.param(name,default_value)`
- `input(name, width)`
- `output(name, width)`
- `inout(name, width)`

### always_comb

`Stmt` で文を作成します。

### always_ff

`Sens::new()` でセンシティビティリストを作成し、監視するワイヤを以下のメソッドで追加します。

- `.posedge(&str)`
- `.negedge(&str)`
- `.bothedge(&str)`

### 出力

`.verilog()` で出力できます。`Vec<String>` なので `.join("\n")` してください。

> APIの設計はわりと雑なので、リクエストあったらなんでもどうぞ～

## 拡張機能

|                   | Rust                         | Verilog                                   |
| ----------------- | ---------------------------- | ----------------------------------------- |
| [DFF](#dff)       | [dff.rs](tests/dff.rs)       | [dff.sv](tests/verilog/test_dff.sv)       |
| [Comb](#comb)     | [comb.rs](tests/comb.rs)     | [comb.sv](tests/verilog/test_comb.sv)     |
| [FSM](#fsm)       | [fsm.rs](tests/fsm.rs)       | [fsm.sv](tests/verilog/test_fsm.sv)       |
| [CSR](#csr)       | [regmap.rs](tests/regmap.rs) | [regmap.sv](tests/verilog/test_regmap.sv) |
| [Stream](#stream) | [stream.rs](tests/stream.rs) | [stream.sv](tests/verilog/test_stream.sv) |
| [FIFO](#fifo)     | [fifo.rs](tests/fifo.rs)     | [fifo.sv](tests/verilog/test_fifo.sv)     |

### DFF

順序回路は `always_ff` ではなく、`sync_ff` / `async_ff` を使うことを推奨します。

DFF には何パターンかの使い方があります。

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

### CSR

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

### And More ...!
