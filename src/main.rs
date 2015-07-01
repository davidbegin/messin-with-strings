#![allow(dead_code, unused_variables)]

extern crate type_printer;
mod str_vs_string;
mod struct_warm_ups;

fn main() {
    println!("\nMessin' with String\n");

    // exploring_str1();
    // exploring_str2();

    // str_vs_string::tell_me_more();
    struct_warm_ups::twenty_reps();
}

// Two types &str, and String

fn exploring_str1() {
    let string = "la la la";
    type_printer::print_type_of(&string);
    // => &'static str
    //
    // so all strings by default have a static lifetime?
    // where is the memory for these static strings allocated?
}

// it says that strings are statically allocated, does this imply it is saved on
// the stack?
//
// that doesn't feel write, because isn't the stack 
//
// in static storage: a string literal "foo" is a &str, where the data is hardcoded into the
// executable and loaded into memory when the program runs.
//
// what does this mean is hardcoded into the executeable
//
// helpful SO http://stackoverflow.com/questions/24158114/rust-string-versus-str

// "In summary, use String if you need owned string data (
// like passing strings to other tasks, or building them at runtime),
// and use &str if you only need a view of a string.

// (This is identical to the relationship between a vector Vec<T> and a slice &[T],
// and is similar to the relationship between by-value T and by-reference &T for general types.)"

// https://en.wikipedia.org/wiki/Static_memory_allocation


// I did not realize this
// Strings will coerce into &str with an &:

// Viewing a String as a &str is cheap,
// but converting the &str to a
// String involves allocating memory.
// No reason to do that unless you have to!


fn exploring_str2() {
   // So I can push a &str onto a mutabale String?
   let mut name = "Same".to_string();
   name.push_str(" stuff");
   println!("&str pushed onto String: {}", name);

   // can I make a mutable string from a nonmutable,
   // can I clone it and make the clone mutable?

   let cannot_change_me = "Unmoveable".to_owned();
   let mut changeable = cannot_change_me.clone();
   changeable.push_str(" something new");
   println!("Clone to the rescue: {:?}", changeable);
}
