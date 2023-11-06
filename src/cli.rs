use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long, help="Specifies if you want to compile the onefig scripts.")]
    pub compile: Option<Option<String>>,
    #[arg(short, long, help="Specifies if you want to execute/interpret the onefig scripts.")]
    pub interpret: bool,
    #[arg(short, long, num_args=1.., help="Onefig scripts or binaries that you want to compile, execute or interpret (determined by flags (if not flags then it executes onefig-binaries))")]
    pub scripts: Vec<String>,
}