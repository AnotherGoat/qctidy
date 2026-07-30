use crossterm::event::KeyCode;
use qctidy::{Circuit, GateOperation};
use ratatui::widgets::ListState;

use crate::picker::{AddState, format_gate};

pub enum AppMode {
    Normal,
    Adding(AddState),
    Importing {
        buffer: String,
    },
    Exporting {
        format_index: usize,
        buffer: String,
        step: ExportStep,
    },
    Help,
}

pub enum ExportStep {
    SelectingFormat,
    EnteringFilename,
}

pub struct App {
    pub operations: Vec<GateOperation>,
    pub list_state: ListState,
    pub mode: AppMode,
    pub message: Option<String>,
    pub message_timer: u8,
    pub scroll_offset: u16,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            operations: Vec::new(),
            list_state,
            mode: AppMode::Normal,
            message: None,
            message_timer: 0,
            scroll_offset: 0,
            should_quit: false,
        }
    }

    pub fn add_operation(&mut self, op: GateOperation) {
        self.operations.push(op);
        let last = self.operations.len().saturating_sub(1);
        self.list_state.select(Some(last));
    }

    pub fn set_operations(&mut self, ops: Vec<GateOperation>) {
        self.operations = ops;
        self.list_state.select(Some(0));
    }

    pub fn remove_selected(&mut self) {
        if let Some(i) = self.list_state.selected()
            && i < self.operations.len()
        {
            let _ = self.operations.remove(i);
            let len = self.operations.len();
            self.list_state.select(if len == 0 {
                None
            } else if i >= len {
                Some(len - 1)
            } else {
                Some(i)
            });
        }
    }

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
        self.message_timer = 10;
    }

    pub fn tick(&mut self) {
        if self.message_timer > 0 {
            self.message_timer -= 1;
            if self.message_timer == 0 {
                self.message = None;
            }
        }
    }

    #[expect(clippy::wildcard_enum_match_arm)]
    pub fn handle_key(&mut self, key: KeyCode) {
        match &mut self.mode {
            AppMode::Normal => match key {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Char('h' | 'H') => {
                    self.mode = AppMode::Help;
                }
                KeyCode::Char('a' | 'A') => {
                    self.mode = AppMode::Adding(AddState::new());
                }
                KeyCode::Char('r' | 'R') => {
                    self.remove_selected();
                }
                KeyCode::Char('i' | 'I') => {
                    self.mode = AppMode::Importing {
                        buffer: String::new(),
                    };
                }
                KeyCode::Char('e' | 'E') => {
                    self.mode = AppMode::Exporting {
                        format_index: 0,
                        buffer: String::new(),
                        step: ExportStep::SelectingFormat,
                    };
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if let Some(i) = self.list_state.selected()
                        && i > 0
                    {
                        self.list_state.select(Some(i - 1));
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if let Some(i) = self.list_state.selected()
                        && i + 1 < self.operations.len()
                    {
                        self.list_state.select(Some(i + 1));
                    }
                }
                KeyCode::Left => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(2);
                }
                KeyCode::Right => {
                    self.scroll_offset = self.scroll_offset.saturating_add(2);
                }
                _ => {}
            },
            AppMode::Adding(state) => match key {
                KeyCode::Esc => {
                    state.handle_escape();
                    if matches!(state, AddState::Selecting { query, .. } if query.is_empty()) {
                        self.mode = AppMode::Normal;
                    }
                }
                KeyCode::Enter => match state.handle_enter() {
                    Ok(Some(op)) => {
                        let name = format_gate(&op);
                        self.add_operation(op);
                        self.set_message(format!("Added {name}"));
                        self.mode = AppMode::Normal;
                    }
                    Ok(None) => {}
                    Err(e) => {
                        self.set_message(format!("Error: {e}"));
                    }
                },
                KeyCode::Tab => {
                    state.handle_tab();
                }
                KeyCode::Up | KeyCode::Left | KeyCode::Char('k') => {
                    state.handle_up();
                }
                KeyCode::Down | KeyCode::Right | KeyCode::Char('j') => {
                    state.handle_down();
                }
                KeyCode::Char(c) => {
                    state.handle_char(c);
                }
                KeyCode::Backspace => {
                    state.handle_backspace();
                }
                _ => {}
            },
            AppMode::Importing { buffer } => {
                match key {
                    KeyCode::Esc => {
                        self.mode = AppMode::Normal;
                    }
                    KeyCode::Enter => {
                        let path = buffer.trim().to_owned();
                        if path.is_empty() {
                            self.set_message("No path entered".to_owned());
                            self.mode = AppMode::Normal;
                            return;
                        }
                        let data = match std::fs::read(&path) {
                            Ok(d) => d,
                            Err(e) => {
                                self.set_message(format!("Cannot read '{path}': {e}"));
                                self.mode = AppMode::Normal;
                                return;
                            }
                        };
                        let fmt = crate::picker::detect_format(&path);
                        let fmt = match fmt {
                            Some(f) => f,
                            None => {
                                self.set_message(format!("Unknown format for '{path}'. Use .json, .xml, .cbor, or .msgpack"));
                                self.mode = AppMode::Normal;
                                return;
                            }
                        };
                        match crate::picker::import_circuit(&data, fmt) {
                            Ok(circuit) => {
                                let count = circuit.operations().len();
                                self.set_operations(circuit.operations().to_vec());
                                self.set_message(format!("Imported {count} gates from '{path}'"));
                            }
                            Err(e) => {
                                self.set_message(format!("Parse error: {e}"));
                            }
                        }
                        self.mode = AppMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        buffer.pop();
                    }
                    _ => {}
                }
            }
            AppMode::Exporting {
                format_index,
                buffer,
                step,
            } => match key {
                KeyCode::Esc => match step {
                    ExportStep::SelectingFormat => {
                        self.mode = AppMode::Normal;
                    }
                    ExportStep::EnteringFilename => {
                        *step = ExportStep::SelectingFormat;
                    }
                },
                KeyCode::Up | KeyCode::Char('k') => {
                    if matches!(step, ExportStep::SelectingFormat) && *format_index > 0 {
                        *format_index -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if matches!(step, ExportStep::SelectingFormat)
                        && *format_index + 1 < crate::picker::FORMATS.len()
                    {
                        *format_index += 1;
                    }
                }
                KeyCode::Enter => match step {
                    ExportStep::SelectingFormat => {
                        let (_, ext, _) = crate::picker::FORMATS[*format_index];
                        let default_name = format!("circuit{ext}");
                        *step = ExportStep::EnteringFilename;
                        buffer.clear();
                        buffer.push_str(&default_name);
                    }
                    ExportStep::EnteringFilename => {
                        let path = buffer.trim().to_owned();
                        if path.is_empty() {
                            self.set_message("No filename entered".to_owned());
                            self.mode = AppMode::Normal;
                            return;
                        }
                        let (name, _, _) = crate::picker::FORMATS[*format_index];
                        let format_name = name.to_lowercase();
                        let circuit = Circuit::from_operations(self.operations.clone());
                        let bytes = match crate::picker::export_circuit(&circuit, &format_name) {
                            Ok(b) => b,
                            Err(e) => {
                                self.set_message(format!("Serialize error: {e}"));
                                self.mode = AppMode::Normal;
                                return;
                            }
                        };
                        match std::fs::write(&path, &bytes) {
                            Ok(()) => {
                                self.set_message(format!("Exported to '{path}'"));
                            }
                            Err(e) => {
                                self.set_message(format!("Cannot write '{path}': {e}"));
                            }
                        }
                        self.mode = AppMode::Normal;
                    }
                },
                KeyCode::Char(c) => {
                    if matches!(step, ExportStep::EnteringFilename) {
                        buffer.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if matches!(step, ExportStep::EnteringFilename) {
                        buffer.pop();
                    }
                }
                _ => {}
            },
            AppMode::Help => match key {
                KeyCode::Char('h' | 'H') | KeyCode::Esc => {
                    self.mode = AppMode::Normal;
                }
                _ => {}
            },
        }
    }

    /// Return a short contextual status line for the current mode.
    pub fn status_line(&self) -> String {
        match self.mode {
            AppMode::Normal => {
                let mut line = String::from("  [A]dd gate");
                if !self.operations.is_empty() {
                    line.push_str("  [R]emove");
                }
                line.push_str("  [I]mport  [E]xport  [H]elp  [q]uit");
                line
            }
            AppMode::Adding(AddState::Selecting { .. }) => {
                "  Enter: select gate  Tab/↓: next  ↑: prev  Esc: cancel".to_owned()
            }
            AppMode::Adding(AddState::EnteringArgs { .. }) => {
                "  Enter: confirm arg  Esc: back to gate selection".to_owned()
            }
            AppMode::Importing { .. } => "  Enter: import  Esc: cancel".to_owned(),
            AppMode::Exporting {
                step: ExportStep::SelectingFormat,
                ..
            } => "  Enter: select format  ↑/↓: navigate  Esc: cancel".to_owned(),
            AppMode::Exporting {
                step: ExportStep::EnteringFilename,
                ..
            } => "  Enter: save  Esc: back to format selection".to_owned(),
            AppMode::Help => "  H or Esc: close help".to_owned(),
        }
    }
}
