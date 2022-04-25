use std::collections::HashMap;

pub fn run() {
  // we are creating teams hashmap to map their scores

  let blue = String::from("blue");
  let yellow = String::from("yellow");

  let mut scores = HashMap::new();

  scores.insert(&blue, 20);
  scores.insert(&yellow, 40);

  // .get returns an option that is why we need to use custom specifier
  println!(
    "scores, for blue: {:?}, for yellow: {:?}",
    scores.get(&blue),
    scores.get(&yellow)
  );

  for (key, value) in &scores {
    println!("team {} has score - {}", key, value);
  }

  // insert only if  not exists
  scores.entry(&blue).or_insert(60); //-> or_insert returns an mutable reference so we can change that blue using the ref
  scores.entry(&blue).or_insert(80);

  // does not change because team blue already have the scores
  for (key, value) in &scores {
    println!("team {} has score - {}", key, value);
  }

  // updating a hashmap and changing value and solution to freq of a word

  let text = String::from("hello hello world world do nothing");

  let mut text_map = HashMap::new();

  for word in text.split_whitespace() {
    let count = text_map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", text_map);
}
