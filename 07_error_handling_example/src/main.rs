// Predefined:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
//  }

use std::fs::File;

fn main() {
    let result = File::open("/dev/urandom");
    let result = match result {
        Ok(file) => process_file(file),
        Err(err) => {
            println!("Failed to open file: {}", err);
            std::process::exit(1);
        },
    };

    let val = result.expect("Lazy ass error handling");
    println!("Sum of random bytes: {}", val);
}

fn process_file(mut file: File) -> std::io::Result<u32> {
    use std::io::Read;

    let mut buf = [0; 42];
    // Same as:
    // match file.read_exact(&mut buf) {
    //     Ok(val) => val,
    //     Err(err) => return err.into(),
    // }
    file.read_exact(&mut buf)?;

    Ok(buf.iter().copied().map(u32::from).sum())
}
