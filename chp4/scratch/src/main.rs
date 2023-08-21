
fn main() {
    let s = String::from("howdy worlderino holla huetti");
    let t = "ooga booga looga";

    fn first_word(s: &str) -> &str {
      let bytes = s.as_bytes();

      for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
          return &s[..i];
        }
      }

      &s[..]
    }

    let word = first_word(&s);
    let worb = first_word(t);
    let slicer = first_word(&s[17..]);
    println!("first word is {}", word);
    println!("first worb is {}", worb);
    println!("first slicer is {}", slicer);

    println!(
      "&String={} &str={}",
      std::mem::size_of::<&String>(),
      std::mem::size_of::<&str>(),
    );

    let mut s = String::from("howdy");
    let s_ref = &s;

    println!("s={}", s);
    println!("s_ref={}", s_ref);
}