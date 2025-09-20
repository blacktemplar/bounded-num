mod add {
    #[test]
    fn compile_fails() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_fail/ops/add/*.rs");
    }
}
