fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    drop(s3);

    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");

    change(&mut s);

    let mut s4 = String::from("hello");

    let r1 = &s4;
    let r2 = &s4;
    println!("{r1} and {r2}");

    let r3 = &mut s4;
    println!("{r3}");

    let mut s5 = String::from("hello world");

    let word = first_word(&s);

    s.clear();

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
