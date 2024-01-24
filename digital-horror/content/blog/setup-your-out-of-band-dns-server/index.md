+++
title = "Setup your Out-of-Band DNS Server"
description = "Tutorial on how to use OOB-Server to setup your own dedicated Out-of-Band DNS server."
date = 2019-08-24
[taxonomies]
tags = ["security"]

[extra]
archive = "This is an old post that has been migrated over one or more times. It may contain issues with certain images and formatting."
+++

I opted to compile the following post in response to a few messages I have been receiving on [twitter](https://web.archive.org/web/20190828215338/https://twitter.com/juxhindb) on how to setup a Bind9 DNS Resolver. Mostly because I published a small project on [GitHub](https://web.archive.org/web/20190828215338/https://github.com/JuxhinDB/OOB-Server) for myself that got "some" traction.

May sound odd, however setting up your own DNS server allows you to detect juicy Out-of-Band (OOB) vulnerabilities or their OOB variants such as SSRF, XXE, XSS and so on. Also DNS is fun.

> Disclaimer — This is **not** the easiest way to setup an OOB DNS resolver and can be quite rigid on its own. My scenario required me to process a ceiling of 40k packets per second without downtime and return an actual response back to avoid detection. You may want to look into using [Burp Collaborator](https://web.archive.org/web/20190828215338/https://portswigger.net/burp/documentation/collaborator) or [DNSChef](https://web.archive.org/web/20190828215338/https://github.com/amckenna/DNSChef) or combine them with Bind9 for performance and flexibility.

## Introduction

In favour of keeping the post short and to the point, I will assume that the reader is aware of Out-of-Band vulnerabilities and how they are detected. If not, the following are a list of great articles to research on the subject.

* https://portswigger.net/blog/oast-out-of-band-application-security-testing
* https://www.acunetix.com/blog/articles/band-xml-external-entity-oob-xxe/

## Pre-requisites

The rest of the post will be based on a fresh DigitalOcean $5/month droplet running Ubuntu 18.04 LTS and uses the project listed above as well as my own dummy domain.

Before getting into the technicalities, I would advise doing the following.

1. Setup your machine of choice. For a quick testing ground, DigitalOcean is perfect. You simply need to have an IP available.
2. Have a domain available to bind to your nameserver. This is key to setting things up. It is DNS after all.
3. Point the domain from your domain registrar to your machine's IP. This varies from registrar to registrar.

In the case of namecheap, I need to first setup my own nameservers from the "Advanced DNS" section as shown below.

> _Unfortunately the original screenshots have been lost with time. If there is demand I can attempt to regenerate them._

Then I had to configure Namecheap to point the domain to that nameserver from the main screen.

> _Unfortunately the original screenshots have been lost with time. If there is demand I can attempt to regenerate them._

Once this is configured, we are ready to get started.

## Setup

> **Note** — everything here is being run as `root`. You may need to adjust your permissions accordingly if need be.

Start off by SSH'ing into your machine and cloning the OOB-Server project.

```bash
$ git clone https://github.com/JuxhinDB/OOB-Server.git
$ cd OOB-Server
```

You can then run the setup script and you're ready.

```bash
./setup $YOUR_DOMAIN $YOUR_IP
```

Initially you should see it update packages and install `bind`. In retrospect, the update ought to be removed from the script. Towards the end of the setup you should see some logs along the lines of.

```none
[+] setup: [08-24-2019 18:40:58] INFO Setting up paths and permissions for Bind9 logs
[+] setup: [08-24-2019 18:40:58] INFO Updating all Bind9 configurations
[?] setup: [08-24-2019 18:40:58] DEBUG Adding db.local to /etc/bind/db.local

[?] setup: [08-24-2019 18:40:58] DEBUG Adding named.conf.options to /etc/bind/named.conf.options
[?] setup: [08-24-2019 18:40:58] DEBUG Creating named.conf.log and including it in named.conf
[?] setup: [08-24-2019 18:40:58] DEBUG Setting up lograte for bind
[+] setup: [08-24-2019 18:40:58] INFO Reloading logrotate to take effect on new Bind9 logs
```

Once that is complete, you are good to go. You can quickly run a test with the following (be sure to change your domain).

```bash
dig A +short google.com @ns1.YOUR_DOMAIN_HERE
```

Then just tail the logs with.

```bash
tail -f /var/log/named/named.log
```

And you should get a valid response back and the query logged in the logfile.

## Wrap up

It's good to keep in mind that logs are automatically rotated, so you can effectively keep a good amount of historical queries in there without exhausting the machine.

One suggestion when looking to exploit vulnerabilities using this, is to keep a specific structure to your subdomains. For example in the following format:

`vuln-type.domain.payload-number.YOUR_DOMAIN`

For instance, when testing for XXE you can inject a payload such as `xxe.targethost.23.evilhost.com`.

```bash
$ dig +short A xxe.targethost.23.gbejna.bid
104.248.84.83

$ tail -f /var/log/named/named.log
24-Aug-2019 21:19:40.546 queries: info: client @0x7f73645138b0 172.217.41.8#63929 (xxe.targethost.23.gbejna.bid): query: xxe.targethost.23.gbejna.bid IN A -E(0)DC (104.248.84.83)
```

As some of you may notice, this may get unwieldy very quickly. I recommend always keeping track of your payload IDs. You may also want to filter your tail to only spit out logs with the specific pattern.

If you want to absolutely automate this, you can put DNSChef in front of Bind9 to process interesting payloads, such as ones that fit the above format, and forward the rest to Bind to process normally. This is nice when paired with something like Slack to notify you when a payload triggers.

Lastly, thank you to [@ianmuscat](https://web.archive.org/web/20190828215338/https://twitter.com/ianmuscat) for the really clean and useful bash utility functions.
