pub mod my_functions {
    pub fn another_function(x: i32) {
        println!("The value of x is: {x}")
    }

    pub fn expressions() -> i32 {
        let y = {
            let x = 3;
            x + 4
        };
        y
    }

    pub fn control_flow() {
        let condition = true;
        let number = if condition { 5 } else { 6 };
        println!("Control flow number is: {number}");

        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The loop result is: {result}");

        let mut num = 3;
        loop {
            if num == 0 {
                break;
            } else {
                println!("The custom while loop number is {num}")
            }
            num -= 1;
        }

        num = 3;
        while num != 0 {
            println!("The custom while loop number is {num}");
            num -= 1;
        }

        let a = [1, 2, 3, 4, 5, 6];
        for i in a {
            println!("The value is {i}");
        }

        for i in (1..4).rev() {
            println!("{i}!")
        }
    }
}
