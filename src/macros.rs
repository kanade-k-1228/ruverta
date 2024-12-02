#[macro_export]
macro_rules! mod_test {
    ($name:ident, $module:expr) => {
        #[test]
        fn $name() {
            use std::{fs, path::PathBuf};
            let m = $module;
            let s = m.verilog().join("\n");
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push(format!("tests/verilog/{}.sv", stringify!($name)));
            fs::write(path, s).unwrap();
        }
    };
}
