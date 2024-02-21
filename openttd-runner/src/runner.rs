use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;
use std::os::unix::fs;

use tempfile::TempDir;
use fs_extra::dir;

pub struct RunResult {
    player_results: Vec<PlayerResult>,
    savefile: String,
    log: String,
}

pub struct PlayerResult {
    name: String,
    corporation_name: String,
    log: String,
    stats: Vec<(String, HashMap<String, i64>)>,
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

    fs::symlink(
        openttd_dir.join("game").join("Logger GS"),
        tmp_dir.join("game").join("Logger GS"));
}