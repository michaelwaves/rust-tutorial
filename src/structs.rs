pub fn structs() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let michael = User {
        active: true,
        username: String::from("michaelwaves"),
        email: String::from("michael@quantoflow.com"),
        sign_in_count: 1,
    };

    println!("Username: {}", michael.username);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let simon = build_user(
        String::from("simonyu.1214@gmail.com"),
        String::from("simonseal"),
    );

    println!("Simon username: {}", simon.username);

    let simon2 = User {
        email: String::from("pikachus@gmail.com"),
        ..simon
    };

    println!("Simon 2 email: {}", simon2.email);

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle {
        width: 100,
        height: 50,
    };

    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    println!("Rectangle: {:?}", rect);
    println!("rect's area is: {}", rect.area());
}
