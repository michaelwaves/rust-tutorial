pub fn enums() {
    enum IPAddress {
        V4(String),
        V6(String),
    }

    let four = IPAddress::V4;
    let six = IPAddress::V6;

    let localhost = IPAddress::V4(String::from("127.0.0.1"));
    let loopback = IPAddress::V6(String::from("::1"));

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32),
    }

    impl Message {
        fn call(&self) {
            println!("My message is {:?}", &self);
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    let none_number: Option<i32> = None;
    let some_char = Some('e');
    println!("This number should be None: {:#?}", none_number);

    #[derive(Debug)]
    enum EggSize {
        Small,
        Medium,
        Large,
        XL,
    }

    fn size_in_cm(es: EggSize) -> u8 {
        match es {
            EggSize::Large => 10,
            EggSize::Medium => 8,
            EggSize::Small => 6,
            _ => {
                println!("Unrecognized Size {:?}", es);
                0
            }
        }
    }

    let e = EggSize::Large;
    let size = size_in_cm(e);

    println!("The size of the egg is {size}")
}
