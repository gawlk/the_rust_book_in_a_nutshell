use the_rust_book_in_a_nutshell::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    assert_eq!(4, add_two(2));
}
