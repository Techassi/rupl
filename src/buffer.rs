use std::fmt::{Display, Write};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BufferError {
    #[error("Invalid start index, must be <= buf len")]
    InvalidStartIndex,

    #[error("Deleting at {at} with count {count} overflows buf len")]
    DeleteCountOverflow { at: usize, count: usize },
}

#[derive(Debug, Default)]
pub struct Buffer {
    buf: Vec<char>,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.buf {
            f.write_char(*c)?;
        }
        Ok(())
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn insert(&mut self, at: usize, chars: &[char]) -> Result<(), BufferError> {
        if at > self.len() {
            return Err(BufferError::InvalidStartIndex);
        }

        for (i, c) in chars.iter().enumerate() {
            self.buf.insert(at + i, *c)
        }

        Ok(())
    }

    pub fn remove(&mut self, at: usize, count: usize) -> Result<Vec<char>, BufferError> {
        if at > self.len() {
            return Err(BufferError::InvalidStartIndex);
        }

        if at + count > self.len() {}

        self.remove_from_to(at, at + count)
    }

    pub fn remove_from_to(&mut self, at: usize, to: usize) -> Result<Vec<char>, BufferError> {
        Ok(self.buf.drain(at..to).collect())
    }

    pub fn clear(&mut self) {
        self.buf.clear()
    }
}

pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct CursorBuffer {
    cur_pos: usize,
    buf: Buffer,
}

impl CursorBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn move_left(&mut self) -> usize {
        if self.cur_pos > 0 {
            self.cur_pos -= 1;
        }

        self.cur_pos
    }

    pub fn move_right(&mut self) -> usize {
        if self.cur_pos <= self.buf.len() {
            self.cur_pos += 1;
        }

        self.cur_pos
    }

    pub fn insert(&mut self, chars: &[char]) {
        self.buf.insert(self.cur_pos, chars);
        self.cur_pos += chars.len();
    }

    pub fn remove_one(&mut self, dir: Direction) {
        match dir {
            Direction::Left => self.buf.remove(self.cur_pos - 1, 1),
            Direction::Right => self.buf.remove(self.cur_pos, 1),
        };
    }

    pub fn remove_many(&mut self, count: usize, dir: Direction) {
        match dir {
            Direction::Left => self.buf.remove(self.cur_pos - count, count),
            Direction::Right => self.buf.remove(self.cur_pos, count),
        };
    }
}
