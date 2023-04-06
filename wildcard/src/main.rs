use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Selina Liu",
    about = "Perform wildcard matching on two strings"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Selina Liu")]
    Match {
        #[clap(short, long)]
        string1: String,
        string2: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Match { string1, string2 }) => {
            let result = wildcard::is_match(string1, string2);
            println!(
                "Given strings match result: {}",
                result
            );
        }
        None => println!("No subcommand was used"),
    }
}
