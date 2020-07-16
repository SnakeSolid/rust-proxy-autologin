# Proxy Autologin

Utility for automatic login to corporative proxy server. Active authorization by default will be checked using [Firefox
detect portal](http://detectportal.firefox.com/success.txt) and can be configured.

When request to detect portal (HTTP GET) returns redirection status, then will be executed POST request to login using
location from redirect header. Detect portal will be checked using interval defined as `cron` (`quartz`) expression.

## Usage

Start `proxy-autologin` with default settings except of login and password:

```bash
AUTOLOGIN_PASS=password ./proxy-autologin --login username
```

Required arguments:

* `-l` (`--login`): Proxy login;
* `-p` (`--pass`): Proxy password;

Optional arguments:

* `-c` (`--cron`): Cron/Quartz expression, default = `0 0 6 * * mon-fri *`;
* `-d` (`--detect-url`): Detect portal URL address, default = `http://detectportal.firefox.com/success.txt`;
* `-i` (`--error-interval`): Interval between retries when error occurred, default = 300;
* `-r` (`--error-retry`): Number of retries when error occurred, default = 3;
* `-n` (`--now`): Authorize immediately after start without stopping scheduled checks;
* `-t` (`--timeout`): HTTP connection timeout in seconds, default = 30;
* `-h` (`--help`): Show help and exit.

Environment variables:

* `AUTOLOGIN_CRON`: variable value will be used if argument `--cron` not given;
* `AUTOLOGIN_DETECT_URL`: variable value will be used if argument `--detect-url` not given;
* `AUTOLOGIN_ERROR_INTERVAL`: variable value will be used if argument `--error-interval` not given;
* `AUTOLOGIN_ERROR_RETRY`: variable value will be used if argument `--error-retry` not given;
* `AUTOLOGIN_LOGIN`: variable value will be used if argument `--login` not given;
* `AUTOLOGIN_PASS`: variable value will be used if argument `--pass` not given;
* `AUTOLOGIN_TIMEOUT`: variable value will be used if argument `--timeout` not given;

Command line options have higher priority than environment variables.

Logs will be shown when environment variable `RUST_LOG=info` given.
