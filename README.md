# gcookie

A tool for getting site cookie from your browser.

> [!note]  
> Chrome above 110 would lock Cookies file when browser is running.
> please run gcookie with administrator privileges or close browser.

Edge has a running process in background, please terminate it or elevate gcookie's privileges

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
site = 'https://cn.bing.com'
# output of cookies in `cn.bing.com`. not `bing.com`
cookie = check_output(['gcookie', '-c', 'Edge', site]).decode("utf-8")
# example in Scrapy
def get_cookies_dict(cookies):
    if not cookies:
        return {}
    return {c.split('=')[0]: c.split('=')[1] for c in cookies.split('; ')}

class DemoSpider(scrapy.Spider):
    name = 'demo'

    def start_requests(self):
        cookies_dict = get_cookies_dict("your_cookies")
        yield scrapy.Request(
            url,
            headers={'Content-Type': 'application/json'},
            cookies=cookies_dict,
            callback=self.parse,
        )
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
