# gcookie

An tool for getting site cookie from your browser.

## About

When you write a script for scraping some data, you may need the cookie of the site.
Instead of copying cookie string from Browser, you can use the std output of this tools.

```js
const { execSync } = require('child_process');
let site = 'bing.com';
const cookie = execSync(`gcookie ${site}`).toString();
```

This package [bertrandom/chrome-cookies-secure](https://github.com/bertrandom/chrome-cookies-secure) depends on `win-dpapi`.
`win-dpapi` based on an older version of node-gyp is diffcult to install on Windows.

```python
from subprocess import check_output
site = 'bing.com'
cookie = check_output(['gcookie', '-c', 'Edge', site]).decode("utf-8")
```

[moonD4rk/HackBrowserData](https://github.com/moonD4rk/HackBrowserData)

> This tools can export all of the data. In my case, I just want to a single site's cookie.

## Supported Browser

### Windows

Firefox, Chrome, Edge, Chromium

### Linux

Firefox

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
