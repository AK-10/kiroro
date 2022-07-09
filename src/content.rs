use crate::row::Row;
use std::{error, fmt};

pub struct Content {
    pub filename: Option<String>,
    pub rows: Vec<Row>,
}

#[derive(Debug)]
struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl error::Error for Error {}

impl Error {
    pub fn new<T>(msg: T) -> Self
    where
        T: Into<String>,
    {
        Self(msg.into())
    }
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

    pub fn row_from_index(&self, n: usize) -> Option<&Row> {
        self.rows.get(n)
    }

    // concat all rows with \n
    pub fn rows_to_string(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.row.clone())
            .collect::<Vec<String>>()
            .join(&"\n")
    }

    pub fn insert_char(&mut self, row_idx: usize, col_idx: usize, c: char) -> Result<(), Box<dyn error::Error>> {
        if let Some(row) = self.rows.get_mut(row_idx) {
            row.insert(col_idx, c)
        } else {
            let msg = format!("row idx: {} | row not found.", row_idx);
            Err(Box::new(Error::new(msg)))
        }
    }

    pub fn delete_char(&mut self, row: usize, col: usize) -> Result<(), Box<dyn error::Error>> {
        Ok(())
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