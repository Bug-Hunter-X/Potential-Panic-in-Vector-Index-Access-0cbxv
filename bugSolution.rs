fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;

    if index < vec.len() {
        println!("Value at index {}: {}", index, vec[index]);
    } else {
        println!("Index out of bounds");
    }
} 