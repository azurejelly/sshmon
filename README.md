# sshmon
Sends push notifications every time someone logs in to your server via SSH.

> [!NOTE]  
> Only Pushover support has been implemented for now.

## Installation
You can easily install sshmon by using Docker. An image is already available at [Docker Hub](https://hub.docker.com/r/azurejelly/sshmon):
```sh
$ docker run -it \
    -d \
    --restart=always \
    -v /var/log:/var/log:ro \
    --env-file .env \
    azurejelly/sshmon:latest
```
Or, check out the [Docker Compose](./docker-compose.yml) on this same repository.

### Configuration
sshmon is configured through environment variables. You can set them by using a `.env` file:

| Environment variable | Description                                                                                                                                                  | Required                                                      | Default value                                  |
|----------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------|------------------------------------------------|
| `SSH_LOGS_PATH`      | The place where successful SSH logins can be found. Usually `/var/log/auth.log` on Ubuntu servers, `/var/log/secure` on Fedora/CentOS/RHEL.                  | No, unless `/var/log/auth.log` is not a thing in your system. | `/var/log/auth.log`                            |
| `AUTH_SUCCESS_REGEX` | The expression used to match successful SSH logins on the previously mentioned file.                                                                         | No, unless the message has been modified.                     | `Accepted (\w+) for (\w+) from ([\d\.]+) port` |
| `RUST_LOG`           | Sets the program log level. You can also set it to `debug` in order to enable debug messages.                                                                | No                                                            | `info`                                         |
| `NOTIFIER`           | Which service to use in order to send push notifications. Can be `pushover` or `stdout` (only intended for testing purposes).                                | Yes                                                           | `pushover`                                     |
| `PUSHOVER_API_KEY`   | Your Pushover application API key.                                                                                                                           | Yes (if `NOTIFIER` is set to `pushover`)                      | N/A                                            |
| `PUSHOVER_USER_KEY`  | Your Pushover user key.                                                                                                                                      | Yes (if `NOTIFIER` is set to `pushover`)                      | N/A                                            |
| `HOSTNAME_OVERRIDE`  | Can be used to override the hostname on each push notification in case it is inaccurate or the automatic hostname detection is causing the program to panic. | No                                                            | N/A                                            |

## Building
Assuming you already have all the required Rust toolchains installed, clone the repository using Git:
```sh
$ git clone https://github.com/azurejelly/sshmon
$ cd sshmon/
```
Then, to build the program:
```sh
$ cargo build
```
Or to run it:
```sh
$ cargo run
```
