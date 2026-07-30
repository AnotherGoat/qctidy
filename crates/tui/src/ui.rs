use qctidy::Circuit;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};

use crate::app::{App, AppMode, ExportStep};
use crate::picker::{ALL_ENTRIES, AddState, FORMATS, format_gate};

#[expect(clippy::integer_division)]
const fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let x_margin = (r.width.saturating_sub((r.width * percent_x) / 100)) / 2;
    let y_margin = (r.height.saturating_sub((r.height * percent_y) / 100)) / 2;
    Rect {
        x: r.x + x_margin,
        y: r.y + y_margin,
        width: r.width - 2 * x_margin,
        height: r.height - 2 * y_margin,
    }
}

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(area);

    // Title bar — no border, just colored text
    let title_style = if matches!(app.mode, AppMode::Adding(_)) {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
            .bg(Color::DarkGray)
    } else {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
            .bg(Color::DarkGray)
    };
    let title = Paragraph::new(" QCTidy Circuit Builder ").style(title_style);
    frame.render_widget(title, main_chunks[0]);

    // Circuit view
    let circuit = Circuit::from_operations(app.operations.clone());
    let circuit_text = circuit.to_string();
    let circuit_para = Paragraph::new(circuit_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Circuit ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .scroll((0, app.scroll_offset));
    frame.render_widget(circuit_para, main_chunks[1]);

    // Operations count — no border, subtle style
    let sep_text = format!("  Operations ({}) ", app.operations.len());
    let separator = Paragraph::new(sep_text).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(separator, main_chunks[2]);

    // Gates list
    let items: Vec<ListItem> = app
        .operations
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let text = format!("{:>4}: {}", i, format_gate(op));
            ListItem::new(text)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Gates ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");
    frame.render_stateful_widget(list, main_chunks[3], &mut app.list_state);

    // Bottom status bar (contextual)
    let bottom_bar = Paragraph::new(app.status_line())
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));
    frame.render_widget(bottom_bar, main_chunks[4]);

    // Overlays based on mode
    match &app.mode {
        AppMode::Adding(state) => render_picker(frame, area, main_chunks[4], state),
        AppMode::Help => render_help(frame, area),
        AppMode::Importing { buffer } => render_import_popup(frame, area, buffer),
        AppMode::Exporting {
            format_index,
            buffer,
            step,
        } => {
            render_export_popup(frame, area, *format_index, buffer, step);
        }
        AppMode::Normal => {}
    }

    // Toast notification overlay (auto-disappears)
    if let Some(msg) = &app.message {
        render_toast(frame, area, main_chunks[4], msg);
    }
}

fn render_picker(frame: &mut Frame, area: Rect, _status_area: Rect, state: &AddState) {
    match state {
        AddState::Selecting {
            query,
            indices,
            selected,
        } => {
            let picker_area = centered_rect(60, 55, area);
            frame.render_widget(Clear, picker_area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)])
                .split(picker_area);

            let gate_items: Vec<ListItem> = indices
                .iter()
                .map(|&idx| {
                    let entry = &ALL_ENTRIES[idx];
                    let name = entry.names[0];
                    let qubits = entry.qubits;
                    let info = format!("{name:<10}  ({qubits})");
                    ListItem::new(info)
                })
                .collect();

            let gate_list = List::new(gate_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Select Gate ")
                        .border_style(Style::default().fg(Color::Cyan)),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▸ ");

            let mut list_state = ratatui::widgets::ListState::default();
            list_state.select(Some(*selected));
            frame.render_stateful_widget(gate_list, chunks[0], &mut list_state);

            let input_para = Paragraph::new(format!(" Gate: {query}"))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Search ")
                        .border_style(Style::default().fg(Color::Green)),
                )
                .style(Style::default().fg(Color::Green));
            frame.render_widget(input_para, chunks[1]);
        }
        AddState::EnteringArgs {
            gate_type,
            args,
            current_arg,
            values,
        } => {
            let popup_area = centered_rect(55, 30, area);
            frame.render_widget(Clear, popup_area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(popup_area);

            let name = gate_type.to_string();
            let mut lines = vec![Line::from(format!(" Gate: {name}"))];

            for (i, arg) in args.iter().enumerate() {
                let value = values.get(i).map(String::as_str).unwrap_or_default();
                let prefix = if i == *current_arg {
                    "  > "
                } else if value.trim().is_empty() {
                    "    "
                } else {
                    "  ✓ "
                };

                let value_str = if value.is_empty() {
                    String::new()
                } else {
                    format!(" = {value}")
                };

                lines.push(Line::from(format!("{prefix}{}:{value_str}", arg.prompt)));
            }

            let remaining = values
                .iter()
                .filter(|value| value.trim().is_empty())
                .count();

            if remaining > 0 {
                lines.push(Line::from(""));
                lines.push(Line::from(format!(
                    "  ({remaining} empty field{})",
                    if remaining > 1 { "s" } else { "" }
                )));
            } else {
                lines.push(Line::from(""));
                lines.push(Line::from("  Press Enter to add gate"));
            }

            let para = Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Add Gate ")
                        .border_style(Style::default().fg(Color::Yellow)),
                )
                .style(Style::default().fg(Color::White).bg(Color::Black));
            frame.render_widget(para, chunks[0]);

            let active = &args[*current_arg.min(&args.len().saturating_sub(1))];
            let active_value = values
                .get(*current_arg)
                .map(String::as_str)
                .unwrap_or_default();
            let input_para = Paragraph::new(format!(" {}: {active_value}", active.prompt))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Value ")
                        .border_style(Style::default().fg(Color::Yellow)),
                )
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(input_para, chunks[1]);
        }
    }
}

fn render_toast(frame: &mut Frame, area: Rect, status_area: Rect, message: &str) {
    let max_width = area.width.saturating_sub(4).max(1);
    let content_width = message.chars().count() as u16 + 4;
    let width = content_width.min(max_width).max(20.min(max_width));
    let height = 3.min(area.height);

    if width == 0 || height == 0 {
        return;
    }

    let is_error = message.starts_with("Error:")
        || message.starts_with("Parse error:")
        || message.starts_with("Serialize error:")
        || message.starts_with("Cannot ")
        || message.starts_with("Unknown ")
        || message.starts_with("No ");

    let title = if is_error { " Error " } else { " Notice " };
    let style = if is_error {
        Style::default().fg(Color::White).bg(Color::Red)
    } else {
        Style::default().fg(Color::Black).bg(Color::LightGreen)
    };

    let toast_area = Rect {
        x: area.x + area.width.saturating_sub(width + 1),
        y: status_area.y.saturating_sub(height),
        width,
        height,
    };

    frame.render_widget(Clear, toast_area);

    let toast = Paragraph::new(message)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style)
        .wrap(Wrap { trim: true });
    frame.render_widget(toast, toast_area);
}

fn render_help(frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(50, 40, area);
    frame.render_widget(Clear, popup_area);

    let help_text = vec![
        Line::from(""),
        Line::from(" A  - Add a gate"),
        Line::from(" R  - Remove selected gate"),
        Line::from(" I  - Import circuit from file"),
        Line::from(" E  - Export circuit to file"),
        Line::from(""),
        Line::from(" ↑  - Navigate up"),
        Line::from(" ↓  - Navigate down"),
        Line::from(" ←  - Scroll circuit left"),
        Line::from(" →  - Scroll circuit right"),
        Line::from(""),
        Line::from(" Esc  - Go back / Quit"),
        Line::from(" H    - Close this panel"),
        Line::from(" q    - Quit"),
    ];

    let help_popup = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" QCTidy TUI Help ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left);
    frame.render_widget(help_popup, popup_area);
}

fn render_import_popup(frame: &mut Frame, area: Rect, buffer: &str) {
    let popup_area = centered_rect(60, 20, area);
    frame.render_widget(Clear, popup_area);

    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let display = if buffer.is_empty() {
        format!("{cwd}/")
    } else {
        format!("{cwd}/{buffer}")
    };

    let para = Paragraph::new(vec![
        Line::from(" Enter file path to import:"),
        Line::from(""),
        Line::from(format!(" {display}")),
        Line::from(""),
        Line::from(" Supported: .json, .xml, .cbor, .msgpack"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Import Circuit ")
            .border_style(Style::default().fg(Color::Green)),
    )
    .style(Style::default().fg(Color::White).bg(Color::Black));
    frame.render_widget(para, popup_area);
}

fn render_export_popup(
    frame: &mut Frame,
    area: Rect,
    format_index: usize,
    buffer: &str,
    step: &ExportStep,
) {
    let popup_area = centered_rect(55, 30, area);
    frame.render_widget(Clear, popup_area);

    let items: Vec<ListItem> = FORMATS
        .iter()
        .enumerate()
        .map(|(i, (name, ext, desc))| {
            let marker = if i == format_index { "▸ " } else { "  " };
            ListItem::new(format!("{marker}{name:<12} {ext:<8}  {desc}"))
        })
        .collect();

    let format_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Export Format ")
            .border_style(Style::default().fg(Color::Green)),
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if matches!(step, ExportStep::EnteringFilename) {
            vec![
                Constraint::Length(FORMATS.len() as u16 + 2),
                Constraint::Length(3),
            ]
        } else {
            vec![Constraint::Length(FORMATS.len() as u16 + 2)]
        })
        .split(popup_area);

    frame.render_widget(format_list, chunks[0]);

    if let ExportStep::EnteringFilename = step {
        let input_para = Paragraph::new(format!(" Filename: {buffer}"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" File ")
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(input_para, chunks[1]);
    }
}
