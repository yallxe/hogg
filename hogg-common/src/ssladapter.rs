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

    match res {
        Ok(res) => {
            if res.url().scheme() == "https" {
                logs::info!("Forced HTTPS redirection detected: {}", domain);
                return true;
            }
        },
        Err(_) => {}, // This can mean that unable to check if HTTPS is forced
    }

    false
}