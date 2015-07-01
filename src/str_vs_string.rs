use std::rc::Rc;
use std::sync::Arc;

pub fn tell_me_more() {
    println!("...str versus String...tell me more");
    // tale_of_four_strings();
    // string_and_structs();
    // str_vs_string_confusion();
}

fn str_vs_string_confusion() {
    struct Person {
        name: String,
    }

    impl Person {
        fn greet(&self) {
            println!("Hello, I'm {}", self.name);
        }
    }

    let name = "Sammy".to_string();
    let sammy = Person { name: name };
    sammy.greet();
    // println!("Name on the outside: {}", name); // BOOM MOVE ERROR

    struct Person2<'a> {
        name: &'a str,
    }

    impl<'a> Person2<'a> {
        fn greet(&self) {
            println!("Hello I'm a person {}, and I'm using lifetimes!", self.name);
        }
    }

    let name = "Sammy";
    let crazy_sammy = Person2 { name: name };
    crazy_sammy.greet();
    println!("Name on the outside! {}", name);
}

fn string_and_structs() {
    let bill = Person { name: "Bill" };
    bill.greet();
}

struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn tale_of_four_strings() {
    let message = "hello world";
    print_me(message);

    // WHA WHA WHA
    //
    // how was I missing that
    //
    // if the compile sees a borrowed String
    // and the function requires a borrowed slice of string
    //
    // the compiler coerces the &String to &str
    //
    // voodoo
    let owned_string = "I own this, this is mine".to_string();
    print_me(&owned_string);

    // So I've never used this before
    let counted_string = Rc::new("hey mang count me in!".to_string());
    print_me(&counted_string);

    // Never ever used this before!
    let auto_counted_string = Arc::new("sorry even this crazy Arc strang");
    print_me(&auto_counted_string);
}

fn print_me(msg: &str) {
    println!("msg = {}", msg);
}

// DUMB IMPLEMENTATION
// fn print_me(msg: String) {
//     println!("the message is {}", msg);
// }
