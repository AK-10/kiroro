use std::ops::Range;

pub struct Row {
    row: String,
    #[allow(dead_code)]
    render_size: usize,
    #[allow(dead_code)]
    render: String,
}

impl Row {
    pub fn new<T>(row: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            row: row.into(),
            render_size: 0,
            render: "".into(),
        }
    }

    pub fn len(&self) -> usize {
        self.row.len()
    }

    pub fn sub_row(&self, range: Range<usize>) -> &str {
        &self.row[range]
    }
}

pub struct Content {
    pub rows: Vec<Row>,
}

impl Content {
    pub fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }

    pub fn from_text(text: &String) -> Self {
        let rows = text.lines().map(|l| {
            Row::new(l)
        });

        Self { rows: rows.collect() }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content {
            rows: Vec::<Row>::new(),
        }
    }
}
