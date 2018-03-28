extern crate clap;

use std::ffi::OsString;
use self::clap::{App, AppSettings, Arg, SubCommand};

#[derive(Debug, PartialEq)]
pub struct CLArgs {
    pub list: bool,
    gen_id: Option<String>,
    gen_args: Vec<String>,
}

impl CLArgs {
    pub fn parse<I, T>(args: I) -> CLArgs
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let args = App::new("")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                SubCommand::with_name("gen")
                    .settings(&[
                        AppSettings::ArgRequiredElseHelp,
                        AppSettings::AllowExternalSubcommands,
                    ])
                    .arg(
                        Arg::with_name("list")
                            .help("List the available generators")
                            .long("list")
                            .short("l")
                            .conflicts_with("GENERATOR_NAME"),
                    ),
            )
            .get_matches_from(args);
        let gen_args = args.subcommand_matches("gen").unwrap();
        match gen_args.subcommand() {
            (subcmd, Some(subcmd_args)) => {
                let subcmd_args = match subcmd_args.values_of("") {
                    Some(subcmd_args) => subcmd_args.map(|s| s.to_owned()).collect(),
                    None => vec![],
                };
                CLArgs {
                    list: false,
                    gen_id: Some(subcmd.to_owned()),
                    gen_args: subcmd_args,
                }
            }
            _ => CLArgs {
                list: gen_args.is_present("list"),
                gen_id: None,
                gen_args: vec![],
            },
        }
    }
}

#[cfg(test)]
mod arg_parsing {
    use super::CLArgs;
    use std::vec::IntoIter;

    fn args<'a>(suffix: &'a [&str]) -> IntoIter<&'a str> {
        let mut a = vec!["cargo", "gen"];
        a.extend(suffix.iter());
        a.into_iter()
    }

    // Tests of invalid arguments are integration tests because they cause the process to exit
    // which is what we want in those cases. Only the successful examples are tested below.

    #[test]
    fn it_sets_the_list_flag() {
        assert_eq!(false, CLArgs::parse(args(&["app"])).list);
        assert_eq!(true, CLArgs::parse(args(&["-l"])).list);
        assert_eq!(true, CLArgs::parse(args(&["--list"])).list);
    }

    #[test]
    fn it_accepts_a_generator_identifier() {
        assert_eq!(
            Some("app".to_string()),
            CLArgs::parse(args(&["app"])).gen_id
        );
    }

    #[test]
    fn it_gathers_the_remaining_arguments_into_generator_arguments() {
        assert_eq!(
            vec!["--dry-run", "--quiet"],
            CLArgs::parse(args(&["app", "--dry-run", "--quiet"])).gen_args
        );
    }
}
