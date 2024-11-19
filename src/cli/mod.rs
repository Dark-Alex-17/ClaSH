use std::io::{stdout, Write};
use std::sync::LazyLock;

use anyhow::Result;
use clap::Subcommand;
use projects::PROJECTS;
use resume::RESUME;
use termimad::crossterm::cursor::{Hide, Show};
use termimad::crossterm::event::{Event, KeyCode, KeyEvent};
use termimad::crossterm::style::Color::*;
use termimad::crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use termimad::crossterm::{event, queue, terminal};
use termimad::{rgb, Alignment, Area, MadSkin, MadView};

mod projects;
mod resume;

#[derive(Debug, Clone, Copy, Subcommand, PartialEq, Eq)]
pub enum Command {
    #[command(about = "Display my Resume", override_usage = "resume")]
    Resume,
    #[command(
        about = "See a list of projects that I'm working on that you can check out",
        override_usage = "projects"
    )]
    Projects,
    #[command(about = "Output my GitHub Profile", override_usage = "github")]
    Github,
    #[command(about = "Output my LinkedIn Profile", override_usage = "linkedin")]
    Linkedin,
    #[command(about = "Output my email address", override_usage = "email")]
    Email,
    #[command(
        about = "Print this message or the help of the given subcommand(s)",
        override_usage = "help"
    )]
    Help,
    #[command(about = "Exit this ClaSH session", override_usage = "exit")]
    Exit,
}

pub static COMMANDS: LazyLock<[String; 8]> = LazyLock::new(|| {
    [
        "resume".into(),
        "projects".into(),
        "github".into(),
        "linkedin".into(),
        "email".into(),
        "help".into(),
        "exit".into(),
        "--help".into(),
    ]
});

pub(super) fn handle_command(command: Command) -> Result<()> {
    match command {
        Command::Resume => display_markdown(RESUME)?,
        Command::Projects => display_markdown(PROJECTS)?,
        Command::Github => println!("https://github.com/Dark-Alex-17"),
        Command::Linkedin => println!("https://linkedin.com/in/alex-clarke-a78b99132"),
        Command::Email => println!("alex.j.tusa@gmail.com"),
        _ => (),
    }

    Ok(())
}

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(180);
    area
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.paragraph.align = Alignment::Left;
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}

fn display_markdown(text: &'static str) -> Result<()> {
    let mut w = stdout();
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?;
    let mut view = MadView::from(text.to_owned(), view_area(), make_skin());
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                KeyCode::Up | KeyCode::Char('k') => view.try_scroll_lines(-1),
                KeyCode::Down | KeyCode::Char('j') => view.try_scroll_lines(1),
                KeyCode::PageUp => view.try_scroll_pages(-1),
                KeyCode::PageDown => view.try_scroll_pages(1),
                _ => break,
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}
