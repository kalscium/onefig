use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about="A mere test command")]
    Test,
    #[command(about="Compiles onefig-scripts into a single onefig-binary")]
    Compile {
        #[arg(index=2, num_args=1.., help="The onefig-scripts to compile.")]
        scripts: Vec<String>,
        #[arg(index=1, help="The output onefig-binary that is compiled.")]
        output: String,
    },
    #[command(about="Checks the validity of a onefig-script without executing or compiling it")]
    Check {
        #[arg(index=1, help="The onefig-scripts to check.")]
        scripts: String,
    },
    #[command(about="Executes a onefig script or binary")]
    Run {
        #[arg(short='u', long="unsafe", help="Stops onefig from caching the old configurations; disallowing for rollbacks.")]
        not_safe: bool,
        #[arg(short='s', long, help="Interprets the files as onefig scripts rather than binaries.")]
        is_script: bool,
        #[arg(index=1, help="The onefig scripts or binaries to execute.")]
        files: String,
    },
    #[command(about="Clears cache (configuration file history) (also disables rollbacks)")]
    ClearCache,
    #[command(about="Lists most of the configuration files in your system (unix only)")]
    Search,
    #[command(about="Rolls back to the state of the system's config-files before an execution")]
    Rollback {
        #[arg(short='s', long, help="Interprets the files as onefig scripts rather than binaries.")]
        is_script: bool,
        #[arg(short, long, help="The onefig script or binary to rollback on")]
        file: String,
    },
}