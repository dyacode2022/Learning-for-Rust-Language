fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("-------------------");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{}, {}, {}, {}, {}, {}", sum, difference, product, quotient, remainder, guess);

    println!("-------------------");

    let t: bool = true;
    println!("boolean var = {}", t);

    let c = 'z';
    println!("char var = {}", c);

    let tup = ("first", "secend", "thired");
    println!("one: {}\ntwo: {}\nthree: {}", tup.0, tup.1, tup.2);

    let array = [1, 2, 3, 4, 5];
    println!("array value: one:{} two:{} three:{} four:{} five:{}", array[0], array[1], array[2], array[3], array[4]);
}
