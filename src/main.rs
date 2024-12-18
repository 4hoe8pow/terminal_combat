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
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        terminal.clear()?;
        loop {
            // Drawing the terminal
            terminal.draw(|f| {
                let size = f.area();
                let layout = self.create_layout(size);
                let list = self.create_list();
                f.render_widget(list, layout[0]);

                // Drawing the selected item area
                let selected_text = format!("Selected: {}", self.selected_item);
                let selected_paragraph = Paragraph::new(Span::raw(selected_text)).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Selected Item"),
                );
                f.render_widget(selected_paragraph, layout[1]);

                // Drawing the message area
                let message_paragraph = Paragraph::new(Span::raw(&self.selected_message))
                    .block(Block::default().borders(Borders::ALL).title("Message"));
                f.render_widget(message_paragraph, layout[2]);
            })?;

            // Handling key inputs
            if event::poll(std::time::Duration::from_millis(200))? {
                if let event::Event::Key(key_event) = event::read()? {
                    if self.handle_key_event(key_event) {
                        return Ok(());
                    }
                }
            }
        }
    }

    // Generate layout
    fn create_layout(&self, size: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(60), // Top 60%
                    Constraint::Percentage(20), // Middle 20%
                    Constraint::Percentage(20), // Bottom 20%
                ]
                .as_ref(),
            )
            .split(size)
            .to_vec()
    }

    // Generate list of items
    fn create_list(&self) -> List {
        let items: Vec<ListItem> = self
            .choices
            .iter()
            .enumerate()
            .map(|(i, choice)| {
                let style = if i == self.selected_index {
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

    // Handle key events
    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        if key_event.kind != KeyEventKind::Press {
            return false; // Disable long-press
        }
        match key_event.code {
            KeyCode::Esc => {
                return true; // Exit with ESC key
            }
            KeyCode::Down => {
                self.selected_index = (self.selected_index + 1) % self.choices.len();
                self.update_message(); // Update message
            }
            KeyCode::Up => {
                self.selected_index = if self.selected_index == 0 {
                    self.choices.len() - 1
                } else {
                    self.selected_index - 1
                };
                self.update_message(); // Update message
            }
            KeyCode::Enter => {
                // Store the selected item
                self.selected_item = self.choices[self.selected_index].clone();
                self.update_message(); // Update message
            }
            _ => {}
        }
        false
    }

    // Update the message based on the selected item
    fn update_message(&mut self) {
        // Set messages corresponding to each choice
        let messages = [
            "You selected Choice 1!",
            "You selected Choice 2!",
            "You selected Choice 3!",
        ];
        self.selected_message = messages[self.selected_index].to_string();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // Initialize Terminal
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize App
    let mut app = App {
        choices: vec![
            "Choice 1".to_string(),
            "Choice 2".to_string(),
            "Choice 3".to_string(),
        ],
        ..Default::default()
    };

    // Run the app
    app.run(&mut terminal).await?;

    // Restore terminal settings on exit
    ratatui::restore();

    Ok(())
}
