# hogg

An experimental project, sniffing visited websites via DNS sniffing-proxy and scanning them for common exploits (e.g.: Git Credentials Leak, Apache Server Status, ...). Inspired by Trufflehog and updns.

## Should I use it now?

Its kinda buggy, so user expirience might be uncomfortable for now.

## Why DNS?

HTTP Proxy is great, but it will cause problems with SSL for different softwares (I've tried mitmproxy with python earlier, and it was honestly a complete fail). So, DNS Sniffing comes as an extremely problem-less solution to silently sniff websites your PC is visiting.

## Vulnerability scanning techniques

To avoid tons of scanners writter in rust, hogg uses nuclei. This tool can rapidly scan multiple websites at once, and is easy to configurate as you can add your own vulnerabilities patterns with it own templates scheme in YAML.

## Differencies from Trufflehog

Hogg scans (almost) every single website your PC is visiting, while Trufflehog is an extenstion for browser which scans traffic from your browser only. However, Trufflehog can see the responses websites are sending, what means that it can lookup for leaked API tokens and etc., while Hogg just can't, because its all DNS.

## Can it use more sniffing/hijacking techniques than just DNS?

Not yet, but I want to add something more. At least I want to think about HTTP proxy in case somebody will need it.

## Limitations

- You are currently unable to use DNS-over-HTTPS or other similar solutions
- You currently have change your DNS servers to localhost (127.0.0.1)
- You may NOT get a full interception of DNS packets yet.

## Todo stuff

- [x] Working DNS Proxy + Nuclei scanner
- [ ] Add notifications
- [ ] Add more scanners
- [ ] Add DNS Network Sniffer (not a proxy!!!), like it would be a wireshark network sniffer
- [ ] Add DNS-over-HTTPS support for DNS Proxy.
