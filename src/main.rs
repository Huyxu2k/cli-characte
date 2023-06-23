use clap::{Parser, Command};
use cli_character::{structure::cliS::{Cli, Commands}, call::stringer};

fn main() {
    let cli=Cli::parse();
    
    match &cli.command {
        Some(Commands::Reverse(name)) => {
            match name.string {
                Some(ref _name)=>{
                    let reverse=stringer::reverse(_name);
                    println!("{}", reverse);
                },
                None=>{
                    print!("Please provide a string to reverse")
                }
            }
        },
        Some(Commands::Inspect(name))=>{
            match name.string {
                Some(ref _name) => {
                    let (res,kind)=stringer::inspects(_name, name.only_digits);

                    let mut p_s="s";
                    if res.0==1{
                        p_s="";
                    }
                    println!("{:?} has {} {}{} {}.", _name, res.0, kind, p_s,res.1);
                },
                None => { println!("Please provide a String to inspect")},
            }
        },
        None => {},
    }
}
