/// Reads the next line from stdin, automatically removing the \n or \r\n at the end
pub fn read_line() -> String {
    use std::io::BufRead;

    let mut line = String::new();

    let stdin = ::std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap(); // TODO: Maybe handle errors
	let trim: &[_] = &['\n', '\r'];

    line.trim_right().to_string()
}

/// Display a standard error message when an invalid command is entered.
pub fn invalid_command(cmd: &str) {
    println!(
        "The command '{}' is invalid. Type 'help' to see a list of possible commands.",
        cmd
    );
}
