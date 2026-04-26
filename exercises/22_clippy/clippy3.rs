fn main() {
    let my_option: Option<&str> = None;

    // FIX 1: Use 'if let' instead of 'is_some() + unwrap()'
    // This is the specific pattern Clippy is looking for.
    if let Some(value) = my_option {
        println!("{}", value);
    }

    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3, // FIX 2: The comma
        -4, -5, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    // FIX 3: Use clear()
    my_vec.clear();
    println!("This Vec is empty, see? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // FIX 4: Use swap()
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}