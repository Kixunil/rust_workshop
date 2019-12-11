fn main() {
    let mut val0 = 42;
    println!("val0: {}", val0);

    let mut val1 = val0;
    println!("val0: {}", val0);
    println!("val1: {}", val1);

    val0 = 47;
    println!("val0: {}", val0);
    println!("val1: {}", val1);

    val1 = 58;
    println!("val0: {}", val0);
    println!("val1: {}", val1);

    let borrowed_val0 = &val0;
    println!("val0: {}", val0);
    println!("borrowed_val0: {}", borrowed_val0);

    let borrowed_mut_val0 = &mut val0;
    println!("borrowed_mut_val0: {}", borrowed_mut_val0);
    *borrowed_mut_val0 = 1337;
    println!("borrowed_mut_val0: {}", borrowed_mut_val0);
    println!("val0: {}", val0);

    let mut boxed_val0 = Box::new(2019);
    println!("boxed_val0: {}", boxed_val0);
    *boxed_val0 = 2020;
    println!("boxed_val0: {}", boxed_val0);
    let boxed_val1 = boxed_val0;
    println!("boxed_val1: {}", boxed_val1);

    // Compile fail
    // println!("boxed_val0: {}", boxed_val0);
}
