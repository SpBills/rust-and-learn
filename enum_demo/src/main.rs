enum SimpleValues {
    A,
    B,
}

enum WithData {
    A(i32),
    B { x: i32, y: i32 },
    C,
}

fn main() {
    let simple = SimpleValues::A;

    match simple {
        SimpleValues::A => println!("Hello, world!"),
        SimpleValues::B => panic!(),
    }

    let with_data = WithData::A(50);

    match with_data {
        WithData::A(x) => println!("{}", x),
        WithData::B { x, y } => println!("{} {}", x, y),
        WithData::C => panic!(),
    }

    let option = Option::Some(50i32);

    match option {
        Some(x) => println!("{}", x),
        None => panic!(),
    }

    let x = if true {
        Result::Ok(50)
    } else {
        Result::Err("true == false")
    };

    match x {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    }
}
