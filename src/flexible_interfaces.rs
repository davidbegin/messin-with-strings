pub fn str_vs_string() {
    println!("\nI'm going to make a function that takes both &str and String");
    println!("=============================================================\n");

    // use a String for a Struct, when the struct
    // needs to own the attribute

    let bill = Person::new("bill");

    // let sammy = Person::new("sammy".to_string()); // Won't compile!

    // So we have to call as_ref() on the String
    let name = "Sammy".to_string();
    let sammy = Person::new(name);
}

struct Person {
    name: String,
}

// So now we accept generics that have the trait Into
// implemented for coverting to a String
impl Person {
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

// The problem with this implementation
// is if we pass a String to the constructor,
// then it won't compile, even though we want name to be a String
// impl Person {
//     fn new(name: &str) -> Person {
//         Person { name: name.to_string() }
//     }
// }
