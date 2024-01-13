use rand::Rng;

pub fn entrypoint() {
    match exists_or_not() {
        Some(text) => println!("{text}"),
        None => {}
    };

    let one_of_two: Result<i32, String> = one_of_two();
    match one_of_two {
        Ok(value) => println!("{value}"),
        Err(value) => println!("{value}"),
    }

    match my_own_type() {
        MySumType::Empty => {}
        MySumType::Numbers(x, y) => println!("Numbers: {x}, {y}"),
        MySumType::Text(t) => println!("Text: {t}"),
        MySumType::Complex(c) => {
            let displayable = c
                .value
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",");
            
            println!("Complex: {0}", displayable)
        }
    }
}

fn exists_or_not() -> Option<String> {
    let mut rng = rand::thread_rng();

    match rng.gen_bool(0.5f64) {
        true => Some("I exists".into()),
        false => None,
    }
}

fn one_of_two() -> Result<i32, String> {
    let mut rng = rand::thread_rng();

    match rng.gen_bool(0.5f64) {
        true => Ok(42),
        false => Err("My name is Ken, I am an error".into()),
    }
}

enum MySumType {
    Empty,
    Numbers(f64, f32),
    Text(String),
    Complex(MyProductType),
}

struct MyProductType {
    value: Vec<u8>,
}

fn my_own_type() -> MySumType {
    let mut rng = rand::thread_rng();

    match rng.gen::<u8>() {
        0..=10 => MySumType::Empty,
        number @ 11..=100 => MySumType::Numbers(number as f64, 42f32),
        101..=200 => MySumType::Text("I am Ken, I am not an error!".into()),
        201.. => MySumType::Complex(MyProductType { value: vec![42] }),
    }
}
