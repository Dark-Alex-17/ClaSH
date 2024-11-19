use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, CommandFactory, Parser};
use cli::{handle_command, Command};
use completer::ClashShellHelper;
use indoc::formatdoc;
use rustyline::{error::ReadlineError, Config, Editor};

mod cli;
mod completer;

#[derive(Debug, Parser)]
#[command(
  name = crate_name!(),
  author = crate_authors!(),
  version = crate_version!(),
  about = crate_description!(),
  disable_version_flag = true,
  disable_help_subcommand = true,
  disable_help_flag = true,
  override_usage = "[COMMAND]",
  help_template = "\
{before-help}{name} {version}
{author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

fn main() -> Result<()> {
    let config = Config::builder().history_ignore_dups(true)?.build();
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(ClashShellHelper::new()));

    println!(
        "{}",
        formatdoc!(
            "Welcome to Alex Clarke's personal website! 
            Here's some commands to get you started:
            
            resume - View my Resume
            projects - See a list of projects that I'm working on that you can check out
            github - Output my GitHub profile
            linkedin - Output my LinkedIn
            email - Output my email
            help - See all the commands you can run"
        )
    );

    loop {
        let readline = rl.readline("clash> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.trim())?;

                let args = Cli::try_parse_from(format!("clash {}", line).split_whitespace());
                match args {
                    Ok(shell_args) => {
                        if let Some(cmd) = shell_args.command {
                            match cmd {
                                Command::Help => Cli::command().print_long_help()?,
                                Command::Exit => {
                                    println!("Goodbye!");
                                    break;
                                }
                                _ => handle_command(cmd)?,
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => {}
            Err(err) => {
                eprintln!("Error: {err}");
            }
        }
    }

    let _ = rl.save_history("history.txt");
    Ok(())
}
