use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    message: String,
    the_answer: u32,
}

fn main() -> Result<(), serde_json::Error> {
    let my_struct = MyStruct {
        message: "I will never write serialization code by hand again".to_owned(),
        the_answer: 42,
    };

    let string = serde_json::to_string(&my_struct).expect("This should never fail");
    println!("Serialized: {}", string);
    let deserialized = serde_json::from_str::<MyStruct>(&string)?;
    println!("Deserialized struct: {:?}", deserialized);

    Ok(())
}
