// Ad Infinitum - a 4x strategy set in a procedurally generated galaxy
// Copyright (C) 2024 Egor Kosachev
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ops::Deref;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Alignment, Line, Span};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, List, ListDirection, ListState, Paragraph, Wrap};
use tokio::sync::mpsc::UnboundedSender;
use crate::action::Action;
use crate::components::Component;
use crate::game::body::Body;
use crate::game::system::System;
use crate::tui::{Event, Frame};

pub struct SystemTree {
    system_displayed: System,
    selected_row: usize,
    list_state: ListState,
}

impl Default for SystemTree {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTree {
    pub fn new() -> Self {
        Self {
            selected_row: 0,
            system_displayed: System::generate(),
            list_state: ListState::default(),
        }
    }

    fn move_selection_up(&mut self) {
        if self.selected_row != 0 {
            self.selected_row -= 1;
        }
    }

    fn move_selection_down(&mut self) {
        self.selected_row += 1;
    }

    fn update_tree(&mut self, new_system: System) {
        self.system_displayed = new_system;
        self.selected_row = 0;
    }
}

impl Component for SystemTree {
    fn handle_key_events(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Up => { Ok(Some(Action::MoveSystemTreeSelectionUp)) }
            KeyCode::Down => { Ok(Some(Action::MoveSystemTreeSelectionDown)) }
            _ => { Ok(None) }
        }
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::MoveSystemTreeSelectionUp => {
                self.move_selection_up();
            },
            Action::MoveSystemTreeSelectionDown => {
                self.move_selection_down();
            },
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Min(0),
            ])
            .split(area);


        let mut bodies = vec![self.system_displayed.star.clone()];
        bodies.append(&mut self.system_displayed.bodies.clone());
        let bodies_names: Vec<Text> = bodies.iter()
            .map(|f| {
                let col: Color = f.clone().kind.into();
                Text::from(f.name.clone()).fg(col)
            })
            .collect();

        let mut selection_list = vec![self.system_displayed.star.clone()];
        selection_list.append(&mut self.system_displayed.bodies.clone());
        let selected =  selection_list.get(self.selected_row).unwrap();

        let list = List::new(bodies_names)
            .block(Block::default().title(format!("System tree - {}", self.system_displayed.star.name)).borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);


        
        let text: Vec<Line> = selected.make_info()
            .iter()
            .map(|f| { Line::from(f.clone()) })
            .collect();
        
        let par = Paragraph::new(text)
            .block(Block::new().title(format!("Information about - {}", selected.name)).borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        self.list_state.select(Some(self.selected_row));
        f.render_stateful_widget(list, rects[0], &mut self.list_state);
        f.render_widget(par, rects[1]);
        Ok(())
    }
}