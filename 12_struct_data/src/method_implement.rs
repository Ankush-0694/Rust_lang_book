#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// this is method implementation
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


pub fn run(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}