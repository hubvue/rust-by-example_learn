fn main() {
  // Iterator::any

  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];
  // 闭包接受的参数类型为 &i32，因此需要解构
  println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
  // 闭包接受的是i32类型，因此无需解构
  // println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

  let array1 = [1,2,3];
  let array2 = [4, 5, 6];

  // 数组没有into_iter，因此使用iter() 闭包接受的类型为 &i32 因此需要解构。
  println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
  println!("2 in array2: {}", array2.iter().any(|&x| x == 2));

  // Iterator::find

  // 闭包接受的是&&i32类型，因此解构成i32类型
  println!("Find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
  println!("Find 2 in vec2: {:?}", vec2.iter().find(|&&x| x == 2));

  println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
  println!("Find 2 in array2: {:?}", array2.iter().find(|&&x| x == 2));
}
