use std::{env, error::Error, fs::File, io, process};

use crate::cli::gprofile::db_file;

#[path = "./gprofile.rs"]
mod gprofile;

fn help() {
    eprintln!(
        "{app} {version}
{description}

USAGE:
  {app} [FLAGS]
  {app} [OPTIONS] <PROFILE>

FLAGS:
  -h, --help            Prints help information
  -v, --version         Prints version information

OPTIONS:
  -r, --remove=<PROFILE>  Delete a given profile
  -e, --edit=<PROFILE>    Edit a given profile
  -u, --use=<PROFILE>     Set the given profile as current user name and email
  -c, --create            Create a new profile
  -l, --list              List available profiles
  -d --dump-config        Dump config path & config to stdout
",
        app = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        description = env!("CARGO_PKG_DESCRIPTION")
    );
}

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    gprofile::init()?;

    if let Some(opt) = env::args().nth(1) {
        match opt.as_str() {
            "--create" | "-c" => {
                gprofile::create()?;
            }
            "--list" | "-l" => {
                gprofile::list()?;
            }
            "--help" | "-h" => {
                help();
            }
            "--version" | "-v" => {
                println!(env!("CARGO_PKG_VERSION"))
            }
            "--dump-config" | "-d" => {
                println!("{}\n", db_file().display());

                let mut file = File::open(db_file())?;

                io::copy(&mut file, &mut io::stdout())?;
            }
            cmd => {
                let pair = cmd.split('=').collect::<Vec<_>>();

                if pair.len() != 2 {
                    eprintln!(
                            "{}: invalid option passed, options should be a pair separated by `=`\nExample: --remove=<PROFILE_NAME>",
                            env!("CARGO_PKG_NAME")
                        );
                    process::exit(1)
                }

                match pair[..] {
                    ["--remove" | "-r", profile] => {
                        gprofile::delete(profile.to_string())?;
                    }
                    ["--edit" | "-e", profile] => {
                        gprofile::edit(profile.to_string())?;
                    }
                    ["--use" | "-u", profile] => {
                        gprofile::switch(profile.to_string())?;
                    }
                    _ => {
                        eprintln!(
                            "{}: unknown option, pass `--help` to see all available options.",
                            env!("CARGO_PKG_NAME")
                        );
                        process::exit(1)
                    }
                }
            }
        }
    } else {
        help();
        process::exit(1);
    }

    Ok(())
}
