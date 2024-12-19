use std::env;
mod lizt;
use lizt::{LzConf, LzList, LzError};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };
    let lzconf = LzConf::new(path.to_string());
    match lizt::lizt(&lzconf) {
        Ok(LzList { entries }) => {
            for entry in entries {
                println!("{}", entry);
            }
        },
        Err(LzError::Io(e)) => println!("IO Error: {}", e),
        // Err(LzError::Other(e)) => println!("Other Error: {}", e)
    }
}