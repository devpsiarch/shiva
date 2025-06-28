# Shiva

**Shiva** is a Rust-based CLI tool to manage `systemd` services.  
The project is under active development but already feature‑complete for service creation and basic management.

## Available Commands

```bash
shiva help

```

Display usage information and a list of commands.

```bash
shiva list

```

List all services that Shiva knows about.

```bash
shiva create

```

Run an interactive “wizard” to generate and install a new `.service` file.

```bash
shiva enable <SERVICE_NAME>
shiva disable <SERVICE_NAME>
shiva kill   <SERVICE_NAME>
shiva start  <SERVICE_NAME>
shiva stop   <SERVICE_NAME>
shiva status <SERVICE_NAME>

```

Enable, disable, kill, start, stop, or query the status of an existing service.

```bash
shiva remove <SERVICE_NAME>

```

Uninstall and delete a managed service.

```bash
shiva log <SERVICE_NAME>

```

Show the journal logs for a managed service.

```bash
shiva backup <SERVICE_NAME>

```

Create a `.tar.gz` backup of the service’s unit file (and any associated files).

## Installation

```bash
git clone https://github.com/yourusername/shiva.git
cd shiva
cargo build --release

```

Copy `target/release/shiva` into your `$PATH` (e.g. `/usr/local/bin/`).

## Example Usage

```bash
# create a new service
shiva create

# list services
shiva list

# start a service called "my-app"
shiva start my-app

# view logs
shiva log my-app

# backup its unit file
shiva backup my-app

```

----------

> ⚠️ **Note**: Shiva is actively being enhanced—new features and refinements are on the roadmap. Contributions and feedback are very welcome!

```text
Current Status:  
- ✅ create  
- ✅ list  
- ✅ enable/disable  
- ✅ start/stop  
- ✅ kill  
- ✅ status  
- ✅ remove  
- ✅ log  
- ✅ backup  
