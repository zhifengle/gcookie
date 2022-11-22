# gcookie

A tool for getting site cookie from your browser.

## About

When you write a script for scraping some data, you may need the cookie of the site.
Instead of copying cookie string from Browser, you can use the std output of this tool.

```js
const { execSync } = require('child_process');
let site = 'bing.com';
const cookie = execSync(`gcookie ${site}`).toString();
```

```python
from subprocess import check_output
site = 'bing.com'
cookie = check_output(['gcookie', '-c', 'Edge', site]).decode("utf-8")
```

This package [bertrandom/chrome-cookies-secure](https://github.com/bertrandom/chrome-cookies-secure) depends on `win-dpapi`.
`win-dpapi` based on an older version of node-gyp is diffcult to install on Windows.

[moonD4rk/HackBrowserData](https://github.com/moonD4rk/HackBrowserData)

> This tools can export all of the data. In my case, I just want to a single site's cookie.

## Supported Browser

### Windows

Firefox, Chrome, Edge, Chromium

### Linux

Firefox

## Install

download the [release](https://github.com/zhifengle/gcookie/releases) for your system and run the binary

## Usage

```text
Usage: gcookie [OPTIONS] <site>

```

`gcookie -h` print help infomation.

## Examples

```shell
# App would use Chrome's cookie
> gcookie "google.com"
1P_JAR=2022-07-20-12; APISID=xxxyyyyy

# App would use Edge's cookie
> gcookie -c Edge "bing.com"

# App would use Firefox's cookie
> gcookie -f /path/to/profiles/xx.p "bing.com"

# App would use Chrome's cookie in this path
> gcookie -p /path/to/User Data/Default "bing.com"
```

## Lib Usage

Add this to your Cargo.toml

```toml
[dependencies]
gcookie = "*"
```

get cookie by Chrome

```Rust
let site = "http://cn.bing.com";
let cookie = gcookie::gcookie_chrome(site, None, None);

let site = "bing.com";
let browser = Some("Edge");
let cookie = gcookie::gcookie_chrome(site, browser, None);
assert!(cookie.is_ok());
```

## Development

```shell
git clone https://github.com/zhifengle/gcookie

# Build
cd gcookie
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```
