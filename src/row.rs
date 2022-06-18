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
        let row: String = row.clone().into();

        let mut render = String::new();
        let mut index = 0;
        for c in row.chars() {
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
        }

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

impl Content {
    pub fn row_from_index(&self, n: usize) -> Option<&Row> {
        self.rows.get(n)
    }
}
