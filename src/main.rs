use std::io;
fn main() {
    let arr = [1, 23, 4, 5, 6, 7];

    let mut index = String::new();
    let mut ele = 0;
    println!("enter the number ");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

   if  index < arr.len()-1 {
     ele = arr[index];
     println!("the element is {}", ele);
   }else {
       println!("out of boundaryyy");
   }
   

    // let mut count = 0;
    // loop {
    //     if count < arr.len() {
    //         println!("{}", arr[count]);
    //     }
    //     count += 1;

    //     if count > arr.len() {
    //         break;
    //     }
    // }

    //  for ele in &arr{
    //     println!("{}",ele);
    //  }
}
