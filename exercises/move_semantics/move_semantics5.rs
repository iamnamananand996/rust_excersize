// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut x = 100;
    {
        let y = &mut x; // First mutable reference
        *y += 100;
    } // y goes out of scope here
    {
        let z = &mut x; // Second mutable reference
        *z += 1000;
    } // z goes out of scope here
    assert_eq!(x, 1200);
}
