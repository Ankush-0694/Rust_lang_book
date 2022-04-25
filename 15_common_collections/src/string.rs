pub fn run() {
    // string are stored as a collection of UTF

    // creating a new string
    let s1 = String::new();

    let s2 = "example string"; // string literals

    let s3 = s2.to_string();

    let s4 = String::from("hello world");

    println!("s1: {:?},s2: {:?}, s3: {:?}, s4: {:?} ", s1, s2, s3, s4);

    // pushing to string

    let mut new_string = String::from("Hlw, ");

    let name = "ankush";
    new_string.push_str(name);

    println!("{}", name); // name is printed after pushing to new string it means that push_str does not take the ownership

    println!("String after push_str: {:?}", new_string);
    new_string.push('!');

    println!("String after pushing character : {:?}", new_string);

    let formatted_concat_string = format!("{}{}{}", "a", "b", "c"); // it also does not take ownership
    println!("concated string : {:?}", formatted_concat_string);

    // accessing a char from string is different here

    let hello = String::from("hello");

    /* this will give an error because we are using UTF-8 ,
    in some other languate some char can be 2 bytes */
    // let character = hello[0];

    // printing bytes for every char
    for i in hello.bytes() {
        println!("{}", i);
    }

    // printing scaler values
    for i in hello.chars() {
        println!("{}", i);
    }
}
