use std::io::{self, Write};
use crossterm::terminal::{
    self,
    size,
    enable_raw_mode, 
    disable_raw_mode, 
    ClearType, 
    EnterAlternateScreen, 
    LeaveAlternateScreen};
use crossterm::{cursor, execute, queue, style};
use crossterm::event::{self, Event, KeyCode};

// State representation - we need to save where the cursor is at (x, y) and the text being written
struct Editor {
    x: u16,
    y: u16,
    buffer: Vec<String>,
}

impl Editor {
    // constructor - creates a blank canvas and inits cursor postion to (0, 0)
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            buffer: vec![String::new()],
        }
    }
}

fn main() -> io::Result<()> {

    let mut editor = Editor::new();
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    execute!(stdout, EnterAlternateScreen)?;

    // Get window size
    let (width, _height) = size()?;

    loop {

        // STEP 1 -- Draw: Clear the screen and print Vec<String>

        // Clear screen and move invisible cursor to top left of screen (0, 0)
        queue!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
	
        // Print every line we have in memory
        for line in &editor.buffer {
            queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        // Adds a new String element to vector if the line at the current cursor is equal to the width of the window
        if editor.x == width {
            editor.buffer.push(String::new());
            editor.x = 0;
            editor.y += 1;
        }

        // Move actual cursor to where x, y variables are
        queue!(stdout, cursor::MoveTo(editor.x, editor.y))?;

        stdout.flush()?;

        // STEP 2 -- Read & Update: Use crossterm to capture a single press and update Vec<String> accordingly

        // Wait for user to input a key
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,

                KeyCode::Char(c) => {

                    let cur_line = &mut editor.buffer[editor.y as usize];
                    cur_line.insert(editor.x as usize, c);
                    editor.x += 1;

                },

                KeyCode::Backspace => {
                    if editor.x == 0 && editor.y == 0 {
                        continue;
                    } else if editor.x == 0 {
                        let prev_line = &mut editor.buffer[(editor.y as usize) - 1];
                        editor.x = prev_line.chars().count() as u16;
                        editor.y -= 1;
                        editor.buffer.pop();
                        
                    } else {
                        let cur_line = &mut editor.buffer[editor.y as usize];
                        cur_line.pop();
                        editor.x -= 1;
                    }
                },

                KeyCode::Enter => {
                    editor.buffer.push(String::new());
                    editor.x = 0;
                    editor.y += 1;
                },

                _ => {}

            }
        }
        
    }

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    
    Ok(())
}
