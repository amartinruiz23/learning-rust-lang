fn main() {
    let s = String::from("hello world !!!");
    let string_literal = "have a nice day";

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: {}", hello);
    println!("world: {}", world);

    let word = first_word(&s);
    let other_word = second_word(&s);
    println!("first word: {}", word);
    println!("second word: {}", other_word);

    let word = first_word(string_literal);
    let other_word = second_word(string_literal);
    println!("first word: {}", word);
    println!("second word: {}", other_word);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first = false;
    let mut beginning = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if !first{
                first = true;
                beginning = i + 1;
            }
            else{
                return &s[beginning..i];
            }

        }
    }

    &s[beginning..]
}
