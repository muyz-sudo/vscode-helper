use anyhow::Result;
use std::path::Path;
use clap::Parser;
use tokio::{fs::OpenOptions, io::{AsyncBufReadExt, BufReader}};

mod engine;
use engine::extension::ExtensionInfo;
use engine::downloader::Downloader;
use engine::cli;

// 从文件读取扩展
async fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<ExtensionInfo>> {
    let file = OpenOptions::new().read(true).open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut res = Vec::new();
    while let Some(line) = lines.next_line().await? {
        if ! &line.is_empty() { // 不处理空白行
            match ExtensionInfo::try_from(line.as_str()) {
                Ok(ext) => res.push(ext),
                Err(e) => println!("{}", e.to_string()),
            }
        }
    }
    Ok(res)
}

// 下载动作
async fn download(ext: &ExtensionInfo) -> Result<()> {
    let url = &ext.get_url().unwrap();
    let filename = &ext.get_filename().unwrap();
    // println!("{:?}", &url);

    let downloader = Downloader::new(&url, Some(&filename), None);
    downloader.download_async().await
}

// 从Vscode获取已安装的扩展列表
fn get_extensions_from_vscode() -> Result<Vec<ExtensionInfo>> {
    let cmd_str = String::from("code --list-extensions --show-versions");
    let ext_list_str = cli::excute_command(&cmd_str)?;
    let ext_list = cli::parse_excmd_res(&ext_list_str)?;
    let mut extensions = Vec::new();
    for ext in ext_list {
        if ! ext.is_empty() { // 不处理空白行
            match ExtensionInfo::try_from(ext) {
                Ok(ext) => extensions.push(ext),
                Err(e) => println!("{}", e.to_string()),
            }
        }
    }
    Ok(extensions)
}

// 从Vscode保存已安装插件到文件
fn save_extensions_from_vscode() -> Result<String> {
    let cmd_str = String::from("code --list-extensions --show-versions >> vscode-extensions.lis");
    cli::excute_command(&cmd_str)
}

#[tokio::main]
async fn main(){
    let opts: cli::Opts = cli::Opts::parse();
    if opts.save_only {
        save_extensions_from_vscode().unwrap();
    } else {
        let extensions = match &opts.file {
            Some(file) => read_lines(file).await.unwrap(),
            None => get_extensions_from_vscode().unwrap(),
        };

        for extension in extensions {
            match download(&extension).await {
                Ok(_) => println!("{} 下载成功", &extension.name),
                Err(e) => println!("下载失败: {}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    // 异步测试宏
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn read_lines_works(){
        let path = PathBuf::from("/Users/wuaq/Desktop/vs_code_extensions_list.txt");
        let extensions = aw!(read_lines(path));
        for extension in &extensions {
            println!("{:?}", extension)
        }
        assert!(extensions.is_ok())
    }

    #[test]
    fn download_works(){
        let ext = ExtensionInfo {
            author: String::from("bungcip"),
            name: String::from("better-toml"),
            version: String::from("0.3.2"),
        };
        assert!(aw!(download(&ext)).is_ok());
    }
}