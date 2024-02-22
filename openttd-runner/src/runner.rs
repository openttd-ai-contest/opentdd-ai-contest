use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;
use std::process::Command;
use std::process::Child;
use std::process::Stdio;
use std::convert::TryFrom;
use std::fs;

use tempfile::TempDir;
use fs_extra::dir;
use ini::Ini;
use regex::Regex;

pub struct RunResult {
    player_results: Vec<PlayerResult>,
    savefile: Vec<u8>,
    log: Vec<String>,
}

impl RunResult {
    pub fn new(player_names: Vec<String>) -> RunResult {
        let mut player_results = Vec::with_capacity(player_names.len());
        for player_name in player_names {
            player_results.push(PlayerResult{
                name: player_name,
                corporation_name: String::new(),
                log: Vec::with_capacity(1024),
                stats: Vec::with_capacity(40) // TODO calculate based on game length
            })
        }
        return RunResult{
            player_results: player_results,
            savefile: Vec::new(),
            log: Vec::with_capacity(1024),
        };
    }
}

pub struct PlayerResult {
    name: String,
    corporation_name: String,
    log: Vec<String>,
    stats: Vec<(String, HashMap<String, i64>)>
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
        .set("Logger GS", "debug_level=1,end_year=1976"); // TODO support configurable game length
    for player_name in player_names {
        config.with_section(Some("ai_players")).set(player_name, "start_date=0");
    }

    config.write_to_file(config_file)
        .expect(&std::format!("Unable to write extended config file: {}", config_file.display()));
}

fn run_openttd(work_dir: &Path, player_names: Vec<String>) -> RunResult {
    let mut child = spawn_openttd_process(work_dir);
    let mut result = RunResult::new(player_names);
    let mut stdin = child.stdin.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let script_log_parser: Regex = Regex::new(r"^dbg: \[script\] \[(?P<script_id>\d+)\] (?P<log>.+)$").unwrap();
    let lines = BufReader::new(stderr).lines();
    for line in lines.map(|l| l.unwrap()) {
        println!("{}", line);
        const SCRIPT_LOG_MARKER: &str = "dbg: [script]";
        if line.starts_with(SCRIPT_LOG_MARKER) {
            let captures = script_log_parser.captures(line.as_str())
                .expect(&std::format!("Unable to parse log line: {}", line));
            let script_id: usize = captures["script_id"].parse()
                .expect(&std::format!("Unable to parse script_id from line: {}", line));
            let log = &captures["log"];
            if script_id < result.player_results.len() {
                result.player_results[script_id].log.push(log.to_string());
            } else if script_id == 18 { // TODO figure out why 18 and if it is stable
                if log == "[I] done" {
                    stdin.write("exit\n".as_bytes()).expect("Failed to send command to OpenTTD binary");
                    stdin.flush().expect("Failed to send command to OpenTTD binary");
                    break;
                } else if log.starts_with("[I] date=") {
                    let (stats, corporation_name) = split_corporation_name(log.strip_prefix("[I] ").unwrap());
                    let (date, stats) = split_date_and_parse_stats(stats);
                    let cid = stats.get("cid")
                        .expect(&std::format!("Unable to parse logger line: {}", log)).clone();

                    let player_result = &mut result.player_results[usize::try_from(cid).unwrap()];
                    player_result.stats.push((date, stats));
                    if player_result.corporation_name.is_empty() {
                        player_result.corporation_name = corporation_name.to_string();
                    } else {
                        assert!(
                            player_result.corporation_name == corporation_name,
                            "Player {} corporation name changed from {} to {}",
                            cid, player_result.corporation_name, corporation_name);
                    }
                }
            }
        }
        result.log.push(line);
    }
    child.wait().expect("OpenTTD binary didn't terminate successfully");
    result.savefile = read_latest_save(work_dir);
    return result;
}

fn spawn_openttd_process(work_dir: &Path) -> Child {
    let config_path = fs::canonicalize(work_dir.join("openttd.cfg")).unwrap();
    return Command::new("./openttd")
        .args(["-D", "localhost"])
        .args(["-d", "script=4"])
        .args(["-t", "1975"])
        .args(["-G", "1"])
        .args(["-p", "12345"]) // TODO generate password
        .arg("-c")
        .arg(&config_path.into_os_string())
        .current_dir(work_dir)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run openttd binary");
}
fn split_corporation_name(log: &str) -> (&str, &str) {
    const NAME_MARKER: &str = ",name=";
    return log.split_once(NAME_MARKER)
        .expect(&std::format!("Unable to parse logger line: {}", log));
}

fn split_date_and_parse_stats(log: &str) -> (String, HashMap<String, i64>) {
    let mut values = log.split(",");

    let (key, date) = split_value(values.next().unwrap());
    assert!(key == "date", "Unable to parse logger line: {}", log);

    return (date.to_string(), values
        .map(split_value)
        .map(|(k, v)| (k, v.parse::<i64>().unwrap()))
        .fold(HashMap::new(), |mut stats, (k, v)| {
            stats.insert(k.to_string(), v);
            return stats;
        }));
}

fn split_value(value: &str) -> (&str, &str) {
    return value.split_once("=")
        .expect(&std::format!("Unable to parse logger value: {}", value));
}

fn read_latest_save(work_dir: &Path) -> Vec<u8> {
    let save_dir = work_dir.join("save").join("autosave");
    let saves = fs::read_dir(save_dir.as_path())
        .expect(&std::format!("Unable to read save directory: {}", save_dir.display()));
    let latest_save = saves
        .map(|s| s.unwrap())
        .max_by_key(|save| save.metadata().unwrap().created().unwrap()).unwrap();

    let mut file = fs::File::open(latest_save.path())
        .expect(&std::format!("Unable to read save file: {}", latest_save.path().display()));
    let mut data: Vec<u8> = Vec::with_capacity(
        usize::try_from(latest_save.metadata().unwrap().len()).unwrap());
    file.read_to_end(&mut data)
        .expect(&std::format!("Unable to read save file: {}", latest_save.path().display()));
    return data;
}