fn main() {
    let a: [i32; 500] = [0; 500];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}

