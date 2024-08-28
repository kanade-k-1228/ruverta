use ruverta::stmt::Stmt;

#[test]
fn test_stmt() {
    let stmt = Stmt::open()
        .add(
            Stmt::cond()
                .r#if(
                    "a == b",
                    Stmt::open()
                        .add(Stmt::assign("a", "1"))
                        .add(Stmt::assign("b", "1"))
                        .close(),
                )
                .r#if("c == d", Stmt::assign("c", "c - 1"))
                .r#else(Stmt::assign("e", "2")),
        )
        .close();
    println!("{}", stmt.blocking().join("\n"));
}
