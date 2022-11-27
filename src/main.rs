pub(crate) mod lex;
pub(crate) mod util;

/// ProtoPacket Command Line Utility Main
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some((path, rest)) = args.split_first() {
        if let Some((command, args)) = rest.split_first() {
            run_command(path, command.as_str(), args).await;
        } else {
            eprintln!("no command given");
        }
    } else {
        eprintln!("no path in args");
    }
}

/// Runs the command in the path.
async fn run_command(path: &str, command: &str, args: &[String]) {
    let result: Result<(), String> = match command {
        "lex" => lex::lex(path, args).await,
        _ => Err(format!("invalid command: {}", command)),
    };
    if let Err(message) = result {
        eprintln!("{}", message);
    }
}
