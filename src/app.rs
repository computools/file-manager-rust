use std::{env, fs, path::PathBuf};

use anyhow::Result;
use ratatui::widgets::ListState;

use crate::files;

#[derive(Debug)]
pub struct App {
    // General
    pub running: bool,

    // Files
    pub current_dir: PathBuf,
    pub files: Vec<files::File>,
    pub files_state: ListState,
    pub show_hidden: bool,

    // Help
    pub show_help: bool,
    pub help_offset: usize,
}

impl App {
    pub fn new() -> Result<Self> {
        let current_dir = env::current_dir()?;

        Ok(App {
            // General
            running: true,

            // Files
            current_dir: current_dir.clone(),
            files: files::list_dir(current_dir, false)?,
            files_state: ListState::default().with_selected(Some(0)),
            show_hidden: false,

            // Help
            show_help: false,
            help_offset: 0,
        })
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select_next(&mut self) {
        if let Some(selected) = self.files_state.selected() {
            if selected == self.files.len() - 1 {
                self.files_state.select_first();
                return;
            }
        }

        self.files_state.select_next();
    }

    pub fn select_previous(&mut self) {
        if let Some(selected) = self.files_state.selected() {
            if selected == 0 {
                self.files_state.select_last();
                return;
            }
        }

        self.files_state.select_previous();
    }

    pub fn toggle_hidden(&mut self) -> Result<()> {
        self.show_hidden = !self.show_hidden;
        self.refresh_files(None)?;

        Ok(())
    }

    pub fn return_path(&mut self) -> Result<()> {
        if let Some(parent_dir) = self.current_dir.parent() {
            self.refresh_files(Some(PathBuf::from(parent_dir)))?;
        }

        Ok(())
    }

    pub fn open(&mut self) -> Result<()> {
        if let Some(selected) = self.files_state.selected() {
            let file = self.files.get(selected).unwrap();
            if file.is_dir {
                self.refresh_files(Some(self.current_dir.join(&file.name)))?;
                return Ok(());
            }

            open::that(self.current_dir.join(&file.name))?;
        }

        Ok(())
    }

    pub fn move_to_trash(&mut self) -> Result<()> {
        if let Some(selected) = self.files_state.selected() {
            let file = self.files.get(selected).unwrap();
            trash::delete(self.current_dir.join(&file.name))?;
            self.refresh_files(None)?;
        }

        Ok(())
    }

    pub fn remove_file(&mut self) -> Result<()> {
        if let Some(selected) = self.files_state.selected() {
            let file = self.files.get(selected).unwrap();
            fs::remove_file(self.current_dir.join(&file.name))?;
            self.refresh_files(None)?;
        }

        Ok(())
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn get_file_names(&self) -> Vec<String> {
        self.files.iter().map(|f| f.formated_name.clone()).collect()
    }

    pub fn get_files_selected(&self) -> usize {
        if let Some(selected) = self.files_state.selected() {
            return selected;
        }

        0
    }

    pub fn get_preview(&self) -> String {
        if let Some(selected) = self.files_state.selected() {
            let file = self.files.get(selected).unwrap();
            if file.is_dir {
                return files::list_dir(self.current_dir.join(&file.name), self.show_hidden)
                    .unwrap()
                    .iter()
                    .map(|f| f.formated_name.clone())
                    .collect::<Vec<_>>()
                    .join("\n");
            }

            return fs::read_to_string(self.current_dir.join(&file.name))
                .unwrap_or("--Cannot read file--".to_string());
        }

        String::new()
    }

    fn refresh_files(&mut self, path: Option<PathBuf>) -> Result<()> {
        if let Some(path_buf) = path {
            self.current_dir = path_buf;
        }

        self.files = files::list_dir(self.current_dir.clone(), self.show_hidden)?;
        self.files_state.select_first();

        Ok(())
    }
}
