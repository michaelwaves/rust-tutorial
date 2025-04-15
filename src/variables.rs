pub fn variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let decimal = 88_44;
    let hex = 0xffc;
    let octal = 0o77;
    let binary = 0b1111_0000;
    //let architecture_number: usize = 64;

    println!("binary: {binary}");
    println!("decimal: {decimal}");
    println!("hex: {hex}");
    println!("octal: {octal}");
    println!("binary: {binary}");

    //const SPEED_OF_LIGHT: i32 = 100000000;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is:{x}");

    let sum = 1 + 1;
    let difference = hex - binary;
    let product = octal * hex;
    let modulo = hex % binary;
    let quotient = hex / binary;

    println!("sum: {sum}");
    println!("difference: {difference}");
    println!("modulo:{modulo}");
    println!("quotient:{quotient}");
    println!("product: {product}");

    let char_cat = '🐈';
    println!("Char: {char_cat}");

    let binary_2 = 0b1010_1010;
    let xor = binary ^ binary_2;
    println!("XOR: {xor:#b}");

    let tup = (500, 3.3, String::from("ABC"));
    println!("Tuple: {tup:?}");

    let array = [1, 2, 3, 4, 5, 6];
    println!("Array: {array:?}");
}
