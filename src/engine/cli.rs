use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;
use std::process::Command;

/// A Visual Studio Code extension downloader.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Opts {
    /// Download extensions in the specified list file
    #[clap(short, long, parse(try_from_str = parse_file))]
    pub file: Option<PathBuf>,
    /// The folder where the download extension is placed
    #[clap(short, long, parse(try_from_str = parse_dir))]
    pub dir: Option<PathBuf>,
    /// Only save extension list to 'vscode-extensions.lis' at current dir
    #[clap(short, long, exclusive(true))]
    pub save_only: bool,
}

/// 解析文件
fn parse_file(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    if path.is_file() {
        Ok(path)
    } else {
        Err(anyhow!("can't parse file"))
    }
}

/// 解析路径
fn parse_dir(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    if path.is_dir() {
        Ok(path)
    } else {
        Err(anyhow!("can't parse directory"))
    }
}

/// 执行控制台命令并返回执行结果字符串
pub fn excute_command(cmd_str: &str) -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &cmd_str])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&cmd_str)
            .output()
            .expect("failed to execute process")
    };

    let res_ok = String::from_utf8_lossy(&output.stdout);
    let res_err = String::from_utf8_lossy(&output.stderr);
    if output.status.success() {
        Ok(res_ok.to_string())
    } else {
        Err(anyhow!(res_err.to_string()))
    }
}

// 解析控制台返回的（用换行符分割的）字符串到子字符串数组
pub fn parse_excmd_res(res: &str) -> Result<Vec<&str>> {
    Ok(res.split("\n").collect::<Vec<&str>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excute_command_works() {
        let cmd_str = String::from("code --list-extensions --show-versions");

        let res = excute_command(&cmd_str);

        match res {
            Ok(res) => println!("{:?}", res),
            Err(err) => println!("{:?}", err),
        }
        assert!(true)
    }
}
