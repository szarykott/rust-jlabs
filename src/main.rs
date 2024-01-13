use std::env;

mod borrow_checker;
mod types;
mod traits;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();
    
    match args.as_slice() {
        [_] => return Err(String::from("provide at least one argument")),
        [_, argument] => execute(&argument), 
        x => return Err(format!("too many arguemnts: {0}", x.join(",")))
    }
}

fn execute(arg: &str) -> Result<(), String> {
    match arg {
        "--borrow-checker" => borrow_checker::entrypoint(),
        "--types" => types::entrypoint(),
        "--traits" => traits::entrypoint(),
        _ => return Err("unknown workflow".into())
    }

    return Ok(());
}