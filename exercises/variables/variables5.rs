fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    number = "3";
    let x = number.parse::<i32>()
        .unwrap();
    println!("Number plus two is : {}", x + 2);
}
