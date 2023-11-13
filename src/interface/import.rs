//! # Import

use crate::interface::{Render, TextArea};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::Rect, Frame};

/// TODO: Add Tags list and file tree.
#[derive(Debug, Clone)]
pub struct Import {
    current: usize,
    start: TextArea,
    end: TextArea,
    title: TextArea,
    selector: TextArea,
}

impl Render for Import {
    fn render(&mut self, f: &mut Frame, area: Rect) {}

    /// TODO: Handle inner swap to lower block
    fn input(&mut self, key: KeyEvent) {
        let _ = match self.current {
            0 => self.start.input(key),
            1 => self.end.input(key),
            2 => self.title.input(key),
            3 => self.selector.input(key),
            _ => false,
        };
    }
}

impl Default for Import {
    fn default() -> Self {
        Self {
            current: Default::default(),
            start: Default::default(),
            end: Default::default(),
            title: Default::default(),
            selector: Default::default(),
        }
    }
}
