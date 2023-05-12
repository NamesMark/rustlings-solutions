// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.



fn main() {
    let mut x = 100; // x is i32
    let y = &mut x; // y is &mut i32
    *y += 100; // we dereference y and add 100 to the value it points to
    let z = &mut x; 
    *z += 1000;
    assert_eq!(x, 1200);
}
