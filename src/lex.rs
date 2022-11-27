use lex::{Lex, Lexer};

use crate::util;

/// Runs the lex command.
pub async fn lex(path: &str, args: &[String]) -> Result<(), String> {
    let path: &str = util::get_single_path_arg("lex", path, args)?;
    let source: String = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| e.to_string())?;
    let lex: Lex = Lexer::default().lex(source.as_str());
    println!("{}", lex);
    Ok(())
}
