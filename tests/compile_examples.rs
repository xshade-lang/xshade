extern crate xshade;

#[test]
fn compile_flat() {
    let flat_source = include_str!("../examples/flat/flat.xs");
    let mut ast = xshade::parse_module(flat_source);

    println!("");

    println!("{:#?}", ast);

    println!("");

    // panic!("");
}
