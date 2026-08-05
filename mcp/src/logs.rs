//! Tail / grep host and docker logs (size-capped).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::UNIX_EPOCH;

use regex::Regex;

use crate::paths::{
    assets_root, container_name_for_profile, repo_root, MAX_TEXT_BYTES,
};

enum LogSource {
    Docker,
    File(PathBuf),
}

fn resolve_source(
    source: &str,
    node_id: Option<u32>,
) -> Result<LogSource, String> {
    let root = assets_root();
    match source.trim().to_ascii_lowercase().as_str() {
        "docker" | "container" => Ok(LogSource::Docker),
        "assets_stdout" | "stdout" | "nctl_stdout" => {
            Ok(LogSource::File(root.join("logs").join("stdout.log")))
        }
        "sidecar" | "sidecar_stdout" => Ok(LogSource::File(
            root.join("logs").join("sidecar-stdout.log"),
        )),
        "node" | "node_stderr" | "node_stdout" => {
            let id = node_id.ok_or_else(|| "node_id required for source=node".to_string())?;
            let node_logs = root.join("nodes").join(format!("node-{id}")).join("logs");
            let stderr = node_logs.join("stderr.log");
            let stdout = node_logs.join("stdout.log");
            if stderr.is_file() {
                Ok(LogSource::File(stderr))
            } else if stdout.is_file() {
                Ok(LogSource::File(stdout))
            } else {
                Ok(LogSource::File(stderr))
            }
        }
        _ => Err("source must be docker | assets_stdout | sidecar | node".into()),
    }
}

fn cap_text(mut text: String) -> String {
    if text.len() > MAX_TEXT_BYTES {
        text = text[text.len() - MAX_TEXT_BYTES..].to_string();
    }
    text
}

fn tail_file(path: &Path, lines: usize) -> String {
    if !path.is_file() {
        return format!("missing file: {}", path.display());
    }
    let out = Command::new("tail")
        .args(["-n", &lines.to_string()])
        .arg(path)
        .output();
    let text = match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).into_owned(),
        _ => {
            let content = fs::read_to_string(path).unwrap_or_default();
            let all: Vec<&str> = content.lines().collect();
            let start = all.len().saturating_sub(lines);
            all[start..].join("\n")
        }
    };
    format!(
        "{} (last {lines} lines):\n{}",
        path.display(),
        cap_text(text)
    )
}

pub fn logs_list() -> String {
    let root = assets_root();
    let mut lines = vec![format!("Log inventory under {}:", root.display()), String::new()];
    let top = root.join("logs");
    if top.is_dir() {
        append_log_files(&mut lines, &root, &top);
    } else {
        lines.push(format!("(no {})", top.display()));
    }
    let nodes = root.join("nodes");
    if nodes.is_dir() {
        if let Ok(rd) = fs::read_dir(&nodes) {
            let mut node_dirs: Vec<_> = rd.filter_map(|e| e.ok()).collect();
            node_dirs.sort_by_key(|e| e.file_name());
            for e in node_dirs {
                let name = e.file_name();
                let name = name.to_string_lossy();
                if !name.starts_with("node-") || !e.path().is_dir() {
                    continue;
                }
                let logdir = e.path().join("logs");
                if logdir.is_dir() {
                    append_log_files(&mut lines, &root, &logdir);
                }
            }
        }
    }
    if lines.len() <= 2 {
        lines.push("(no log files yet — start the network first)".into());
    }
    lines.join("\n")
}

fn append_log_files(lines: &mut Vec<String>, root: &Path, dir: &Path) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    let mut files: Vec<_> = rd.filter_map(|e| e.ok()).collect();
    files.sort_by_key(|e| e.file_name());
    for e in files {
        let p = e.path();
        if !p.is_file() {
            continue;
        }
        let rel = p.strip_prefix(root).unwrap_or(&p);
        let meta = fs::metadata(&p).ok();
        let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
        let mtime = meta
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        lines.push(format!(
            "- {} size={size} mtime={mtime}",
            rel.display()
        ));
    }
}

pub fn logs_tail(source: &str, lines: u32, node_id: Option<u32>, profile: &str) -> String {
    let lines = (lines as usize).clamp(1, 500);
    let resolved = match resolve_source(source, node_id) {
        Ok(r) => r,
        Err(e) => return e,
    };
    match resolved {
        LogSource::Docker => {
            let name = container_name_for_profile(profile);
            let out = Command::new("docker")
                .args(["logs", "--tail", &lines.to_string(), &name])
                .current_dir(repo_root())
                .output();
            match out {
                Ok(o) => {
                    let mut text = String::from_utf8_lossy(&o.stdout).into_owned();
                    text.push_str(&String::from_utf8_lossy(&o.stderr));
                    if !o.status.success() && text.trim().is_empty() {
                        return format!(
                            "docker logs failed for {name} ({})",
                            o.status.code().unwrap_or(1)
                        );
                    }
                    format!("docker logs {name} (last {lines}):\n{}", cap_text(text))
                }
                Err(e) => format!("docker logs error: {e}"),
            }
        }
        LogSource::File(path) => tail_file(&path, lines),
    }
}

pub fn logs_grep(
    pattern: &str,
    source: &str,
    node_id: Option<u32>,
    profile: &str,
    max_matches: u32,
) -> String {
    let max_matches = (max_matches as usize).clamp(1, 200);
    let rx = match Regex::new(&format!("(?i){pattern}")) {
        Ok(r) => r,
        Err(e) => return format!("invalid regex: {e}"),
    };
    let resolved = match resolve_source(source, node_id) {
        Ok(r) => r,
        Err(e) => return e,
    };

    let candidates: Vec<String> = match resolved {
        LogSource::Docker => {
            let blob = logs_tail("docker", 400, None, profile);
            blob.lines().map(|s| s.to_string()).collect()
        }
        LogSource::File(path) => {
            if !path.is_file() {
                return format!("missing file: {}", path.display());
            }
            let data = fs::read(&path).unwrap_or_default();
            let slice = if data.len() > MAX_TEXT_BYTES * 4 {
                &data[data.len() - MAX_TEXT_BYTES * 4..]
            } else {
                &data[..]
            };
            String::from_utf8_lossy(slice)
                .lines()
                .map(|s| s.to_string())
                .collect()
        }
    };

    let hits: Vec<&String> = candidates.iter().filter(|ln| rx.is_match(ln)).collect();
    let shown = if hits.len() > max_matches {
        &hits[hits.len() - max_matches..]
    } else {
        &hits[..]
    };
    let mut out = format!(
        "grep /{pattern}/i source={source} matches={} shown={}",
        hits.len(),
        shown.len()
    );
    for ln in shown {
        out.push('\n');
        out.push_str(ln);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::test_env::ENV_LOCK;
    use std::env;

    #[test]
    fn grep_assets_stdout() {
        let _g = ENV_LOCK.lock().unwrap();
        let mut dir = env::temp_dir();
        dir.push(format!(
            "nctl-mcp-logs-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("assets/logs")).unwrap();
        fs::write(
            dir.join("assets/logs/stdout.log"),
            "hello\nERROR boom\nbye\n",
        )
        .unwrap();
        fs::create_dir_all(dir.join("docker")).unwrap();
        fs::write(dir.join("docker/docker-compose.yml"), "services: {}\n").unwrap();
        env::set_var("NCTL_DOCKER_ROOT", &dir);
        let out = logs_grep("error", "assets_stdout", None, "stable", 40);
        assert!(
            out.contains("ERROR boom"),
            "unexpected grep output: {out}"
        );
        env::remove_var("NCTL_DOCKER_ROOT");
        let _ = fs::remove_dir_all(&dir);
    }
}
