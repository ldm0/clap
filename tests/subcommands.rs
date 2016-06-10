extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, SubCommand, ErrorKind};

static VISIBLE_ALIAS_HELP: &'static str = "clap-test 2.6

USAGE:
    clap-test [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    vim|vi    Some help";

#[test]
fn subcommand() {
    let m = App::new("test")
        .subcommand(SubCommand::with_name("some")
            .arg(Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(true)
                .help("testing testing")))
        .arg(Arg::with_name("other").long("other"))
        .get_matches_from(vec!["myprog", "some", "--test", "testing"]);

    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "testing");
}

#[test]
fn subcommand_none_given() {
    let m = App::new("test")
        .subcommand(SubCommand::with_name("some")
            .arg(Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(true)
                .help("testing testing")))
        .arg(Arg::with_name("other").long("other"))
        .get_matches_from(vec![""]);

    assert!(m.subcommand_name().is_none());
}

#[test]
fn subcommand_multiple() {
    let m = App::new("test")
        .subcommands(vec![
            SubCommand::with_name("some")
                .arg(Arg::with_name("test")
                    .short("t")
                    .long("test")
                    .takes_value(true)
                    .help("testing testing")),
            SubCommand::with_name("add")
                .arg(Arg::with_name("roster").short("r"))
        ])
        .arg(Arg::with_name("other").long("other"))
        .get_matches_from(vec!["myprog", "some", "--test", "testing"]);

    assert!(m.subcommand_matches("some").is_some());
    assert!(m.subcommand_matches("add").is_none());
    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "testing");
}

#[test]
fn single_alias() {
    let m = App::new("myprog")
                .subcommand(SubCommand::with_name("test")
                    .alias("do-stuff"))
                .get_matches_from(vec!["myprog", "do-stuff"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn multiple_aliases() {
    let m = App::new("myprog")
                .subcommand(SubCommand::with_name("test")
                    .aliases(&["do-stuff", "test-stuff"]))
                .get_matches_from(vec!["myprog", "test-stuff"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn subcmd_did_you_mean_output() {
    test::check_err_output(test::complex_app(), "clap-test subcm",
"error: The subcommand 'subcm' wasn't recognized
\tDid you mean 'subcmd' ?

If you believe you received this message in error, try re-running with 'clap-test -- subcm'

USAGE:
    clap-test [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

For more information try --help", true);
}

#[test]
fn alias_help() {
    let m = App::new("myprog")
                .subcommand(SubCommand::with_name("test")
                    .alias("do-stuff"))
                .get_matches_from_safe(vec!["myprog", "help", "do-stuff"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn visible_aliases_help_output() {
    let app = App::new("clap-test")
        .version("2.6")
        .subcommand(SubCommand::with_name("vim")
            .about("Some help")
            .alias("invisible")
            .visible_alias("vi"));
    test::check_help(app, VISIBLE_ALIAS_HELP);
}