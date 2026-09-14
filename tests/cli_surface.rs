//! CLI surface regression tests.
//!
//! `connect` and `watch` were advertised in `--help` in earlier revisions while only printing a
//! "still under development" notice and exiting 0. They were removed rather than left as stubs, so
//! that every command the help lists is a command that works. These tests keep that true: if a
//! command is ever re-advertised, it must be as a real implementation, not a placeholder.

use clap::Parser;
use deltasafe::cli::{Cli, Commands};

fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    let mut argv = vec!["deltasafe"];
    argv.extend_from_slice(args);
    Cli::try_parse_from(argv)
}

/// `Cli` deliberately does not derive `Debug`, so both result paths are unwrapped by hand.
fn parse_error(args: &[&str]) -> clap::Error {
    match parse(args) {
        Ok(_) => panic!("expected `{}` to be rejected", args.join(" ")),
        Err(error) => error,
    }
}

fn parse_ok(args: &[&str]) -> Cli {
    match parse(args) {
        Ok(cli) => cli,
        Err(error) => panic!("expected `{}` to parse: {error}", args.join(" ")),
    }
}

#[test]
fn help_lists_only_implemented_commands() {
    let help = parse_error(&["--help"]).to_string();

    for command in ["sync", "discover", "server"] {
        assert!(
            help.contains(command),
            "help should list `{command}`:\n{help}"
        );
    }
    for removed in ["connect", "watch"] {
        assert!(
            !help.contains(removed),
            "help must not advertise the removed command `{removed}`:\n{help}"
        );
    }
}

#[test]
fn sync_is_accepted() {
    let cli = parse_ok(&["sync", "--source", "./fixtures"]);
    assert!(matches!(cli.command, Commands::Sync { .. }));
}

#[test]
fn server_and_discover_are_accepted() {
    let server = parse_ok(&["server", "--password", "secret"]);
    assert!(matches!(server.command, Commands::Server { .. }));

    let discover = parse_ok(&["discover"]);
    assert!(matches!(discover.command, Commands::Discover { .. }));
}

#[test]
fn removed_commands_are_rejected() {
    for removed in [
        vec!["connect", "--ip", "127.0.0.1"],
        vec!["watch", "--folder", "/tmp"],
    ] {
        let error = parse_error(&removed);
        assert_eq!(
            error.kind(),
            clap::error::ErrorKind::InvalidSubcommand,
            "`{}` should fail as an unknown subcommand",
            removed[0]
        );
    }
}
