use primrose::generator::{run};
use std::env;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { // skip the first arg
        let _ = run("./spec_code/example_unique.rs".to_string(), "default".to_string(), 5);
        Ok(())    
    } else if args.len() == 4 { // skip the first arg
        let model_size_input = match args.get(3).unwrap().parse::<u64>() {
            Ok(val) => val,
            Err(_) => {
                println!("here");
                return Err(Error::new(ErrorKind::Other, "Invalid model size"));
            }
        };
        let model_size = model_size_input as usize;
        let _ = run("./spec_code/".to_string() + args.get(1).unwrap(), args.get(2).unwrap().to_string(), model_size);
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Invalid source code paths"))
    }
}