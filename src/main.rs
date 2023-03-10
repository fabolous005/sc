#[allow(
    unused_imports,
    unused_import_braces,
    unused_variables
)]

use {
    std::{
        thread,
        time::Duration,
        io::{
            stdin,
            stdout,
            Read,
            Error,
            Stdout,
            BufRead,
            StdinLock
        },
    },
    tui::{
        Terminal,
        Frame,
        backend::{
            CrosstermBackend,
            Backend
        },
        widgets::{
            Wrap,
            Paragraph,
            Widget,
            Block,
            Borders,
            List,
            ListState,
            ListItem
        },
        layout::{
            Layout,
            Constraint,
            Direction,
            Alignment,
            Rect
        },
        style::{
            Style,
            Color,
            Modifier
        }
    },
    num::{
        ToPrimitive
    },
    crossterm::{
        execute,
        event::{
            self,
            DisableMouseCapture,
            EnableMouseCapture,
            Event,
            KeyCode
        },
        terminal::{
            disable_raw_mode,
            enable_raw_mode,
            EnterAlternateScreen,
            LeaveAlternateScreen
        },
    },
};

mod model;
use crate::model::{
    Command,
    LangFunc
};


fn close_ui(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), Error> {
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn quit(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    close_ui(terminal).unwrap();
    // TODO quit programm
}

fn frame<B: Backend>(f: &mut Frame<B>) {

}

fn run_sc<B: Backend>(terminal: &mut Terminal<B>, sc: &mut Sc) -> Result<()> {
    loop {
        terminal.draw(|f| frame(f))?;

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}

fn get_sc(args: Args) -> &mut Sc {

}

fn main() -> Result<(), Error> {
    // open terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend      = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let _commands: Vec<Command> = vec![
        Command {
            name: "q".to_string(),
            func: LangFunc::Rust(quit),
            sc  : None
        }
    ];
    let mut input: [u8; 1]     = [0];
    let stdin                  = stdin();


    let mut handle = stdin.lock();
    handle.read_exact(&mut input)?;
    let ascii: String = input[0].to_string();
    // thread::sleep(Duration::from_millis(1000));





    terminal.flush()?;

    let mut frame = terminal.get_frame();

    let paragraph = Paragraph::new("text");
    let area      = Rect::new(5, 3, 8, 3);
    frame.render_widget(paragraph, area);
    terminal.draw(|frame|{})?;
    thread::sleep(Duration::from_millis(1000));

    /*
    TODO check which is the right stdin function to use
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read message");
    */

    close_ui(terminal)?;
    Ok(())
}