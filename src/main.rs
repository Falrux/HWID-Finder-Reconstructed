use std::process::Command;
use std::error::Error;
use regex::Regex;
use sha1::{Sha1, Digest};

fn uuid() -> Result<String, Box<dyn Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ioreg -d2 -c IOPlatformExpertDevice")
        .output()?;

    if !output.status.success() {
        return Err("ioreg command failed".into());
    }

    let stdout = String::from_utf8(output.stdout)?;

    let re = Regex::new(r#""IOPlatformUUID"\s*=\s*"([^"]+)""#)?;
    let caps = re
        .captures(&stdout)
        .ok_or("IOPlatformUUID not found")?;

    Ok(caps[1].trim().to_string())
}

fn compute(uuid: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(uuid.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn output(hwid: &str) {
    let line = format!("Your HWID:  {}", hwid);
    let width = line.len() + 4;

    println!("╭{}╮", "─".repeat(width));
    println!("│  {}  │", line);
    println!("╰{}╯", "─".repeat(width));
    println!("              - HWID finder by ZackDaQuack 🦆");
    println!("              - Reconstructed by Falrux 💛");
}

fn main() -> Result<(), Box<dyn Error>> {
    let uuid = uuid()?;
    let hwid = compute(&uuid);
    output(&hwid);
    Ok(())
}