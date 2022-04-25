use std :: io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    // looping using -> Loop
    let mut x = 1;
    loop{
        println!("Value of x: {}", x);
        x = x+1;
        if x == 4 {
            break;
        }
    }

    // we can break the any loop by binding any keyword to any loop
    let x = 3;

    'breakOuterLoop: loop{
        let mut y = 1;
        loop{
            println!("Value of x: {}", x);
            println!("Value of y: {}", y);
            y = y+1;
            if y > x {
                break 'breakOuterLoop; // this break the outer loop instead of innermost loop
            }
        }
        x = x + 1;
    }

    // we can return the value from loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);




    // loop usign -> while
    println!("looping using while : ");
    let mut number = 3;

    while number != 0 {

        // these two line if we want to print output in a single line
        print!("{}, ", number);
        io::stdout().flush().unwrap();

        number -= 1;
    }
    println!("");

    // loop using -> for

    let arr = [1,2,3,4,5];

    for number in arr {
        println!("{}, ", number);
    }

}
