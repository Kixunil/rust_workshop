fn main() {
    let imut_str: &str = "immutable string";
    println!("imut_str: {}", imut_str);
    let mut owned: String = imut_str.to_owned();
    println!("owned_str: {}", owned);
    owned.push_str(" trololo");

    println!("owned_str: {}", owned);

    let joined = ["Orange", "coin", "good"].join(" ");

    println!("joined: {}", joined);

    let mut path = std::path::PathBuf::from("/etc");
    path.push("tor");

    println!("path: {}", path.display());
}
