use std::fs::File;
use std::io::{stdin, stdout, Read, Stdout, Write};

use termion::cursor::*;
use termion::event;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use std::num::Wrapping;

use crate::VERSION;

pub struct EditorConfig {
    #[allow(dead_code)]
    pub cols: usize,
    pub rows: usize,
}

impl EditorConfig {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self { cols, rows }
    }
}

pub struct Content {
    rows: Vec<String>,
}

impl Content {
    pub fn new(rows: Vec<String>) -> Self {
        Content { rows }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content {
            rows: Vec::<String>::new(),
        }
    }
}

pub struct Editor {
    config: EditorConfig,
    out: RawTerminal<Stdout>,
    cursor_x: usize,
    cursor_y: usize,
    content: Option<Content>,
    num_rows: usize,
    row_offset: usize,
    col_offset: usize,
}

impl Editor {
    pub fn new() -> Self {
        // change rawmode
        // TODO: describe canonical mode and raw mode
        // dropped stdout, restore original state
        // into_raw_mode invoke libc::cfmakeraw()
        // cfmakeraw() set like `version7` driver's row mode
        //
        // specifically, set flags below
        // termios_p->c_iflag &= ~(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON)
        // termios_p->c_oflag &= ~OPOST;
        // termios_p->c_lflag &= ~(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
        // termios_p->c_cflag &= ~(CSIZE | PARENB);
        // termios_p->c_cflag |= CS8;
        //
        // into_raw_mode: https://github.com/redox-os/termion/blob/dce5e7500fd709987f9bf8f3911e4daa61d0ad14/src/raw.rs#L101-L114
        // raw_terminal_attr: https://github.com/redox-os/termion/blob/8054e082b01c3f45f89f0db96bc374f1e378deb1/src/sys/unix/attr.rs#L17-L19
        let mut out = stdout().into_raw_mode().unwrap();
        let (cols, rows) = Self::get_window_size(&mut out);
        let config = EditorConfig::new(cols.into(), rows.into());

        Self {
            config,
            out,
            cursor_x: 0,
            cursor_y: 0,
            content: None,
            num_rows: 0,
            row_offset: 0,
            col_offset: 0,
        }
    }

    // In ASCII character code, control characters that are not displayed as characters
    // are assigned to 0~31(0000_0000 ~ 0001_1111)
    // so ctrl char code can be get by bit and
    // example:
    // b'q' & 0b0001_1111
    #[allow(dead_code)]
    fn ctrl_key(key: u8) -> u8 {
        key & 0b001_1111
    }

    pub fn run(&mut self, path: Option<&String>) {
        match path {
            Some(path) => self.open(path),
            _ => {}
        };

        self.refresh_screen();
        for k in stdin().keys() {
            match k {
                Ok(k) => {
                    match k {
                        // if k == ctrl_key(b'q')
                        event::Key::Ctrl('q') => {
                            self.reset_screen_on_end();
                            break;
                        }
                        k @ (event::Key::Char('w')
                        | event::Key::Up
                        | event::Key::Char('a')
                        | event::Key::Left
                        | event::Key::Char('s')
                        | event::Key::Down
                        | event::Key::Char('d')
                        | event::Key::Right
                        | event::Key::PageUp
                        | event::Key::PageDown
                        | event::Key::Home
                        | event::Key::End
                        | event::Key::Delete) => self.update_cursor_state(&k),
                        _ => println!("{:?}\r", k),
                    }

                    self.refresh_screen();
                }
                Err(e) => {
                    self.reset_screen_on_end();
                    panic!("{}", e);
                }
            }
        }
    }

    fn open(&mut self, path: &String) {
        let mut f = File::open(path).unwrap();
        // read_line returns string when \r or \n appear
        // read only one row
        let mut content_string = String::with_capacity(4096);
        let mut rows = Vec::<String>::new();
        let mut num_rows = 0;

        let _ = f.read_to_string(&mut content_string);
        for l in content_string.lines() {
            num_rows += 1;
            rows.push(String::from(l));
        }

        self.content = Some(Content::new(rows));
        self.num_rows = num_rows;
    }

    fn update_cursor_state(&mut self, key: &event::Key) {
        match key {
            event::Key::Char('w') | event::Key::Up => {
                if 0 < self.cursor_y {
                    self.cursor_y -= 1;
                }
            }
            // left Left Arrow is \x1b[D
            event::Key::Char('a') | event::Key::Left => {
                if 0 < self.cursor_x {
                    self.cursor_x -= 1;
                }
            }
            // down Down Arrow is \x1b[B
            event::Key::Char('s') | event::Key::Down => {
                if self.cursor_y < self.num_rows {
                    self.cursor_y += 1;
                }
            }
            // right Right Arrow is \x1b[C
            event::Key::Char('d') | event::Key::Right => {
                self.cursor_x += 1;
            }
            // pageup is \x1b[5~
            event::Key::PageUp => {
                if self.cursor_y <= self.config.rows {
                    self.cursor_y = 0;
                } else {
                    self.cursor_y -= self.config.rows;
                }
            }
            // pagedown is \x1b[6~
            event::Key::PageDown => {
                if self.cursor_y < self.config.rows {
                    self.cursor_y = self.config.rows - 1;
                }
            }
            // home depends on OS.
            // colud be \x1b[1~, \x1b[7~, \x1b[H, \x1b[0H
            event::Key::Home => {
                self.cursor_x = 0;
            }
            // end depends on OS.
            // colud be \x1b[4~, \x1b[8~, \x1b[F, \x1b[0F
            event::Key::End => {
                self.cursor_x = self.config.cols - 1;
            }
            // del is \x1b[3~
            // do nothing as of now
            event::Key::Delete => {}
            _ => {
                unreachable!("key is allowed only wasd, allow, pageup, pagedown")
            }
        }
    }

    // return (col, row)
    fn get_window_size(out: &mut Stdout) -> (u16, u16) {
        match termion::terminal_size() {
            Ok(ts) => ts,
            Err(_) => {
                // get termsize manually
                // move cursor bottom right
                // \x1b[nC (n: natural number) move cursor to right direction amount of n
                // \x1b[nB (n: natural number) move cursor to bottom direction amount of n
                // print!("\x1b[999C\x1b[999B");
                print!("{}", termion::cursor::Goto(999, 999));
                out.flush().unwrap();

                // then, get cursor position
                // get_cursor_pos()
                out.cursor_pos().unwrap()
            }
        }
    }

    // return (col, row)
    #[allow(dead_code)]
    fn get_cursor_pos(out: &mut Stdout) -> (u16, u16) {
        // get terminal status by \x1b[6n (https://vt100.net/docs/vt100-ug/chapter3.html#DSR)
        // temrinal responses to stdin (http://vt100.net/docs/vt100-ug/chapter3.html#CPR)
        // like `\x1b[{row};{col}R`
        // after print \x1b[6n, parse response
        print!("\x1b[6n");
        print!("\r\n");
        out.flush().unwrap();

        let mut response = Vec::<u8>::new();

        // parse terminal response
        // get cols and rows from `\x1b[{cols};{rows}`
        for b in stdin().bytes() {
            match b.unwrap() {
                b'\x1b' | b'[' => {}
                b'R' => break,
                b => response.push(b),
            }
        }
        let row_col = String::from_utf8(response).unwrap();
        let row_col = row_col.splitn(2, ';').collect::<Vec<&str>>();
        let row_col: Vec<u16> = row_col
            .iter()
            .map(|num_str| num_str.parse::<u16>().unwrap())
            .collect();

        (row_col[1], row_col[0])
    }

    fn reset_screen_on_end(&mut self) {
        print!("\x1b[2j");
        print!("\x1b[H");
        self.out.flush().unwrap();
    }

    fn refresh_screen(&mut self) {
        self.editor_scroll();
        // \x1b is escape character
        // this is write escape sequence to terminal
        // example: \x1b[2j
        // \x1b = ESC
        // [    = ANSI control sequence indtoducer
        // 2j   = erase entire screen

        // set mode; change to ANSI/VT52 mode
        print!("\x1b[?25l");
        // moves cursor to home position (1, 1)
        print!("\x1b[H");

        self.draw_rows();

        // set cursor position current state of cursor
        // \x1b[{line};{column}
        // cursor_y range is less than numrows
        // therefore, it may exceed the rows of the window
        // to solve this problem, draw the value (cursor_y - row_offset)
        print!(
            "\x1b[{};{}H",
            (self.cursor_y - self.row_offset) + 1,
            (self.cursor_x - self.col_offset) + 1
        );

        // reset mode (change to screen mode)
        print!("\x1b[?25h");
        self.out.flush().unwrap();
    }

    fn draw_rows(&mut self) {
        // draw `~` terminal rows number
        let rows = self.config.rows;
        let cols = self.config.cols;
        (0..rows).for_each(|i| {
            let filerow = i + self.row_offset;
            if filerow < self.num_rows {
                if let Some(content) = &self.content {
                    let range = if content.rows[filerow].len() < self.col_offset {
                        // no content in display range
                        0..0
                    } else {
                        let end = self.col_offset
                            + (content.rows[filerow].len() - self.col_offset).min(cols);
                        self.col_offset..end
                    };
                    print!("{}", &content.rows[filerow][range]);
                }
            } else {
                if i == rows / 3 && self.num_rows == 0 {
                    let msg = format!("kiroro editor -- version {}", VERSION);
                    let msg_len = msg.len().min(cols as usize);
                    let padding_space_count = (cols as usize - msg_len) / 2;
                    print!(
                        "~{}{}",
                        " ".repeat(padding_space_count - 1),
                        &msg[..msg_len]
                    );
                } else {
                    print!("~");
                }
            }
            // \1b[K is erase in line
            // erase a line on current cursor
            print!("\x1b[K");
            if i < rows - 1 {
                print!("\r\n");
            }
        });
        self.out.flush().unwrap();
    }

    fn editor_scroll(&mut self) {
        // vartical scroll
        if self.cursor_y < self.row_offset {
            self.row_offset = self.cursor_y;
        } else if self.row_offset + self.config.rows <= self.cursor_y {
            self.row_offset += 1;
        }

        // horizontal scroll
        if self.cursor_x < self.col_offset {
            self.col_offset = self.cursor_x;
        } else if self.col_offset + self.config.cols <= self.cursor_x {
            self.col_offset += 1;
        }
    }
}
