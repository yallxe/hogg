# Hogg 🐽

An experimental passive website scanner. Hogg acts as a proxy between you and your DNS server and scans every website you visit for common vulnerabilities.

## Building

0. Make sure you have Rust installed. If not, follow the instructions [here](https://www.rust-lang.org/tools/install).
1. Install dependencies for [tonic](https://github.com/hyperium/tonic) from [here](https://github.com/hyperium/tonic#dependencies).
2. Install [Nuclei](https://github.com/projectdiscovery/nuclei) and make sure it's in your `$PATH`.
3. Clone the repo and `cd` into it.
4. Run `cargo build --release` to build the binary.

## Using it

To make hogg work, you need to run the daemon, which will serve the DNS proxy and scan the websites you visit. You will get a notification when a vulnerability is found. To view the vulnerabilities, you can use the `hogg` CLI. Use `hogg-cli -h` to see the available commands.
To run the daemon, use `hogg-daemon` binary.

## Configuration

Checkout your configuration path, which is printed when you start the daemon, or use `echo $HOGG_CONFIG_DIR`

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
- [x] Notifications (OS notifications for now)
- [ ] Automatic request redirection to DNS Proxy
- [ ] GUI (a tray icon)
- [ ] DNS over HTTPS

## Credits

- Inspired by [Trufflehog-Chrome-Extension](https://github.com/trufflesecurity/Trufflehog-Chrome-Extension) ❤️
