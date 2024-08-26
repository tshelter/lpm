# L Process Manager (lpm)

L Process Manager, or `lpm`, is a Rust-based project that leverages the power of systemd (in either root or user mode) to create and manage services.

### Install
You can install prebuilt binaries from the [releases](https://github.com/tshelter/lpm/releases) page.

Or using the following command:
```bash
wget -qO- "https://github.com/tshelter/lpm/releases/latest/download/lpm-$(uname -m).tar.gz" | tar xzvf -
mv lpm /usr/local/bin
```

Also, you can build and install the project by running the following commands:
```bash
cargo install --git https://github.com/tshelter/lpm.git
```

### Setup
To run `lpm` in user-mode, you need to enable dbus socket to be available anytime by running the following command:
```bash
sudo loginctl enable-linger $USER
```

### Usage
```commandline
$ lpm --help
Usage: lpm <COMMAND>

Commands:
  run      Run a command as a service
  start    Start a service
  status   Get the status of a service
  stop     Stop a service
  restart  Restart a service
  enable   Enable a service
  disable  Disable a service
  reload   Reload a service
  logs     Display logs for a service
  remove   Remove a service
  list     List services
  cat      Print the service file for a service
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Examples
Creating a simple http server using `python3`:
```bash
lpm run -n http-server 'python3 -m http.server 8000'
```
Service will execute command in the current directory. Environment won't be inherited.

Instead, if you want to run a service with the current environment, you can use the following command:
```bash
lpm run -E -n http-server 'python3 -m http.server 8000'
```

Also, you can pass environment variables to the service:
```bash
lpm run -E -n http-server -e 'PORT=8000' 'python3 -m http.server $PORT'
```
> note, using 'single quotes' is important to prevent shell expansion on `$PORT`.

You can use `-e` multiple times to pass multiple environment variables.

[//]: # (Currently, this feature is unsupported)
[//]: # (CLI interface is quite similar to `docker`, so you can use `lpm` with common patterns:)
[//]: # (```bash)
[//]: # (# Stop all services)
[//]: # (lpm ls -q | xargs lpm stop)
[//]: # ()
[//]: # (# Remove all services)
[//]: # (lpm ls -q | xargs lpm rm)
[//]: # ()
[//]: # (# Restart all services)
[//]: # (lpm ls -q | xargs lpm restart)
[//]: # (```)

### Advanced
You can pass additional arguments to the unit file using --unit, --service, and --install options.
```commandline
$ lpm run --help
Run a command as a service

Usage: lpm run [OPTIONS] --name <NAME> <COMMAND>

Arguments:
  <COMMAND>  The command to run as a service. Wrap 'command in quotes' to pass arguments.

Options:
  -n, --name <NAME>                The name of the service
  -E, --inherit-env                Inherit the current environment to the service. Usually not required.
  -e, --env <ENV>                  List of key=value pairs to pass to the service as environment variables
  -d, --description <DESCRIPTION>  A description of the service [default: ]
  -u, --unit <UNIT>                List of key=value pairs for the [Unit] section of the service file
  -s, --service <SERVICE>          List of key=value pairs for the [Service] section of the service file
  -i, --install <INSTALL>          List of key=value pairs for the [Install] section of the service file
  -h, --help                       Print help
```

Changing working directory for the service:
```basg
lpm run -n http-server -u 'WorkingDirectory=/path/to/working/directory' 'python3 -m http.server 8000'
```

`lpm` uses `/usr/bin/env` to run the command, but you can override it by passing ExecStart option:
```bash
lpm run -n http-server -s 'ExecStart=/usr/bin/python3 -m http.server 8000' 'this will be ignored, but required argument'
```
