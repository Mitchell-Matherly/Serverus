//This program was created by Mitchell Matherly for the ConsenSys project. All liscences from the
//github repo apply.


mod wireg;


use std::env;
use std::process;

fn main() 
{

    let args:Vec<String> = env::args().collect();

    if args.len() < 2
    {
        eprintln!("whoops! you'll need to be more specific than that.");
        eprintln!("useage...");
        return
    }

    match args[1].as_str() 
    {
        "wireg" => wireg::route(&args[2..]),
    
        _=> eprintln!("unknown command: {}", args[1]),
    }
    


}
