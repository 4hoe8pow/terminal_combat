use color_eyre::eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
struct App {
    selected_index: usize,
    selected_item: String,    // Store the selected item
    selected_message: String, // Store the message related to the selected item
    choices: Vec<String>,
}

impl App {
    // Initialize the app
    pub fn new(choices: Vec<String>) -> Self {
        App {
            selected_index: 0,
            selected_item: String::new(),
            selected_message: String::new(),
            choices,
        }
    }

    // Update the message based on the selected item
    pub fn update_message(&mut self) {
        let messages = [
            "You selected Choice 1!",
            "You selected Choice 2!",
            "You selected Choice 3!",
        ];
        self.selected_message = messages[self.selected_index].to_string();
    }
}

struct InputHandler;

impl InputHandler {
    // Handle key events from the user
    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) -> bool {
        if key_event.kind != KeyEventKind::Press {
            return false; // Return false if it's not a key press
        }
        match key_event.code {
            KeyCode::Esc => true, // Exit when Esc is pressed
            KeyCode::Down => {
                app.select_next();
                false
            }
            KeyCode::Up => {
                app.select_previous();
                false
            }
            KeyCode::Enter => {
                app.confirm_selection();
                false
            }
            _ => false,
        }
    }
}

impl App {
    // Handle the logic for selecting the next item
    pub fn select_next(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.choices.len();
        self.update_message();
    }

    // Handle the logic for selecting the previous item
    pub fn select_previous(&mut self) {
        self.selected_index = if self.selected_index == 0 {
            self.choices.len() - 1
        } else {
            self.selected_index - 1
        };
        self.update_message();
    }

    // Confirm the selection and update the selected item
    pub fn confirm_selection(&mut self) {
        self.selected_item = self.choices[self.selected_index].clone();
        self.update_message();
    }
}

struct AppPresenter;

impl AppPresenter {
    // Render the app's state to the terminal
    pub fn render(app: &App, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
        terminal
            .draw(|f| {
                let size = f.area();
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                    ])
                    .split(size);

                // Rendering list of choices
                let list = AppPresenter::render_choices(app);
                f.render_widget(list, layout[0]);

                // Rendering selected item
                let selected_text = format!("Selected: {}", app.selected_item);
                let selected_paragraph = Paragraph::new(Span::raw(selected_text)).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Selected Item"),
                );
                f.render_widget(selected_paragraph, layout[1]);

                // Rendering message
                let message_paragraph = Paragraph::new(Span::raw(&app.selected_message))
                    .block(Block::default().borders(Borders::ALL).title("Message"));
                f.render_widget(message_paragraph, layout[2]);
            })
            .unwrap();
    }

    // Render the list of choices
    fn render_choices(app: &App) -> List {
        let items: Vec<ListItem> = app
            .choices
            .iter()
            .enumerate()
            .map(|(i, choice)| {
                let style = if i == app.selected_index {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                ListItem::new(Span::raw(choice)).style(style)
            })
            .collect();

        List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Choices"))
            .highlight_style(Style::default().fg(Color::Yellow))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize App
    let mut app = App::new(vec![
        "Choice 1".to_string(),
        "Choice 2".to_string(),
        "Choice 3".to_string(),
    ]);
    terminal.clear()?;

    loop {
        // Drawing the app
        AppPresenter::render(&app, &mut terminal);

        // Handling key events
        if event::poll(std::time::Duration::from_millis(500))? {
            if let event::Event::Key(key_event) = event::read()? {
                // If Esc is pressed, return to exit the loop
                if InputHandler::handle_key_event(&mut app, key_event) {
                    break; // Exit the loop
                }
            }
        }
    }

    Ok(())
}
