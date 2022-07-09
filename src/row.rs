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

    fn update_render(&mut self) {
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

pub struct Content {
    pub filename: Option<String>,
    pub rows: Vec<Row>,
}

impl Content {
    pub fn new(filename: String, rows: Vec<Row>) -> Self {
        Self {
            filename: Some(filename),
            rows,
        }
    }

    pub fn from_text(filename: String, text: &String) -> Self {
        let rows = text.lines().map(|l| Row::new(l));

        Self {
            filename: Some(filename),
            rows: rows.collect(),
        }
    }

    // concat all rows with \n
    pub fn rows_to_string(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.row.clone())
            .collect::<Vec<String>>()
            .join(&"\n")
    }
}

impl Default for Content {
    fn default() -> Self {
        Content {
            filename: None,
            rows: Vec::<Row>::new(),
        }
    }
}

impl Content {
    pub fn row_from_index(&self, n: usize) -> Option<&Row> {
        self.rows.get(n)
    }
}
