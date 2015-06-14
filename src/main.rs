extern crate type_printer;

fn main() {
    println!("\nMessin' with String\n");
    exploring_str1();
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

