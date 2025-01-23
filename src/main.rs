mod ksuid;
mod snowflake_id;

use snowflake_id::SnowflakeIdGenerator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [-snowflakeid | -ksuid]", args[0]);
        return;
    }

    match args[1].as_str() {
        "-snowflakeid" => {
            let mut generator = SnowflakeIdGenerator::new();
            let id = generator.generate();
            println!("{}", id);
        }
        "-ksuid" => {
            let id = ksuid::generate_ksuid();
            println!("{}", id);
        }
        _ => {
            eprintln!("Invalid option: {}. Use -snowflakeid or -ksuid.", args[1]);
        }
    }
}
