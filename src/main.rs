use std::env;

use kiroro::editor::Editor;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = env::args().nth(1);
    // test_string(path.unwrap());

    let mut editor = Editor::new();
    editor.run(path);
}

#[allow(dead_code)]
fn test_string(path: String) {
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

    println!("{:?}", num_rows);
    println!("{:?}", rows);
}
