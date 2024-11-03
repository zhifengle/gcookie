# gcookie

A tool for getting site cookie from your browser.

> [!note]  
> Chrome above 130 has changed the security of Chrome cookies on Windows, please try running gcookie with administrator privileges.

## About

When you write a script for scraping some data, you may need the cookie of the site.
Instead of copying cookie string from Browser, you can use the std output of this tool.

```javascript
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
> gcookie -p "/path/to/User Data/Default" "bing.com"
```

## Lib Usage

Add this to your Cargo.toml

```toml
[dependencies]
gcookie = "0.1.1"
```

> [!note]  
> Version 0.1.0 introduces a breaking change: the API has changed.
> It now uses the library "rookie" as the backend to read cookies.
> The original internal cookie reading function is deprecated.

get cookie by Chrome

```rust
let site = "http://google.com";
let cookie = gcookie::get_cookies("chrome", site);

let site = "bing.com";
let browser = "Edge";
let cookie = gcookie::get_cookies(browser, site);
assert!(cookie.is_ok());

let site = "https://google.com";
let mut path = PathBuf::new();
path.push(r"C:\my_chrome\user data\default");
let cookie =  match gcookie::get_chrome_cookies_by_path(site, &path) {
    Ok(cookie) => cookie,
    Err(err) => panic!("An error occurred when get cookie '{}': {}", site, err),
};
```

get cookie by Firefox with path

```rust
let site = "https://www.mozilla.org/";

let mut path = PathBuf::new();
path.push(r"C:\my_firefox\profile");

let cookie =  match gcookie::get_firefox_cookies_by_path(site, &path) {
    Ok(cookie) => cookie,
    Err(err) => panic!("An error occurred when get cookie '{}': {}", site, err),
};
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
