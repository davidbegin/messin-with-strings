use std::borrow::Cow;

pub fn lemme_see_dat() {
    println!("\nStep right up! and see the amazing function, that returns either a &str or String!");

    let messy_strang = "su pe rca li fragilistice xpia lid oci ous";
    let result = remove_spaces(messy_strang);
    println!("Result: {}", result);

    let clean_strang = "supercalifragilisticexpialidocious";
    let result = remove_spaces(clean_strang);
    println!("Result: {}", result);
}

fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
  // do these have to be single quotes?!
  if input.contains(' ') {
    let mut buf = String::with_capacity(input.len());

    for c in input.chars() {
      if c != ' ' {
        buf.push(c);
      }
    }

    return Cow::Owned(buf);
  }

  return Cow::Borrowed(input);
}

fn og_remove_spaces(input: &str) -> String {
  let mut buf = String::with_capacity(input.len());

  for c in input.chars() {
    if c != ' ' {
      buf.push(c);
    }
  }
  buf
}
