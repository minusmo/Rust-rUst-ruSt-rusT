fn main() {
    // This is an array
    let array: [i32; 3] = [1,2,3];
    // This is a slice
    let slice: [i32; 500] = [1; 500];

    for i in 0..array.len() {
        println!("{}",array[i]);
    }
}