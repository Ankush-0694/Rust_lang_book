fn main() {
    // characters
    let x = 'c';
    let y = 'ß';
    let z = '😻';
    println!("The value of x is: {}, The value of y is: {}, The value of z is: {}", x,y,z);


    // tuples
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of x is: {}, The value of y is: {}, The value of z is: {}", x,y,z);
    // OR
    let x = tup.0;
    println!("The value of x is: {}", x);

    //arrays
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let months = ["January", "February"];
    let a = [3; 5];  //  3 is value, 5 is length so it will contain [3,3,3,3,3]
    
    println!("{:?}", months);
    println!("{:?}", a);

}
