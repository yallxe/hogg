// This optimizer helps to detect forced HTTPS redirections and
// if needed, tell the scanner to non-SSL port. Also, it helps
// to detect if there are different HTTP and HTTPS web servers.
use reqwest::Client;

fn get_http_client() -> Client {
    Client::builder()
        .build()
        .unwrap()
}

pub async fn check_force_https(domain: String) -> bool {
    let client = get_http_client();
    let res = client.get(
        format!("http://{}", domain)
    ).send().await;

    if let Ok(res) = res {
        if res.status().is_success() {
            logs::info!("Forced HTTPS redirection detected: {}", domain);
            return true;
        }
    }

    false
}