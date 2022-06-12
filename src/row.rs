use crate::TAB_STOP;

pub struct Row {
    pub row: String,
    pub render: String,
}

impl Row {
    pub fn new<T>(row: T) -> Self
    where
        T: Into<String> + Clone,
    {
        let row = row.clone().into();
        let render: String = row.clone().replace('\t', &" ".repeat(TAB_STOP.into()));

        Self { row, render }
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
        let rows = text.lines().map(|l| Row::new(l));

        Self {
            rows: rows.collect(),
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content {
            rows: Vec::<Row>::new(),
        }
    }
}
