use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    symbols,
    text::{Line, Text},
    widgets::{Block, Borders, List, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::{app::CurrentScreen, controls::StatefulList};

pub fn ui(frame: &mut Frame, list_with_state: &mut StatefulList, app: &App) {
    let layout: Rc<[Rect]> = create_main_layout(frame.size());

    render_title(frame, layout[0]);

    render_list(frame, layout[1], list_with_state);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(layout[2]);

    render_footer(frame, footer_chunks, app);

    // When you enter a Task section, this popup will appear
    if let CurrentScreen::Task = app.current_screen {
        // frame.render_widget(Clear, frame.size()); // this clears the entire screen and anything already drawn
        let popup_task_block = Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::DOUBLE)
            .style(Style::default().fg(ratatui::style::Color::DarkGray));

        let task_string: String;
        if list_with_state.state.selected() == None {
            task_string = "No task Selected".to_string();
        } else {
            // we are using tasks : Vec<String>
            task_string = list_with_state.tasks[list_with_state.state.selected().unwrap()].clone();
            // create variable of selected task.
        }

        let styled_task = Text::styled(
            task_string,
            Style::default().fg(ratatui::style::Color::Green).bold(),
        );
        let display_task = Paragraph::new(styled_task)
            .block(popup_task_block)
            .wrap(Wrap { trim: false });
        let area = centered_rect(20, 20, frame.size());

        frame.render_widget(display_task, area);
    }

    if let CurrentScreen::Exiting = app.current_screen {
        // frame.render_widget(Clear, frame.size()); // this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::DOUBLE)
            .style(Style::default().fg(ratatui::style::Color::DarkGray));

        let exit_text = Text::styled(
            "Do you want to exit Task Manager?",
            Style::default().fg(ratatui::style::Color::Red),
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(exit_paragraph, area);
    }
}

/// This function creates the main layout of three blocks vertically.
fn create_main_layout(size: Rect) -> Rc<[Rect]> {
    Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ],
    )
    .split(size)
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED);
    let title_style = Style::default().fg(ratatui::style::Color::Green).bold();
    let title = Paragraph::new(Text::styled("Task Manager", title_style))
        .block(title_block)
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(title, area);
}

fn render_list(frame: &mut Frame, area: Rect, list_with_state: &mut StatefulList) {
    let list_block = Block::default()
        .title("List")
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .white();
    let list_style = Style::default().fg(ratatui::style::Color::Cyan);
    let list = List::new(list_with_state.tasks.clone())
        .block(list_block)
        .style(list_style)
        .highlight_style(
            Style::default()
                .fg(ratatui::style::Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("* ");

    frame.render_stateful_widget(list, area, &mut list_with_state.state);
}

fn render_footer(frame: &mut Frame, area: Rc<[Rect]>, app: &App) {
    let navigation = match app.current_screen {
        CurrentScreen::Main => Line::styled(
            " Main Menu",
            Style::default().fg(ratatui::style::Color::Gray),
        ),
        CurrentScreen::Editing => Line::styled(
            " Editing",
            Style::default().fg(ratatui::style::Color::Green),
        ),
        CurrentScreen::Task => Line::styled(
            " Task View",
            Style::default().fg(ratatui::style::Color::Yellow),
        ),
        CurrentScreen::Exiting => {
            Line::styled(" Exiting", Style::default().fg(ratatui::style::Color::Red))
        }
    };
    let navigation = Paragraph::new(navigation).block(Block::default().borders(Borders::ALL));
    frame.render_widget(navigation, area[0]);

    let control_panel = match app.current_screen {
        CurrentScreen::Main => Line::styled(
            " [⬆] / [⬇] to move, [➡] to select [⬅] to unselect, [q] to quit",
            Style::default().fg(ratatui::style::Color::Green),
        ),
        CurrentScreen::Editing => Line::styled(
            " [⬆] / [⬇] to move, [➡] to select [⬅] to unselect",
            Style::default().fg(ratatui::style::Color::Green),
        ),
        CurrentScreen::Task => Line::styled(
            " [⬆] / [⬇] to move, [➡] to select [⬅] to unselect",
            Style::default().fg(ratatui::style::Color::Green),
        ),
        CurrentScreen::Exiting => Line::styled(
            " [y] for yes, [n] for no",
            Style::default().fg(ratatui::style::Color::Green),
        ),
    };
    let control_panel = Paragraph::new(control_panel).block(Block::default().borders(Borders::ALL));
    frame.render_widget(control_panel, area[1]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece itno three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod ui_test {
    use super::*;

    #[test]
    fn test_centered_rect() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 200,
            height: 150,
        };
        let expected_rect = Rect {
            x: 74,
            y: 18,
            width: 50,
            height: 113,
        };
        let actual_rect = centered_rect(25, 75, rect);
        assert_eq!(actual_rect, expected_rect);
    }

    #[test]
    fn test_create_main_layout() {
        let screen_size = Rect {
            x: 0,
            y: 0,
            width: 80,
            height: 25,
        };

        let layout: Rc<[Rect]> = create_main_layout(screen_size);

        let expected_sizes: Rc<[Rect]> = [
            // Title: 3 units of height (assuming full width)
            Rect { x: 0, y: 0, width: 80, height: 3 },
            // List: Min height of 2 (assuming full width)
            Rect { x: 0, y: 3, width: 80, height: 19 },
            // Footer: 3 units of height (assuming full width)
            Rect {
                x: 0,
                y: 22,
                width: 80,
                height: 3,
            },
        ].into();

        for (i, rect) in layout.iter().enumerate() {
            assert_eq!(*rect, expected_sizes[i], "Element {} size mismatch", i);
        }
    }
}
