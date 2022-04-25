fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // passing reference instead of transfering ownership or move 

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");

    change(&mut s2);
}

// We call the action of creating a reference borrowing.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}



/* 

-> cannot borrow `s2` as mutable more than once at a time

let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

*/


/* 

let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
    

because user of immuatable reference do not expect to change their value 

*/


/* 

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

  let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);

    // variables r1 and r2 will not be used after this point
    // The scopes of the immutable references r1 and r2 end after the println!

    let r3 = &mut s; // no problem
    println!("{}", r3);



   

*/