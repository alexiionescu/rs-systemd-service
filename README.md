# rs-systemd-service

`rs-systemd-service` is a Rust command-line tool for creating a systemd service file from a template.

## Features

- Create systemd service files from a template
- Optionally install the service file to `/etc/systemd/system/`

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/yourusername/rs-systemd-service.git
cd rs-systemd-service
cargo build --release
```

## Usage

For help and usage information, run:

```bash
rs-systemd-service --help
```

Example usage:

```bash
rs-systemd-service --name my_service --target /path/to/executable --description "My Service Description" --user myuser --group mygroup --restart always --restart-after 30 --template /path/to/template.service -- ARGS
```

## Options

- `--name`: Name of the service.
- `--target`: Path to the executable that the service will run.
- `--description`: Description of the service.
- `--user`: User under which the service will run.
- `--group`: Group under which the service will run.
- `--restart`: Restart policy for the service (e.g., `always`, `on-failure`). Default is `always`.
- `--restart-after`: Time in seconds to wait before restarting the service. Default is `30`.
- `--template`: Path to the service file template. Default is `rust.service.templ` found in the git repository.
- `--args`: Additional arguments to pass to the executable.
