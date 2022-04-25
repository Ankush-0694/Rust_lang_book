pub fn run(){
  let mut v1 : Vec<i32> = Vec:: new();

  v1.push(1);
  v1.push(2);
  v1.push(3);
  v1.push(4);
  v1.push(5);

  println!("{:?}", v1);


  let v2 = vec![1,2,3,4,];
  println!("{:?}", v2);


  /*  we are using refernce - so we are borrowing
    If we don't use reference then v1 will be moved and  we will not able to access the element afterwards
    */
  for i in &v1 {
      println!("Element is {}", i);
  }

  // accessing the elements from vectors
  let third_element = &v1[2]; // check last comments for understand this line
  println!("{}", third_element);




  // safe option to get element in vector because it also handle run time error -Like index out of bound
  // using option enum
  match v1.get(2){
    Some(element) => println!("The Element is : {}", element),
    None => println!("There is no element found"),
  }

}



/* 


- When you use `vectors`inside a `for..in` loop, Rust will call the `[IntoIterator::into_iter]
trait method of the `Vec`, which takes ownership of `self.`



- The [index operator] on the other hands, calls the
 `[Index::index] trait method of the `Vec.` 
This is why we can use &v[2] or v[2] does not matter

*/