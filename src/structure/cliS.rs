use std::iter::Inspect;

use clap::Command;
use  ::clap::{Parser,Subcommand,Args};


#[derive(Parser)]
#[command(author,version)]
#[command(about="simple CLI")]
pub struct Cli{
    #[command(subcommand)]
    pub command:Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Reverse(Reverse),
    Inspect(Inspects),
    //CountNum(CountNum),
}

#[derive(Args)]
pub struct Reverse{
    pub string:Option<String>,
}

#[derive(Args)]
pub struct Inspects{
    pub string:Option<String>,
    #[arg(short='d',long="digits")]
    pub only_digits:bool,
}
