use anyhow::Result;
use hijackers::dnsproxy::DnsProxyHijacker;
use crate::hijackers::Hijacker;
use crate::scanner::scanners::gitcreds::GitCredsScanner;

mod scanner;
mod hijackers;

#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => {
        {
            logs::error!($($arg)*);
            std::process::exit(1)
        }
    };
}


#[tokio::main]
async fn main() -> Result<()> {
    logs::Logs::new().init();
    let scanner = scanner::ServicesScanner::new(vec![
        Box::new(GitCredsScanner {})
    ]);
    DnsProxyHijacker::new("127.0.0.1:53".parse()?, vec![
        "1.1.1.1:53".parse()?
    ]).run(&scanner).await;

    Ok(())
}
