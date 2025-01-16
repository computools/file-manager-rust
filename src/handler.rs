use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, ui::HELP};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Char('c') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Up => {
            if app.show_help {
                if app.help_offset > 0 {
                    app.help_offset = app.help_offset - 1;
                }
            } else {
                app.select_previous();
            }
        }
        KeyCode::Down => {
            if app.show_help {
                if app.help_offset < (HELP.lines().count() - 1) {
                    app.help_offset = app.help_offset + 1;
                }
            } else {
                app.select_next();
            }
        }
        KeyCode::Char('h') => {
            if key_event.modifiers == KeyModifiers::CONTROL && !app.show_help {
                app.toggle_hidden()?;
            }
        }
        KeyCode::Backspace => {
            if !app.show_help {
                app.return_path()?;
            }
        }
        KeyCode::Enter => {
            if !app.show_help {
                app.open()?;
            }
        }
        KeyCode::Char('d') => {
            if !app.show_help {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.remove_file()?;
                } else {
                    app.move_to_trash()?;
                }
            }
        }
        KeyCode::Char('?') => {
            app.toggle_help();
        }
        _ => {}
    }
    Ok(())
}
