extern crate type_printer;

pub fn still_no_mans_land() {
    println!("Need to try this again");
    println!("======================\n\n");

    ok_ok();
}

fn ok_ok() {
    let real = 4;
    let reference = &4;

    type_printer::print_type_of(&real); // i32
    type_printer::print_type_of(&reference); // &'static i32'

    // so val here can be anything, as long as the matching is a reference
    match reference {
        &val => println!("val: {}", val)
    }


    let reference2 = &5;
    match reference {
        &akjfhlkajh => println!("val: {}", akjfhlkajh)
    }

    // I can deference before I do the match,
    // but why is everything matched?

    let reference3 = &6;

    match *reference3 {
        got_to_see_this => println!("val: {}", got_to_see_this)
    }

    let ref reference4 = 7;

    // wait this matches also?!
    // match reference4 {
    //     word => println!("val: {}", word)
    // }

    match reference4 {
        &word => println!("val: {}", word)
    }

    let reference5 = &8;

    // So here I matched the value and then turned it back to a reference
    match reference5 {
        ref val => type_printer::print_type_of(val)
    }

    // I can also mut said variable

    let mut reference6 = 9;

    match reference6 {
        ref mut val => {
            *val += 1;
            println!("{}", val);
        }
    }
}

// From Rust by Example

// For pointers, a distinction needs to be made between destructuring and dereferencing
//  as they are different concepts which are used differently from a language like C.
//
// Dereferencing uses *
// Destructuring uses &, ref, and ref mut

fn thief() {

    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference`s is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exacty this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
