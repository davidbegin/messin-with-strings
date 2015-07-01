use std::ascii::AsciiExt;

pub fn twenty_reps() {
    println!("\nSo I finally made a struct with an attribute of type &str");
    println!("...now its time to enforce that through some muscule memory");
    println!("===========================================================\n");

    // warm_up_one();
    // warm_up_two();
    warm_up_three();

    // char_aside();
    // break_dem_chars_down_and_sell_em();
}

fn char_aside() {
    // so chars need single quotes?
    let x: char = 'x';
    println!("char: {:?}", x.to_uppercase().next().unwrap());

    // does this mean strings and str need double,
    // and I just always use them, so I never noticed;
    //
    // ... YEP!
    // let y: &str = 'word'; // unterminated character constant: ';'
}

fn warm_up_three() {
    let meek_milly = Person { name: "Meek Mill" };

    all_caps(meek_milly);
}

fn all_caps(person: Person) {
    for c in person.name.chars() {
        print!("{}", c.to_uppercase().next().unwrap());
    }
}

fn warm_up_two() {
    let sam = Person { name: "Sam" };

    // So the Person Struct has a lifetime of person_lifetime
    // but the impl for Person has a lifetime of a
    //
    // but this works still
    sam.greet();
}

fn warm_up_one() {
    let name = "Bill";
    let bill = Person { name: name };
    println!("Person's name: {}", bill.name);
    println!("Person's name again: {}", name);
}

struct Person<'person_lifetime> {
    name: &'person_lifetime str,
}

impl<'a> Person<'a> {
    fn greet(&self) {
        println!("Hello: {}", self.name);
    }
}

// Explore later converting a &str all caps
// let result = strang.chars().map(|c| c.to_uppercase()).collect::<Vec<char>>();

fn break_dem_chars_down_and_sell_em() {
    let strang: &str = "commas";
    // for c in strang.chars() {
    //     println!("char: {}", c);
    // }
}

