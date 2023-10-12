use bad_apple_rust::app::{App, AppResult};
use bad_apple_rust::event::{Event, EventHandler};
use bad_apple_rust::handler::handle_key_events;
use bad_apple_rust::tui::Tui;
use bad_apple_rust::txt;
use std::io;

use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Generate txt files from images.
    txt::generate_txt(true, 360, 280);

    // Create an application.
    let mut app = App::new("txts");

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(200);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app).unwrap();

        // Handle events.
        match tui.events.next().unwrap() {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app).unwrap(),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
