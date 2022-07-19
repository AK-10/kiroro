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
    pub row: String,
    pub render: String,
}

impl Row {
    pub fn new<T>(row: T) -> Self
    where
        T: Into<String> + Clone,
    {
        let row = row.into();
        let render = String::with_capacity(row.len());

        let mut row = Self { row, render };
        row.update_render();

        row
    }

    pub fn insert(&mut self, n: usize, c: char) -> Result<(), Box<dyn error::Error>> {
        if n <= self.row.len() {
            // O(n) operation
            self.row.insert(n, c);
            self.update_render();
            Ok(())
        } else {
            let msg = format!("failed insert index: {}, char: {}", n, c);
            Err(Box::new(Error::new_write(msg)))
        }
    }

    pub fn delete(&mut self, n: usize) -> Result<(), Box<dyn error::Error>> {
        if n <= self.row.len() {
            self.row.remove(n);
            self.update_render();
            Ok(())
        } else {
            let c = self.row.chars().nth(n).unwrap_or('\0');
            let msg = format!("failed insert index: {}, char: {}", n, c);
            Err(Box::new(Error::new_write(msg)))
        }
    }

    pub fn split(&self, pivot: usize) -> Result<(Row, Row), Box<dyn error::Error>> {
        if pivot <= self.row.len() {
            let (first, second) = self.row.split_at(pivot);
            Ok((Row::new(first), Row::new(second)))
        } else {
            let msg = format!("failed split index out of range. index: {}", &pivot);
            Err(Box::new(Error::new_write(msg)))
        }
    }

    pub fn update_render(&mut self) {
        let mut render = String::new();
        let mut index = 0;
        (&self.row).chars().for_each(|c| {
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
}
