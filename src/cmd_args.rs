extern crate clap;

use self::clap::{App, AppSettings, Arg, SubCommand};
use std::ffi::OsString;

#[derive(Debug, PartialEq)]
pub struct CLArgs {
    pub list: bool,
    pub crate_dir: String,
    pub gen_id: Option<String>,
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
                    )
                    .arg(
                        Arg::with_name("crate-dir")
                            .help("The root directory of the crate to run the generator on")
                            .long("crate-dir")
                            .short("d")
                            .takes_value(true),
                    ),
            )
            .get_matches_from(args);
        let gen_args = args.subcommand_matches("gen").unwrap();
        let is_list = gen_args.is_present("list");
        let crate_dir = gen_args.value_of("crate-dir").unwrap_or(".").to_string();
        let (gen_id, subcmd_args) = match gen_args.subcommand() {
            (subcmd, Some(subcmd_args)) => {
                let subcmd_args = match subcmd_args.values_of("") {
                    Some(subcmd_args) => subcmd_args.map(|s| s.to_owned()).collect(),
                    None => vec![],
                };
                (Some(subcmd.to_owned()), subcmd_args)
            }
            _ => (None, vec![]),
        };
        CLArgs {
            list: is_list,
            crate_dir: crate_dir,
            gen_id: gen_id,
            gen_args: subcmd_args,
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
    fn it_sets_the_crate_dir() {
        assert_eq!(
            "/tmp/the-crate",
            CLArgs::parse(args(&["--crate-dir", "/tmp/the-crate", "app"])).crate_dir
        );
        assert_eq!(
            "/tmp/the-crate",
            CLArgs::parse(args(&["-d", "/tmp/the-crate", "app"])).crate_dir
        );
    }

    #[test]
    fn it_defaults_the_crate_dir_to_the_current_dir_if_unset() {
        assert_eq!(".", CLArgs::parse(args(&["app"])).crate_dir);
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
