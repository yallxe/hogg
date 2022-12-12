use async_trait::async_trait;
use crate::scanner::ServicesScanner;

#[async_trait]
pub trait Hijacker {
    fn name(&self) -> String; // Returns name of hijacker (for example, "DNS Proxy Hijacker")
    async fn run(&mut self, scanner_ctx: &ServicesScanner); // As hijacking is "forever" process, this function returns nothing and is called as a task.
}

pub mod dnsproxy;