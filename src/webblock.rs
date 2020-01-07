use std::fs;
use std::process::Command;
use regex;

static HOSTS_COMMENT: &'static str = "#Added by trusty-timer";

fn find_hosts_path() -> String {
    let windows_path: String = "C:/Windows/System32/Drivers/etc/hosts".to_string();
    return windows_path;
}

fn flush_dns() {
    Command::new("cmd").args(&["/C", "ipconfig /flushdns"]);
}

pub fn read_hosts() -> String {
    let host_data = fs::read_to_string(find_hosts_path()).expect("Failed to read hosts file!");
    return host_data;
}

pub fn add_web_blocks() {
    let mut new_hosts = read_hosts();
    let block_list_raw = fs::read_to_string("blocklist.txt").expect("Failed to read block list file!");
    let block_list = block_list_raw.split("\n");
    for url in block_list.into_iter() {
        new_hosts += &format!("\n127.0.0.1 {} {}", url.trim(), HOSTS_COMMENT).to_owned();
        new_hosts += &format!("\n127.0.0.1 www.{} {}", url.trim(), HOSTS_COMMENT).to_owned();
        new_hosts += &format!("\n::1 {} {}", url.trim(), HOSTS_COMMENT).to_owned();
        new_hosts += &format!("\n::1 www.{} {}", url.trim(), HOSTS_COMMENT).to_owned();
    }
    fs::write(find_hosts_path(), new_hosts).expect("Failed to write hosts file!");
    flush_dns();
}

pub fn rm_web_blocks() {
    let old_hosts = &read_hosts().to_owned();
    let pattern = regex::RegexBuilder::new(&format!("^.*({})$", HOSTS_COMMENT).to_owned()).multi_line(true).build().unwrap();
    let new_hosts = (*pattern.replace_all(old_hosts, "")).to_string();
    fs::write(find_hosts_path(), &new_hosts.trim()).expect("Failed to write hosts file!");
    flush_dns();
}