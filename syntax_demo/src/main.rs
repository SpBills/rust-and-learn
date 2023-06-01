use std::ops::Add;

struct HelloWorld {
    field1: i32,
    field2: Option<f64>,
}

struct NoFields();

struct AnonymousFields(i32, f32);

fn return_type() -> i32 {
    return 3;
}

fn immutable_by_default() {
    let x = 5;
    // x = 6;

    // Note that variables in the same scope can shadow earlier ones
    let mut x = 6i64;
    x = 7;
}

fn type_inferrence() {
    let mut v = Vec::new();

    v.push(100i32);
}

fn everythings_an_expression() {
    let x = {
        let v = return_type();
        let v = (0..v).map(|x| x * 2).collect::<Vec<_>>();

        v
    };

    println!("{}", x[1]);

    let g = if return_type() == 2 { 5 } else { 1 };

    println!("{}", g);

    let res = Add::add(
        5,
        match function_without_return() {
            2 => 5,
            _ => 1,
        },
    );

    println!("{}", res);

    let _weird = return immutable_by_default();
}

fn function_without_return() -> i32 {
    3
}

fn main() {
    println!("Hello, world!");
}
