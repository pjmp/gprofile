use std::{env, error::Error, process};

mod gprofile;

fn main() {
    if let Err(err) = app_main() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn app_main() -> Result<(), Box<dyn Error>> {
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
                gprofile::help();
            }
            "--version" | "-v" => {
                println!(env!("CARGO_PKG_VERSION"))
            }
            cmd => {
                let pair = cmd.split('=').collect::<Vec<_>>();

                if pair.len() != 2 {
                    eprintln!(
                        "{}: invalid option passed, options should be a pair separated by `=`\nExample: --delete=<PROFILE_NAME>",
                        env!("CARGO_PKG_NAME")
                    );
                    process::exit(1)
                }

                match pair[..] {
                    ["--delete" | "-d", profile] => {
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
                            "{}: unknown option passed, run `--help` to get available options.",
                            env!("CARGO_PKG_NAME")
                        );
                        process::exit(1)
                    }
                }
            }
        }
    } else {
        gprofile::help();
        process::exit(1);
    }

    Ok(())
}
