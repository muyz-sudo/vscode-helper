use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Url};
use std::path::{Path, PathBuf};
use tokio::{fs, io::AsyncWriteExt, runtime::Runtime};

/// Downloader
/// 参考 download_rs 库
/// url: 下载链接
/// out: 输出目录或输出文件路径
/// proxy: 使用代理，发现 reqwest 自动使用系统代理不好用 ,
///         目前只支持 http 代理 如：`http://127.0.0.1:7890` ,不支持 https,socks5代理（懒）
pub struct Downloader<'a> {
    url: &'a str,
    out: Option<&'a str>,
    proxy: Option<&'a str>,
}

impl<'a> Downloader<'a> {
    /// 创建 Download对象
    /// url: 需要下载的url
    /// out: 保存地址（具体文件夹或具体文件名）
    /// proxy: 使用代理，发现 reqwest 自动使用系统代理不好用 ,
    ///         目前只支持 http 代理 如：`http://127.0.0.1:7890` ,不支持 https,socks5代理（懒）
    pub fn new(url: &'a str, out: Option<&'a str>, proxy: Option<&'a str>) -> Downloader<'a> {
        return Downloader { url, out, proxy };
    }

    /// 异步方法下载
    /// 使用 `Runtime` 的 `block_on` 方法避免修改主函数
    #[allow(dead_code)]
    pub fn download(&self) -> Result<()> {
        let rt = Runtime::new()?;
        rt.block_on(self.download_async())?;
        Ok(())
    }

    /// 异步下载方法
    /// 参考: https://github.com/otavio/rsget.git
    pub async fn download_async(&self) -> Result<()> {
        let mut out_dir = "";

        let path_url = Url::parse(self.url)?;
        let mut filename = path_url
            .path_segments()
            .and_then(std::iter::Iterator::last)
            .unwrap_or("tmp.bin");

        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36"));

        let client_builder = match self.proxy {
            None => reqwest::Client::builder().no_proxy(),
            Some(proxy) => reqwest::Client::builder().proxy(reqwest::Proxy::all(proxy)?),
        };

        let client = client_builder.default_headers(headers).build()?;
        // 输出文件夹
        if let Some(output) = self.out {
            if Path::new(output).is_dir() {
                out_dir = output;
            } else {
                filename = output;
            }
        }
        let mut out_filename = PathBuf::from(out_dir);
        out_filename.push(filename);

        let resp = client.head(self.url).send().await?;
        let total_size = resp.content_length().unwrap();

        // 进度条
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"));

        let request = client.get(self.url);

        // 获取文件内容
        let mut source = request.send().await?;
        // 创建获取追加文件
        let mut dest = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&out_filename)
            .await?;

        while let Some(chunk) = source.chunk().await? {
            dest.write_all(&chunk).await?;
            pb.inc(chunk.len() as u64);
        }
        Ok(())
    }
}
