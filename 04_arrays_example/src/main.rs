fn main() {
    let arr0 = [0, 1, 2, 4];
    let arr1 = [0; 3];

    println!("{}", arr0[1]);
    println!("{}", arr1[1]);

    let subslice = &arr0[1..2];

    println!("{:?}", subslice);

    let mut vec = Vec::new();

    println!("{:?}", vec);

    vec.push(42);

    println!("{:?}", vec);

    fn process_slice(slice: &[u32]) {
        println!("Slice: {:?}", slice);
    }

    process_slice(subslice);
    process_slice(&vec);

    let mut vec_expr = vec![5, 4, 3, 2, 1, 0];
    vec_expr.pop();
    println!("vec_expr: {:?}", vec_expr);

    let _big = Box::new([0u8; 4096]) as Box<[u8]>;
}
