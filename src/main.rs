use std::{env, process};

use filewatch::get_input_config;

mod processor;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let input_config = get_input_config(args);
    // 判断input_config是否成功
    if let Err(e) = input_config {
        eprintln!("Err: {}", e);
        process::exit(1);
    }

    let config = input_config?;

    println!("{:?}", config);

    Ok(())
}
