use ruverta::stmt::Stmt;

#[test]
fn test_stmt() {
    let stmt = Stmt::begin()
        .add(
            Stmt::cond()
                .r#if(
                    "a == b",
                    Stmt::begin().assign("a", "1").assign("b", "1").end(),
                )
                .r#if("c == d", Stmt::assign("c", "c - 1"))
                .r#else(Stmt::assign("e", "2")),
        )
        .end();
    println!("{}", stmt.blocking().join("\n"));
}
