use crate::row::Row;
use std::{error, fmt, mem};

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
            .map(|row| row.raw.clone())
            .collect::<Vec<String>>()
            .join(&"\n")
    }

    pub fn insert_char(
        &mut self,
        row_idx: usize,
        col_idx: usize,
        c: char,
    ) -> Result<(), Box<dyn error::Error>> {
        if let Some(row) = self.rows.get_mut(row_idx) {
            row.insert(col_idx, c)?
        } else {
            let row = Row::new(String::from(c));
            self.rows.push(row);
        }

        Ok(())
    }

    pub fn insert_new_line(
        &mut self,
        row_idx: usize,
        col_idx: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        if let Some(row) = self.rows.get_mut(row_idx) {
            let (first, second) = row.split(col_idx)?;
            let _ = mem::replace(row, first);
            self.rows.insert(row_idx + 1, second);
        } else {
            self.rows.push(Row::new(""));
        }

        Ok(())
    }

    pub fn delete_char(
        &mut self,
        row_idx: usize,
        col_idx: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        if let Some(row) = self.rows.get_mut(row_idx) {
            row.delete(col_idx)
        } else {
            let msg = format!("row idx: {} | row not found.", row_idx);
            Err(Box::new(Error::new(msg)))
        }
    }

    pub fn concatenate_previous_row(
        &mut self,
        row_idx: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        if row_idx == 0 {
            // case of first row, there is no previous string.
            // do nothing
            Ok(())
        } else if 0 < row_idx && row_idx < self.rows.len() {
            let row_string = self.rows.remove(row_idx);
            if let Some(prev_row) = self.rows.get_mut(row_idx - 1) {
                prev_row.raw.push_str(&*row_string.raw);
                prev_row.update_render();
            }
            Ok(())
        } else {
            let msg = format!("row: {} | row index is out of range", row_idx);
            Err(Box::new(Error::new(msg)))
        }
    }

    pub fn is_phantom(&self) -> bool {
        self.filename.is_none()
    }

    // return (row idx, col idx)
    pub fn find(&self, query: &String) -> Option<(usize, usize)> {
        for (row_idx, row) in self.rows.iter().enumerate() {
            if let Some(col_idx) = row.render.find(query) {
                return Some((row_idx, row.convert_index_render_to_raw(col_idx)));
            }
        }

        None
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
