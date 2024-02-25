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

use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
  action::Action,
  config::Config,
  tui::{Event, Frame},
};

pub mod fps;
pub mod home;

/// `Component` is a trait that represents a visual and interactive element of the user interface.
/// Implementors of this trait can be registered with the main application loop and will be able to receive events,
/// update state, and be rendered on the screen.
pub trait Component {
  /// Register an action handler that can send actions for processing if necessary.
  ///
  /// # Arguments
  ///
  /// * `tx` - An unbounded sender that can send actions.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  #[allow(unused_variables)]
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    Ok(())
  }
  /// Register a configuration handler that provides configuration settings if necessary.
  ///
  /// # Arguments
  ///
  /// * `config` - Configuration settings.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  #[allow(unused_variables)]
  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    Ok(())
  }
  /// Initialize the component with a specified area if necessary.
  ///
  /// # Arguments
  ///
  /// * `area` - Rectangular area to initialize the component within.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  fn init(&mut self, area: Rect) -> Result<()> {
    Ok(())
  }
  /// Handle incoming events and produce actions if necessary.
  ///
  /// # Arguments
  ///
  /// * `event` - An optional event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
    let r = match event {
      Some(Event::Key(key_event)) => self.handle_key_events(key_event)?,
      Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event)?,
      _ => None,
    };
    Ok(r)
  }
  /// Handle key events and produce actions if necessary.
  ///
  /// # Arguments
  ///
  /// * `key` - A key event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  #[allow(unused_variables)]
  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    Ok(None)
  }
  /// Handle mouse events and produce actions if necessary.
  ///
  /// # Arguments
  ///
  /// * `mouse` - A mouse event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  #[allow(unused_variables)]
  fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
    Ok(None)
  }
  /// Update the state of the component based on a received action. (REQUIRED)
  ///
  /// # Arguments
  ///
  /// * `action` - An action that may modify the state of the component.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  #[allow(unused_variables)]
  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    Ok(None)
  }
  /// Render the component on the screen. (REQUIRED)
  ///
  /// # Arguments
  ///
  /// * `f` - A frame used for rendering.
  /// * `area` - The area in which the component should be drawn.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()>;
}
