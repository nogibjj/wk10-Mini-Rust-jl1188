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
        s: String,
        #[clap(short, long)]
        p: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Match { s, p }) => {
            let result = wildcard::is_match(s, p);
            println!("Given strings match result: {}", result);
        }
        None => println!("No subcommand was used"),
    }
}
