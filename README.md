# Hogg 🐽
An experimental passive website scanner. Hogg acts as a proxy between you and your DNS server and scans every website you visit for common vulnerabilities.

## Installation
Get the appropriate binary from the [GitHub Releases](https://github.com/yallxe/hogg/releases) page. If you wish to build Hogg yourself, run these commands (assuming you have Rust and Git installed):
```shell
git clone https://github.com/yallxe/hogg
cd hogg
cargo run
```

## Usage
Just `cargo run`, and then set your DNS server to `localhost:53`

## Configuration
Checkout your configuration path, which is printed when you start the program, or use `echo $HOGG_CONFIG_DIR`

## How does it work?
1. Your browser or a desktop app resolves a domain name via DNS.
2. Hogg requests the data from your upstream DNS provider (Cloudflare by default) and sends it back to the app.
3. Hogg scans the website using [Nuclei](https://github.com/projectdiscovery/nuclei).

## How is it different?
Hogg will help you scan almost every website you visit (not limited to your browser) without causing any disruption to the app's functionality.

## Anything besides DNS?
Not yet. Stay tuned for future updates that may include other solutions (like an HTTP proxy).

## Limitations
- Hogg doesn't yet support DNS over HTTPS, DNS over TLS etc.
- Some apps may bypass your system's default DNS resolver. In this case, Hogg will not intercept the app's requests.

## Progress
- [x] Working DNS proxy and Nuclei scanner
- [x] Notifications (Telegram and OS notifications for now)
- [ ] Automatic request redirection to DNS Proxy
- [ ] GUI (a tray icon)
- [ ] DNS over HTTPS

## Credits
- Inspired by [Trufflehog-Chrome-Extension](https://github.com/trufflesecurity/Trufflehog-Chrome-Extension) ❤️
