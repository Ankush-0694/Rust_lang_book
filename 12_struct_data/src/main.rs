mod method_implement;


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}



// tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);


fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);


    // whole structure can be mutable not a single field
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); 

    println!("{}", user1.email);


    let black = Color(0, 0, 0);

    println!("{:#?}", black);


    method_implement:: run();

}

// // we can use shorthand for same key and value pair -> not calling this function yet
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}