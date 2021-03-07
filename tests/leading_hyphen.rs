use clap::{App, Arg, AppSettings, ArgSettings, ErrorKind};

#[test]
fn leading_hyphen_short() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::new("some"))
        .arg(Arg::new("other").short('o'))
        .try_get_matches_from(vec!["", "-bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "-bar");
}

#[test]
fn leading_hyphen_long() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::new("some"))
        .arg(Arg::new("other").short('o'))
        .try_get_matches_from(vec!["", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "--bar");
}

#[test]
fn leading_hyphen_opt() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::new("some").takes_value(true).long("opt"))
        .arg(Arg::new("other").short('o'))
        .try_get_matches_from(vec!["", "--opt", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "--bar");
}

#[test]
fn allow_negative_numbers() {
    let res = App::new("negnum")
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(Arg::new("panum"))
        .arg(Arg::new("onum").short('o').takes_value(true))
        .try_get_matches_from(vec!["negnum", "-20", "-o", "-1.2"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("panum").unwrap(), "-20");
    assert_eq!(m.value_of("onum").unwrap(), "-1.2");
}

#[test]
fn allow_negative_numbers_fail() {
    let res = App::new("negnum")
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(Arg::new("panum"))
        .arg(Arg::new("onum").short('o').takes_value(true))
        .try_get_matches_from(vec!["negnum", "--foo", "-o", "-1.2"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument)
}

#[test]
fn leading_double_hyphen_trailingvararg() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from("[opt]... 'some pos'"))
        .get_matches_from(vec!["", "--foo", "-Wl", "bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["--foo", "-Wl", "bar"]
    );
}
#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args() {
    let res = App::new("prog")
        .global_setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from("--some-argument"))
        .try_get_matches_from(vec!["prog", "hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args_no_vals() {
    let res = App::new("prog")
        .global_setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from("--some-argument"))
        .try_get_matches_from(vec!["prog", "--hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args_option() {
    let res = App::new("prog")
        .global_setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from("--some-argument=[val]"))
        .try_get_matches_from(vec!["prog", "-hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn leading_hyphen_pass() {
    let r = App::new("mvae")
        .arg(Arg::from("-o [opt]... 'some opt'").setting(ArgSettings::AllowHyphenValues))
        .try_get_matches_from(vec!["", "-o", "-2", "3"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["-2", "3"]);
}

#[test]
fn leading_hyphen_fail() {
    let r = App::new("mvae")
        .arg(Arg::from("-o [opt] 'some opt'"))
        .try_get_matches_from(vec!["", "-o", "-2"]);
    assert!(r.is_err());
    let m = r.unwrap_err();
    assert_eq!(m.kind, ErrorKind::UnknownArgument);
}

#[test]
fn leading_hyphen_with_flag_after() {
    let r = App::new("mvae")
        .arg(Arg::from("-o [opt]... 'some opt'").setting(ArgSettings::AllowHyphenValues))
        .arg("-f 'some flag'")
        .try_get_matches_from(vec!["", "-o", "-2", "-f"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["-2", "-f"]);
    assert!(!m.is_present("f"));
}

#[test]
fn leading_hyphen_with_flag_before() {
    let r = App::new("mvae")
        .arg(Arg::from("-o [opt]... 'some opt'").setting(ArgSettings::AllowHyphenValues))
        .arg("-f 'some flag'")
        .try_get_matches_from(vec!["", "-f", "-o", "-2"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["-2"]);
    assert!(m.is_present("f"));
}

#[test]
fn leading_hyphen_with_only_pos_follows() {
    let r = App::new("mvae")
        .arg(
            Arg::from("-o [opt]... 'some opt'")
                .number_of_values(1)
                .setting(ArgSettings::AllowHyphenValues),
        )
        .arg("[arg] 'some arg'")
        .try_get_matches_from(vec!["", "-o", "-2", "--", "val"]);
    assert!(r.is_ok(), "{:?}", r);
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["-2"]);
    assert_eq!(m.value_of("arg"), Some("val"));
}

#[test]
fn double_hyphen_as_value() {
    let res = App::new("prog")
        .arg(
            Arg::new("cfg")
                .setting(ArgSettings::AllowHyphenValues)
                .long("config"),
        )
        .try_get_matches_from(vec!["prog", "--config", "--"]);
    assert!(res.is_ok(), "{:?}", res);
    assert_eq!(res.unwrap().value_of("cfg"), Some("--"));
}

#[test]
fn multiple_vals_with_hyphen() {
    let res = App::new("do")
        .arg(
            Arg::new("cmds")
                .multiple(true)
                .allow_hyphen_values(true)
                .value_terminator(";"),
        )
        .arg(Arg::new("location"))
        .try_get_matches_from(vec![
            "do",
            "find",
            "-type",
            "f",
            "-name",
            "special",
            ";",
            "/home/clap",
        ]);
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);

    let m = res.unwrap();
    let cmds: Vec<_> = m.values_of("cmds").unwrap().collect();
    assert_eq!(&cmds, &["find", "-type", "f", "-name", "special"]);
    assert_eq!(m.value_of("location"), Some("/home/clap"));
}

#[test]
fn allow_leading_hyphen_positional() {
    static COPY_ABOUT: &str = "Copy mode accepts two parts of arguments <from> and <to>,\
the two parts are separated by [--]. <from> is the exact as Consumer\
 mode, and <to> is the exact as Producer mode.";

    let res = App::new("copy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(
            Arg::new("from")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::new("to")
                .takes_value(true)
                .multiple(true)
                .last(true)
                .required(true),
        )
        .about(COPY_ABOUT)
        .try_get_matches_from(&[
            "copy",
            "-b",
            "localhost",
            "-t",
            "topic1",
            "--foo",
            "bar",
            "-e",
            "--",
            "-b",
            "localhost",
            "-t",
            "topic2",
            "--foo",
            "bar",
        ]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(
        m.values_of("from").unwrap().collect::<Vec<_>>(),
        vec!["-b", "localhost", "-t", "topic1", "--foo", "bar", "-e",]
    );
    assert_eq!(
        m.values_of("to").unwrap().collect::<Vec<_>>(),
        vec!["-b", "localhost", "-t", "topic2", "--foo", "bar"]
    );
}

#[test]
fn allow_hyphen_values_positional() {
    // Should have the same effect with `allow_hyphen_values_positional()`;
    static COPY_ABOUT: &str = "Copy mode accepts two parts of arguments <from> and <to>,\
the two parts are separated by [--]. <from> is the exact as Consumer\
 mode, and <to> is the exact as Producer mode.";

    let res = App::new("copy")
        .arg(
            Arg::new("from")
                .takes_value(true)
                .multiple(true)
                .setting(ArgSettings::AllowHyphenValues)
                .required(true),
        )
        .arg(
            Arg::new("to")
                .takes_value(true)
                .multiple(true)
                .last(true)
                .setting(ArgSettings::AllowHyphenValues)
                .required(true),
        )
        .about(COPY_ABOUT)
        .try_get_matches_from(&[
            "copy",
            "-b",
            "localhost",
            "-t",
            "topic1",
            "--foo",
            "bar",
            "-e",
            "--",
            "-b",
            "localhost",
            "-t",
            "topic2",
            "--foo",
            "bar",
        ]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(
        m.values_of("from").unwrap().collect::<Vec<_>>(),
        vec!["-b", "localhost", "-t", "topic1", "--foo", "bar", "-e",]
    );
    assert_eq!(
        m.values_of("to").unwrap().collect::<Vec<_>>(),
        vec!["-b", "localhost", "-t", "topic2", "--foo", "bar"]
    );
}
