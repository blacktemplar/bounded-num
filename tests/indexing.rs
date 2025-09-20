#![cfg(feature = "indexing")]

#[test]
fn compile_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/indexing/*.rs");
}
