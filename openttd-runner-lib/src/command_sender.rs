use std::io::{BufReader, Lines, Write};
use std::process::{ChildStdin, ChildStdout};
use crate::random::random_password;

pub fn send_command(stdin: &mut ChildStdin,
                       stdout_reader: &mut Lines<BufReader<ChildStdout>>,
                       command: &str,
                       command_output_end_marker: Option<&str>) -> String {
    stdin.write(command.as_bytes()).unwrap();
    stdin.write("\n".as_bytes()).unwrap();
    stdin.flush().unwrap();

    println!("Sending command: {}", command);

    let end_marker;
    if command_output_end_marker.is_none() {
        end_marker = random_password();
        stdin.write(format!("echo \"{end_marker}\"\n").as_bytes()).unwrap();
        stdin.flush().unwrap();
    } else {
        end_marker = String::from(command_output_end_marker.unwrap());
    }

    let mut command_output = String::new();

    for line in stdout_reader.map(|l| l.unwrap()) {
        println!("OPENTTD STDOUT: {}", line);
        command_output.push_str(line.as_str());
        if line.contains(end_marker.as_str()) {
            break;
        }
    }

    return command_output;
}