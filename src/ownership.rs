pub fn ownership() {
    {
        let _s = "hello";
    }
    //s out of scope
    //println!("{s}");

    let s1 = String::from("hello"); //String is mutable datatype stored on heap memory
    let s2 = s1; //duplicate pointer to same position in heap
    //after line above, rust considers s1 as invalide to prevent double pointer and double freeing of same heap memory
    //println!("s1 is {s1} and s2 is also {s2}"); //this line is invalid

    let s3 = s2.clone();

    println!("s2 = {s2} and s3 is a clone of s3={s3}");

    let s1 = gives_ownership();
    let s2 = takes_and_returns_ownership(s1);
    println!("s2 is: {s2}");

    let mut string = String::from("HELLO");
    let len = calculate_length_with_borrow(&string);
    println!("The length of string is {len}");

    let r1 = &mut string;
    //let r2: &mut String = &mut string; //illegal second mutable reference to string
    change_string_with_mutable_reference(r1);
    println!("r1 is a mutable reference of string, which is now: {string}");

    let words = String::from("Simon is smart");
    let f = first_word(&words);
    println!("The first word is {f}");

    let a = [1, 2, 3, 4, 5];
    println!("Array elements 1 to 3 are: {:?}", &a[0..3])
}

fn gives_ownership() -> String {
    let my_string = String::from("take this string");
    my_string
}

fn takes_and_returns_ownership(my_string: String) -> String {
    my_string
}

fn calculate_length_with_borrow(string: &String) -> usize {
    string.len()
}

fn change_string_with_mutable_reference(string: &mut String) {
    string.push_str("!");
}

fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }
    &string[..]
}
