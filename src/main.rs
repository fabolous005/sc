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
        backend::CrosstermBackend,
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

fn quit(terminal: Terminal<CrosstermBackend<Stdout>>) {
    close_ui(terminal).unwrap();
    // TODO quit programm
}

fn main() -> Result<(), Error> {
    // open terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend      = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    /*
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    */

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




    terminal.draw(|f| {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(ascii)        
            .title_alignment(Alignment::Center);
        let size = f.size();

        /*
        let block2 = Block::default()
            .borders(Borders::ALL)
            .style(
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD)
            );
        */
        // let current = f.size();
        /*
        let size2 = Rect::new(
            current.x / 2 - 4,
            current.y / 2 - 8,
            // 3,
            // 3,
            8,
            4
        );
        */
        f.render_widget(block, size);
    })?;

    // terminal.draw(|f|{});

    // let mut frame = terminal.get_frame();
      // let frame     = terminal.get_frame();
    terminal.flush()?;

    let mut frame = terminal.get_frame();

    let paragraph = Paragraph::new("text");
    let area      = Rect::new(5, 3, 8, 3);
    frame.render_widget(paragraph, area);
    terminal.draw(|frame|{});
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