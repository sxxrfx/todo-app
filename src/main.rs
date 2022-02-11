use ncurses::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct TODO {
    todo: Vec<String>,
    done: Vec<String>,
}

type Id = usize;

#[derive(Default)]
struct UI {
    list_curr: Option<Id>,
}

impl UI {
    fn begin(&mut self) {
        todo!()
    }
    fn begin_list(&mut self, id: Id) {
        todo!()
    }

    fn list_element(&mut self, label: &str, id: Id) {
        todo!()
        // let pair = {
        //     if curr_todo == index {
        //         highlight_pair
        //     } else {
        //         regular_pair
        //     }
        // };
        // attron(color_pair(pair));
        // mv((index) as i32, 0);
        // addstr("  [ ] - ");
        // addstr(&*todo);
        // attroff(color_pair(pair));
    }
    fn label(&mut self, s: &str) {
        todo!()
    }
    fn end_list(&mut self) {
        todo!()
    }
    fn end(&mut self) {
        todo!()
    }
}

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    let mut todo1 = TODO {
        todo: vec![
            "Write a TODO app.".to_string(),
            "Do your work.".to_string(),
            "Learn things.".to_string(),
        ],
        done: vec![
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

    // alignment += 2;
    // addstr(" [TODO]\n ---------------------");
    while !quit {
        // game loop
        ui.begin();
        {
            ui.begin_list(curr_todo);
            for (index, todo) in todo1.todo.iter().enumerate() {
                ui.list_element(todo, index);
            }
            ui.end_list();
            ui.label("--------------------------------------");
            ui.begin_list(done_todo);
            for (index, done) in todo1.done.iter().enumerate() {
                ui.list_element(done, index);
            }
            ui.end_list();
        }
        ui.end();
        refresh();
        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
            'j' => {
                if curr_todo < todo1.todo.len() - 1 {
                    curr_todo += 1
                }
            }
            'k' => {
                if curr_todo > 0 {
                    curr_todo -= 1
                }
            }
            ' ' => {
                todo1.done.push(todo1.todo.remove(curr_todo));
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
