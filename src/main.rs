use std::env;

use kiroro::editor::Editor;

fn main() {
    let path = env::args().nth(1);

    let mut editor = Editor::new();
    editor.run(path.as_ref());
}
