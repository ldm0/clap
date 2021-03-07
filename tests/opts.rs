mod utils;

use clap::{App, Arg, ArgMatches, ArgSettings, ErrorKind};

#[cfg(feature = "suggestions")]
static DYM: &str =
    "error: Found argument '--optio' which wasn't expected, or isn't valid in this context

\tDid you mean '--option'?

\tIf you tried to supply `--optio` as a value rather than a flag, use `-- --optio`

USAGE:
    clap-test --option <opt>...

For more information try --help";

#[cfg(feature = "suggestions")]
static DYM_ISSUE_1073: &str =
    "error: Found argument '--files-without-matches' which wasn't expected, or isn't valid in this context

\tDid you mean '--files-without-match'?

\tIf you tried to supply `--files-without-matches` as a value rather than a flag, use `-- --files-without-matches`

USAGE:
    ripgrep-616 --files-without-match

For more information try --help";

#[test]
fn require_equals_fail() {
    let res = App::new("prog")
        .arg(
            Arg::new("cfg")
                .setting(ArgSettings::RequireEquals)
                .setting(ArgSettings::TakesValue)
                .long("config"),
        )
        .try_get_matches_from(vec!["prog", "--config", "file.conf"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn require_equals_min_values_zero() {
    let res = App::new("prog")
        .arg(
            Arg::new("cfg")
                .setting(ArgSettings::RequireEquals)
                .min_values(0)
                .long("config"),
        )
        .arg(Arg::new("cmd"))
        .try_get_matches_from(vec!["prog", "--config", "cmd"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("cfg"));
    assert_eq!(m.value_of("cmd"), Some("cmd"));
}

#[test]
fn require_equals_no_empty_values_fail() {
    let res = App::new("prog")
        .arg(
            Arg::new("cfg")
                .setting(ArgSettings::RequireEquals)
                .long("config"),
        )
        .arg(Arg::new("some"))
        .try_get_matches_from(vec!["prog", "--config=", "file.conf"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn require_equals_empty_vals_pass() {
    let res = App::new("prog")
        .arg(
            Arg::new("cfg")
                .setting(ArgSettings::RequireEquals)
                .setting(ArgSettings::AllowEmptyValues)
                .long("config"),
        )
        .try_get_matches_from(vec!["prog", "--config="]);
    assert!(res.is_ok());
}

#[test]
fn require_equals_pass() {
    let res = App::new("prog")
        .arg(
            Arg::new("cfg")
                .setting(ArgSettings::RequireEquals)
                .long("config"),
        )
        .try_get_matches_from(vec!["prog", "--config=file.conf"]);
    assert!(res.is_ok());
}

#[test]
fn stdin_char() {
    let r = App::new("opts")
        .arg(Arg::from("-f [flag] 'some flag'"))
        .try_get_matches_from(vec!["", "-f", "-"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("f"));
    assert_eq!(m.value_of("f").unwrap(), "-");
}

#[test]
fn opts_using_short() {
    let r = App::new("opts")
        .args(&[
            Arg::from("-f [flag] 'some flag'"),
            Arg::from("-c [color] 'some other flag'"),
        ])
        .try_get_matches_from(vec!["", "-f", "some", "-c", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("f"));
    assert_eq!(m.value_of("f").unwrap(), "some");
    assert!(m.is_present("c"));
    assert_eq!(m.value_of("c").unwrap(), "other");
}

#[test]
fn lots_o_vals() {
    let r = App::new("opts")
        .arg(Arg::from("-o [opt]... 'some opt'"))
        .try_get_matches_from(vec![
            "", "-o", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some",
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().count(), 297); // i.e. more than u8
}

#[test]
fn opts_using_long_space() {
    let r = App::new("opts")
        .args(&[
            Arg::from("--flag [flag] 'some flag'"),
            Arg::from("--color [color] 'some other flag'"),
        ])
        .try_get_matches_from(vec!["", "--flag", "some", "--color", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_long_equals() {
    let r = App::new("opts")
        .args(&[
            Arg::from("--flag [flag] 'some flag'"),
            Arg::from("--color [color] 'some other flag'"),
        ])
        .try_get_matches_from(vec!["", "--flag=some", "--color=other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed() {
    let r = App::new("opts")
        .args(&[
            Arg::from("-f, --flag [flag] 'some flag'"),
            Arg::from("-c, --color [color] 'some other flag'"),
        ])
        .try_get_matches_from(vec!["", "-f", "some", "--color", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed2() {
    let r = App::new("opts")
        .args(&[
            Arg::from("-f, --flag [flag] 'some flag'"),
            Arg::from("-c, --color [color] 'some other flag'"),
        ])
        .try_get_matches_from(vec!["", "--flag=some", "-c", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn default_values_user_value() {
    let r = App::new("df")
        .arg(Arg::from("-o [opt] 'some opt'").default_value("default"))
        .try_get_matches_from(vec!["", "-o", "value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), "value");
}

#[test]
fn multiple_vals_pos_arg_equals() {
    let r = App::new("mvae")
        .arg(Arg::from("-o [opt]... 'some opt'"))
        .arg(Arg::from("[file] 'some file'"))
        .try_get_matches_from(vec!["", "-o=1", "some"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), "1");
    assert!(m.is_present("file"));
    assert_eq!(m.value_of("file").unwrap(), "some");
}

#[test]
fn multiple_vals_pos_arg_delim() {
    let r = App::new("mvae")
        .arg(Arg::from("-o [opt]... 'some opt'").setting(ArgSettings::UseValueDelimiter))
        .arg(Arg::from("[file] 'some file'"))
        .try_get_matches_from(vec!["", "-o", "1,2", "some"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["1", "2"]);
    assert!(m.is_present("file"));
    assert_eq!(m.value_of("file").unwrap(), "some");
}

#[test]
fn require_delims_no_delim() {
    let r = App::new("mvae")
        .arg(Arg::from("-o [opt]... 'some opt'").setting(ArgSettings::RequireDelimiter))
        .arg(Arg::from("[file] 'some file'"))
        .try_get_matches_from(vec!["mvae", "-o", "1", "2", "some"]);
    assert!(r.is_err());
    let err = r.unwrap_err();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
}

#[test]
fn require_delims() {
    let r = App::new("mvae")
        .arg(Arg::from("-o [opt]... 'some opt'").setting(ArgSettings::RequireDelimiter))
        .arg(Arg::from("[file] 'some file'"))
        .try_get_matches_from(vec!["", "-o", "1,2", "some"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["1", "2"]);
    assert!(m.is_present("file"));
    assert_eq!(m.value_of("file").unwrap(), "some");
}

#[test]
#[cfg(feature = "suggestions")]
fn did_you_mean() {
    assert!(utils::compare_output(
        utils::complex_app(),
        "clap-test --optio=foo",
        DYM,
        true
    ));
}

#[test]
fn issue_665() {
    let res = App::new("tester")
        .arg("-v, --reroll-count=[N] 'Mark the patch series as PATCH vN'")
        .arg(Arg::from(
"--subject-prefix [Subject-Prefix] 'Use [Subject-Prefix] instead of the standard [PATCH] prefix'") )
        .try_get_matches_from(vec!["test", "--subject-prefix", "-v", "2"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn issue_1047_min_zero_vals_default_val() {
    let m = App::new("foo")
        .arg(
            Arg::new("del")
                .short('d')
                .long("del")
                .setting(ArgSettings::RequireEquals)
                .min_values(0)
                .default_missing_value("default"),
        )
        .get_matches_from(vec!["foo", "-d"]);
    assert_eq!(m.occurrences_of("del"), 1);
    assert_eq!(m.value_of("del"), Some("default"));
}

fn issue_1105_setup(argv: Vec<&'static str>) -> Result<ArgMatches, clap::Error> {
    App::new("opts")
        .arg(Arg::from("-o, --option [opt] 'some option'").setting(ArgSettings::AllowEmptyValues))
        .arg(Arg::from("--flag 'some flag'"))
        .try_get_matches_from(argv)
}

#[test]
fn issue_1105_empty_value_long_fail() {
    let r = issue_1105_setup(vec!["app", "--option", "--flag"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn issue_1105_empty_value_long_explicit() {
    let r = issue_1105_setup(vec!["app", "--option", ""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}

#[test]
fn issue_1105_empty_value_long_equals() {
    let r = issue_1105_setup(vec!["app", "--option="]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}

#[test]
fn issue_1105_empty_value_short_fail() {
    let r = issue_1105_setup(vec!["app", "-o", "--flag"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn issue_1105_empty_value_short_explicit() {
    let r = issue_1105_setup(vec!["app", "-o", ""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}

#[test]
fn issue_1105_empty_value_short_equals() {
    let r = issue_1105_setup(vec!["app", "-o="]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}

#[test]
fn issue_1105_empty_value_short_explicit_no_space() {
    let r = issue_1105_setup(vec!["app", "-o", ""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}

#[test]
#[cfg(feature = "suggestions")]
fn issue_1073_suboptimal_flag_suggestion() {
    let app = App::new("ripgrep-616")
        .arg(Arg::new("files-with-matches").long("files-with-matches"))
        .arg(Arg::new("files-without-match").long("files-without-match"));
    assert!(utils::compare_output(
        app,
        "ripgrep-616 --files-without-matches",
        DYM_ISSUE_1073,
        true
    ));
}

#[test]
fn short_non_ascii_no_space() {
    let matches = App::new("app")
        .arg("<opt> -磨 <opt>")
        .get_matches_from(&["test", "-磨VALUE"]);

    assert_eq!("VALUE", matches.value_of("opt").unwrap());
}

#[test]
fn short_eq_val_starts_with_eq() {
    let matches = App::new("app")
        .arg("<opt> -f <opt>")
        .get_matches_from(&["test", "-f==value"]);

    assert_eq!("=value", matches.value_of("opt").unwrap());
}

#[test]
fn long_eq_val_starts_with_eq() {
    let matches = App::new("app")
        .arg("<opt> --foo <opt>")
        .get_matches_from(&["test", "--foo==value"]);

    assert_eq!("=value", matches.value_of("opt").unwrap());
}

#[test]
fn issue_2022_get_flags_misuse() {
    let app = App::new("test")
        .help_heading("test")
        .arg(Arg::new("a").long("a").default_value("32"));
    let matches = app.get_matches_from(&[""]);
    assert!(matches.value_of("a").is_some())
}

#[test]
fn issue_2279() {
    let before_help_heading = App::new("app")
        .arg(Arg::new("foo").short('f').default_value("bar"))
        .help_heading("This causes default_value to be ignored")
        .get_matches_from(&[""]);

    assert_eq!(before_help_heading.value_of("foo"), Some("bar"));

    let after_help_heading = App::new("app")
        .help_heading("This causes default_value to be ignored")
        .arg(Arg::new("foo").short('f').default_value("bar"))
        .get_matches_from(&[""]);

    assert_eq!(after_help_heading.value_of("foo"), Some("bar"));
}
