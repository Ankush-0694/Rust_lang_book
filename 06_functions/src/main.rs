fn main() {
    another_function();
    another_function_with_param(5);

    // expressions example
    let y = {
        let x = 3;
        x + 1   // note that we have not put semicolon here that is why it will be evaluate to 4 and return it and store it in y
    };
    println!("The value of y is: {}", y);


    let sumOftwoIntegers = sum(10,15);

    println!("The value of sumOftwoIntegers is: {}", sumOftwoIntegers);

}

fn another_function(){
    println!("this is function without parameters");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn sum(a: i32 , b: i32) -> i32{
    a + b
}
/* 

must understand statement vs expressions

let y = 6; // statement -> they do not return values

let x = (let y = 6); // -> we can not do that because y=6 is a statement


*/