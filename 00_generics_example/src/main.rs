fn noop<T>(val: T) -> T {
    val
}

fn show_size<T>(_value: T) {
    println!("The size is {}", std::mem::size_of::<T>());
}

fn main() {
    let val = noop(0u32);
    show_size(val);
}
