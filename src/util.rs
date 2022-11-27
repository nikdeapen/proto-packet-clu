/// Gets the single path argument from the args.
pub fn get_single_path_arg<'a>(
    command: &str,
    _path: &str,
    args: &'a [String],
) -> Result<&'a str, String> {
    if args.is_empty() {
        Err(format!("no path given for command: {}", command))
    } else if args.len() > 1 {
        Err(format!(
            "unrecognized args for command: {} args: {:?}",
            command,
            &args[1..]
        ))
    } else {
        Ok(args[0].as_str())
    }
}
