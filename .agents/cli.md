# CLI guidelines

## Dependencies

- `clap` (features: at least "derive", "env")
- `tokio` (features: at least "macros", "rt", "rt-multi-thread")

## File layout and required items

### File `src/main.rs`

- Must define a `main` entrypoint
- Must define a basic test for the top-level command

Example:

```rust
use clap::Parser;
use errgonomic::exit_result;
use my_crate_name::Command;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = Command::parse();
    let result = args.run().await;
    exit_result(result)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Command::command().debug_assert();
}
```

### File `src/command.rs`

- Must define a [command-like struct](#command-like-struct) named `Command`
- Must define a [subcommand-like enum](#subcommand-like-enum) named `Subcommand`

Example:

```rust
use errgonomic::map_err;
use thiserror::Error;
use Subcommand::*;

mod print_command;

pub use print_command::*;

#[derive(clap::Parser, Debug)]
#[command(author, version, about)]
pub struct Command {
    #[command(subcommand)]
    command: Subcommand,
}

impl Command {
    pub async fn run(self) -> Result<(), CommandRunError> {
        use CommandRunError::*;
        let Self {
            command,
        } = self;
        map_err!(command.run().await, SubcommandRunFailed)
    }
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Subcommand {
    Print(PrintCommand),
}

impl Subcommand {
    pub async fn run(self) -> Result<(), SubcommandRunError> {
        use SubcommandRunError::*;
        match self {
            Print(command) => map_err!(command.run().await, PrintCommandRunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to run command")]
    SubcommandRunFailed { source: SubcommandRunError },
}

#[derive(Error, Debug)]
pub enum SubcommandRunError {
    #[error("failed to run print command")]
    PrintCommandRunFailed { source: PrintCommandRunError },
}
```

## Definitions

### Command-like struct

A struct that contains fields for CLI arguments.

- Must have a name that is a concatenation of all command names leading up to and including this command name, and ends with `Command` (see example above)
- Must derive `clap::Parser`
- Must be attached to a parent module: if it's a top-level command: `src/lib.rs`, else: `src/command.rs`
- May contain a `subcommand` field annotated with `#[command(subcommand)]`
- Must have a `pub async fn run`
  - Must return a `Result`

Command example:

- Shell command: `cargo run -- db download ycombinator-startups`
- Name: `DbDownloadYcombinatorStartupsCommand`
- File: `src/command/db_download_ycombinator_startups_command.rs` (attached to `src/command.rs`)

### Subcommand-like enum

An enum that contains variants for CLI subcommands.

- Must have a name that is a reverse concatenation of all command names leading up to and including this command name, and ends with `Subcommand` (see example above)
- Must derive `clap::Subcommand`
- Must be located in the same file as its parent command struct
- Each variant must be a tuple variant containing exactly one subcommand
- Must have a `pub async fn run`
  - Must match on `self` and call `run` on each subcommand
  - Must return a `Result`

Subcommand example:

- Shell command: `cargo run -- db download`
- Name: `DownloadDbSubcommand`
- File: `src/cli/db_command/download_db_command.rs` (same file as its parent `DownloadDbCommand`)
