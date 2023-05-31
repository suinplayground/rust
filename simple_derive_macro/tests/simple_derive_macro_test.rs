use simple_derive_macro::Simple;

#[derive(Simple)]
struct TestStruct;

#[test]
fn simple_derive_macro_test() {
    let test_struct = TestStruct;
    test_struct.simple(); // エラーが発生しないことを確認
}
