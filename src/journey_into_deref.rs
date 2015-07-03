use std::ops::Deref;

pub fn no_mans_land() {
    println!("I've heard about it, but never used it");

    let deref_enabled_char = DerefExample { value: 'a'};
    println!("{}", *deref_enabled_char);
    assert_eq!('a', *deref_enabled_char);

    let non_deref_enabled_char = NoDerefExample { value: 'a' };

    // Ok so I can't just reference a char without implementing
    // Deref
    // println!("{}", *non_deref_enabled_char); // won't compile
    // assert_eq!('a', *non_deref_enabled_char); // won't compile

    // I need to be more concious on when I working with a pointer
    // and when I am working with a value

    // I also need to try and fully grok the & *
    // and all its uses
}

struct NoDerefExample<T> {
    value: T,
}

struct DerefExample<T> {
    value: T,
}

// Never seen this type Target before
impl<T> Deref for DerefExample<T> {
    type Target = T;

    // So this is a method on the DerefExample struct
    // that returns the a borrowed Generic?

    // and it gets that borrowed generic,
    // but calling the &self.value
    // which I am very confused about
    fn deref(&self) -> &T {
        &self.value
    }
}

