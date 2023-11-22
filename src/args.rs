use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Args {
    /// 配置文件路径
    #[arg(short, long, default_value = "/etc/ddnspod.json")]
    pub cfg: String,
    /// 模拟运行
    #[arg(short, long, default_value = "false")]
    pub dry: bool,
    /// 循环运行还是一次性运行
    #[arg(short, long, default_value = "false")]
    pub lrun: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
