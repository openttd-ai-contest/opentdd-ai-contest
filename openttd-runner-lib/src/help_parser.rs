use std::vec::Vec;
use std::process::Command;
use std::path::Path;
use regex::Regex;

pub struct Ai {
    pub name: String,
    pub version: i64,
    pub description: String,
}

pub fn parse_ai_list(openttd_dir: &Path) -> Vec<Ai> {
    let help = Command::new("./openttd")
        .arg("--help")
        .current_dir(openttd_dir)
        .output()
        .expect("Failed to run openttd binary with --help");
    let output = String::from_utf8(help.stdout).unwrap();
    let lines = output.split("\n");
    let mut ai_list: Vec<Ai>  = Vec::new();
    let mut ai_list_started = false;

    let ai_line_parser = Regex::new(
        r"(?P<name>[^\s]+) \(v(?P<version>\d+)\): (?P<description>.+)$").unwrap();

    for line in lines {
        if !ai_list_started {
            ai_list_started = line == "List of AIs:";
        } else {
            if line.is_empty() {
                break;
            }
            let captures = ai_line_parser.captures(line)
                .expect(&std::format!("Unable to parse AI list help line: {}", line));
            ai_list.push(Ai{
                name: String::from(&captures["name"]),
                version: captures["version"].parse().unwrap(),
                description: String::from(&captures["description"])
            })
        }
    }
    return ai_list;
}