use ncurses::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct DataStructure {
    todos: Vec<String>,
    dones: Vec<String>,
}

type Id = usize;
// #[derive(Default)]
enum UiType {
    Spacer,
    Heading,
    Done,
    Todo,
}

#[derive(Default)]
struct UI {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl UI {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }
    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested lists are not allowed.");
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id, uitype: UiType) {
        let id_curr = self.list_curr.expect(&format!(
            "Not allowed to create list element outside the list"
        ));
        self.label(
            label,
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            },
            uitype,
        );
    }
    fn label(&mut self, s: &str, pair: i16, uitype: UiType) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(s);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }
    fn end_list(&mut self) {
        self.list_curr = None;
    }
    fn end(&mut self) {}
}

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;
const HEADING_PAIR: i16 = 2;
const SPACER_PAIR: i16 = 3;
const DONE_PAIR: i16 = 4;

fn main() {
    let mut todo1 = DataStructure {
        todos: vec![
            "Write a TODO app.".to_string(),
            "Do your work.".to_string(),
            "Learn things.".to_string(),
        ],
        dones: vec![
            "Write a TODO app. done".to_string(),
            "Do your work.done".to_string(),
        ],
        // done: Vec::<String>::new(),
    };
    // let file =
    //     File::open("/home/sg/.config/todo-app/config.json").expect("ERROR: cannot read the file.");
    // let mut todo1: TODO = serde_json::from_reader(file).expect("Error while parsing");
    // // Convert the Point to a JSON string.
    // // println!("TODOS = {:?}", todo2);
    // let serialized = serde_json::to_string(&todo1).unwrap();
    // let deserialized: TODO = serde_json::from_str(&serialized).unwrap();

    let mut curr_todo: usize = 0;
    let mut done_todo: usize = 0;
    let mut quit = false;
    // let mut alignment: usize = 0;
    let mut ui = UI::default();

    initscr(); // initializing window

    noecho(); // no echo chars
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // cursor visibility

    start_color(); // colored pairs to show selected element
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK); // non selected element color
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE); // selected element color
    init_pair(HEADING_PAIR, COLOR_BLUE, COLOR_BLACK); // selected element color
    init_pair(SPACER_PAIR, COLOR_MAGENTA, COLOR_BLACK); // selected element color
    init_pair(DONE_PAIR, COLOR_GREEN, COLOR_BLACK); // selected element color

    // alignment += 2;
    // addstr(" [TODO]\n ---------------------");
    while !quit {
        // game loop
        erase();
        ui.begin(0, 0);
        {
            ui.label(" # [TODO]", HEADING_PAIR, UiType::Heading);
            ui.begin_list(curr_todo);
            for (index, todo) in todo1.todos.iter().enumerate() {
                ui.list_element(&format!(" - [ ] {}", todo), index, UiType::Todo);
            }
            ui.end_list();
            refresh();
            ui.label(
                "--------------------------------------",
                SPACER_PAIR,
                UiType::Spacer,
            );
            ui.label(" # [DONE]", HEADING_PAIR, UiType::Heading);
            ui.begin_list(done_todo);
            for (index, done) in todo1.dones.iter().enumerate() {
                ui.list_element(&format!(" - [x] {}", done), index + 1, UiType::Done);
            }
            ui.end_list();
        }
        ui.end();
        refresh();
        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
            'j' => {
                if curr_todo < todo1.todos.len() - 1 {
                    curr_todo += 1
                }
            }
            'k' => {
                if curr_todo > 0 {
                    curr_todo -= 1
                }
            }
            '\n' => {
                // todo1.dones.push(todo1.todos.remove(curr_todo));
                if todo1.todos.len() > curr_todo {
                    todo1.dones.push(todo1.todos.remove(curr_todo));
                }
                if curr_todo > 0 {
                    curr_todo -= 1;
                }
            }
            _ => {}
        }
    }
    endwin(); // destroy window
              // // Prints serialized = {"x":1,"y":2}
              // println!("serialized = {}", serialized);

    // // Convert the JSON string back to a Point.

    // // Prints deserialized = Point { x: 1, y: 2 }
    // println!("deserialized = {:?}", deserialized);
    // let outfile = File::create("/home/sg/.config/todo-app/config.json").expect("cannot create.");
    // serde_json::to_writer(outfile, &deserialized).expect("gg");
}
