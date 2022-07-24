use crate::TAB_STOP;
use std::{error, fmt};

#[derive(Debug)]
enum Error {
    Read(String),
    Write(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "row: {}", self)
    }
}

impl error::Error for Error {}

impl Error {
    #[allow(dead_code)]
    fn new_read(msg: String) -> Self {
        Self::Read(msg)
    }
    fn new_write(msg: String) -> Self {
        Self::Write(msg)
    }
}

pub struct Row {
    pub raw: String,
    pub render: String,
}

impl Row {
    pub fn new<T>(raw: T) -> Self
    where
        T: Into<String> + Clone,
    {
        let raw = raw.into();
        let render = String::with_capacity(raw.len());

        let mut row = Self { raw, render };
        row.update_render();

        row
    }

    pub fn insert(&mut self, n: usize, c: char) -> Result<(), Box<dyn error::Error>> {
        if n <= self.raw.len() {
            // O(n) operation
            self.raw.insert(n, c);
            self.update_render();
            Ok(())
        } else {
            let msg = format!("failed insert index: {}, char: {}", n, c);
            Err(Box::new(Error::new_write(msg)))
        }
    }

    pub fn delete(&mut self, n: usize) -> Result<(), Box<dyn error::Error>> {
        if n <= self.raw.len() {
            self.raw.remove(n);
            self.update_render();
            Ok(())
        } else {
            let c = self.raw.chars().nth(n).unwrap_or('\0');
            let msg = format!("failed insert index: {}, char: {}", n, c);
            Err(Box::new(Error::new_write(msg)))
        }
    }

    pub fn split(&self, pivot: usize) -> Result<(Row, Row), Box<dyn error::Error>> {
        if pivot <= self.raw.len() {
            let (first, second) = self.raw.split_at(pivot);
            Ok((Row::new(first), Row::new(second)))
        } else {
            let msg = format!("failed split index out of range. index: {}", &pivot);
            Err(Box::new(Error::new_write(msg)))
        }
    }

    pub fn update_render(&mut self) {
        let mut render = String::new();
        let mut index = 0;
        (&self.raw).chars().for_each(|c| {
            if c == '\t' {
                render.push(' ');
                index += 1;
                while index % TAB_STOP != 0 {
                    render.push(' ');
                    index += 1;
                }
            } else {
                render.push(c);
                index += 1;
            }
        });

        self.render = render;
    }

    pub fn convert_index_raw_to_render(&self, raw_index: usize) -> usize {
        let mut render_index = 0;
        for c in self.raw.chars().take(raw_index) {
            if c == '\t' {
                render_index += TAB_STOP - (render_index % TAB_STOP);
            } else {
                render_index += 1;
            }
        }

        render_index.into()
    }

    pub fn convert_index_render_to_raw(&self, render_index: usize) -> usize {
        let mut index = 0usize;
        for (i, c) in self.raw.chars().enumerate() {
            if c == '\t' {
                index += TAB_STOP - (index % TAB_STOP);
            } else {
                index += 1;
            }

            if index > render_index {
                return i.into();
            }
        }

        self.raw.len()
    }
}
