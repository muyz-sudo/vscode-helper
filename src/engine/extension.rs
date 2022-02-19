use anyhow::{Result, anyhow};
use regex::Regex;
use std::convert::TryFrom;

/// 扩展信息
#[derive(Debug)]
pub struct ExtensionInfo {
    /// 作者
    pub author: String,
    /// 扩展名
    pub name: String,
    /// 版本
    pub version: String,
}

/// 让 ExtensionInfo 可以生产一个字符串
impl From<&ExtensionInfo> for String {
    fn from(ext: &ExtensionInfo) -> Self {
        format!("author: {}, name: {}, version: {}", ext.author, ext.name, ext.version)
    }
}

/// 让 ExtensionInfo 可以通过字符串创建
impl TryFrom<&str> for ExtensionInfo {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^([\w-]+)\.([\w-]+)@(\d+\.\d+\.\d+)$").unwrap();
        match re.captures(value) {
            Some(caps) => Ok(ExtensionInfo {
                author: caps[1].to_string(),
                name: caps[2].to_string(),
                version: caps[3].to_string(),
            }),
            None => Err(anyhow!("{} could not be parsed", value)),
        }
    }
}

impl ExtensionInfo {
    /// 获取下载链接
    pub fn get_url(&self) -> Result<String> {
        Ok(format!("https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{}/vsextensions/{}/{}/vspackage", &self.author, &self.name, &self.version))
    }

    /// 获取文件名
    pub fn get_filename(&self) -> Result<String> {
        Ok(format!("{}.{}-{}.vsix", &self.author, &self.name, &self.version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_url_works() {
        let plugin = ExtensionInfo {
            author: String::from("bungcip"),
            name: String::from("better-toml"),
            version: String::from("0.3.2"),
        };
        assert!(&plugin.get_url().unwrap() == "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/bungcip/vsextensions/better-toml/0.3.2/vspackage");
    }

    #[test]
    fn extension_try_from_works(){
        let s = String::from("2gua.rainbow-brackets@0.0.6");
        let ext = ExtensionInfo::try_from(s.as_str()).unwrap();
        println!("{:?}", ext);
        assert!(ext.author == "2gua");
        assert!(ext.name == "rainbow-brackets");
        assert!(ext.version == "0.0.6");
    }
}