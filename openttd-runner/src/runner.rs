use std::fmt::Debug;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;
use std::process::Command;
use std::process::Stdio;
use std::fs;

use tempfile::TempDir;
use fs_extra::dir;
use ini::Ini;
use regex::Regex;

pub struct RunResult {
    player_results: Vec<PlayerResult>,
    savefile: String,
    log: Vec<String>,
}

pub struct PlayerResult {
    name: String,
    corporation_name: String,
    log: Vec<String>,
    stats: Vec<HashMap<String, i64>>,
}

pub fn run_game(openttd_dir: &Path, player_names: Vec<String>) -> RunResult {
    let tmp_dir = TempDir::with_prefix("openttd-runner").expect("Failed to create a temporary directory");
    println!("Created temp directory: {}", tmp_dir.path().display());
    if cfg!(debug_assertions) {
        let path_buf = tmp_dir.into_path();
        return run_game_impl(openttd_dir, path_buf.as_path(), player_names);
    } else {
        return run_game_impl(openttd_dir, tmp_dir.path(), player_names);
    }
}

fn run_game_impl(openttd_dir: &Path, tmp_dir: &Path, player_names: Vec<String>) -> RunResult {
    dir::copy(
        openttd_dir,
        tmp_dir,
        &dir::CopyOptions::new().content_only(true))
    .expect(&std::format!(
        "Failed to copy openttd dir {} to tmp dir {}",
        openttd_dir.display(),
        tmp_dir.display()));
    extend_config(tmp_dir.join("openttd.cfg").as_path(), &player_names);
    return run_openttd(tmp_dir, player_names);
}

fn extend_config(config_file: &Path, player_names: &Vec<String>) {
    let mut config = Ini::load_from_file(config_file)
        .expect(&std::format!("Unable to parse config file: {}", config_file.display()));

    config.with_section(Some("gui"))
        .set("autosave", "yearly");
    config.with_section(Some("script"))
        .set("settings_profile", "hard");

    config.with_section(Some("game_scripts"))
        .set("Logger GS", "debug_level=0,end_year=1976"); // TODO support configurable game length
    for player_name in player_names {
        config.with_section(Some("ai_players")).set(player_name, "start_date=0");
    }

    config.write_to_file(config_file)
        .expect(&std::format!("Unable to write extended config file: {}", config_file.display()));
}

fn run_openttd(work_dir: &Path, player_names: Vec<String>) -> RunResult {
    let config_path = fs::canonicalize(work_dir.join("openttd.cfg")).unwrap();
    let mut child = Command::new("./openttd")
        .args(["-D", "localhost"])
        .args(["-d", "script=4"])
        .args(["-t", "1975"])
        .args(["-G", "1"])
        .args(["-p", "12345"]) // TODO generate password
        .args(["-c", &config_path.into_os_string().into_string().unwrap()])
        .current_dir(work_dir)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run openttd binary");

    let mut player_results = Vec::with_capacity(player_names.len());
    for player_name in player_names {
        player_results.push(PlayerResult{
            name: player_name,
            corporation_name: String::new(),
            log: Vec::with_capacity(1024),
            stats: Vec::with_capacity(40) // TODO calculate based on game length
        })
    }
    let mut result = RunResult{
        player_results: player_results,
        savefile: String::new(),
        log: Vec::with_capacity(1024),
    };
    let script_log_parser: Regex = Regex::new(r"^dbg \[script\] \[(?P<script_id>\d+)\] (?P<log>.+)$").unwrap();

    let stdout = child.stdout.take().unwrap();

    let lines = BufReader::new(stdout).lines();
    for line_result in lines {
        let line = line_result.unwrap();
        println!("{}", line);
        result.log.push(line.clone());

        const SCRIPT_LOG_MARKER: &str = "dbg: [script]";
        if line.starts_with(SCRIPT_LOG_MARKER) {
            let captures = script_log_parser.captures(&line)
                .expect(&std::format!("Unable to parse log line: {}", line));
            let script_id: usize = captures["script_id"].parse()
                .expect(&std::format!("Unable to parse script_id from line: {}", line));
            let log = &captures["log"];

            if script_id < result.player_results.len() {
                result.player_results[script_id].log.push(log.to_string());
            }
        }
    }

    return result;
}