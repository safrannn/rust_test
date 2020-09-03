mod css_classes;
use css_classes::C;

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ulid::Ulid;

const STORAGE_KEY: &str = "todos-seed";

#[derive(Default)]
struct Model {
    todos: BTreeMap<Ulid, Todo>,
    new_todo_content: String,
}

#[derive(Deserialize, Serialize)]
struct Todo {
    id: Ulid,
    content: String,
}

impl Todo {
    fn new(id: Ulid, text: String) -> Self {
        Todo {
            id: id,
            content: text,
        }
    }
}

#[derive(Clone)]
enum Msg {
    ChangeContent(String),
    SaveTodo,
    DeleteTodo(Ulid),
    ClearAll,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeContent(new_text) => model.new_todo_content = new_text,
        Msg::SaveTodo => {
            let content = model.new_todo_content.trim().to_owned();
            if content.is_empty() {
                return;
            };
            let id = Ulid::new();
            model.todos.insert(id, Todo::new(id, content));
            model.new_todo_content = String::new();
        }
        Msg::DeleteTodo(id) => {
            model.todos.remove(&id);
        }
        Msg::ClearAll => {
            model.todos.clear();
        }
    }
    // save list to local
    LocalStorage::insert(STORAGE_KEY, &model.todos).expect("save todos to LocalStorage");
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C![C.py_4, C.px_4],
        view_header(&model.new_todo_content),
        view_main(&model.todos),
    ]
}

fn view_header(new_todo_content: &str) -> Node<Msg> {
    header![
        img![
            C![C.py_2],
            attrs! {At::Src => "https://lvt.co/wp-content/uploads/2019/07/pasted-svg-303101x59.svg"}
        ],
        div![
            C![C.py_2],
            label![C!["header", C.block, C.text_gray_7, C.font_bold,],],
            input![
                C![
                    "new-todo",
                    C.w_3of12,
                    C.shadow,
                    C.appearance_none,
                    C.border,
                    C.rounded,
                    C.text_gray_7,
                    C.leading_tight,
                    C.py_2,
                    C.px_4,
                    C.focus__outline_none,
                    C.focus__shadow_outline,
                ],
                attrs! {
                    At::Placeholder => "What needs to be done?",
                    At::AutoFocus => AtValue::None,
                    At::Value => new_todo_content,
                },
                input_ev(Ev::Input, Msg::ChangeContent),
            ],
            button![
                C![
                    "save-button",
                    C.w_1of12,
                    C.bg_blue_5,
                    C.hover__bg_blue_7,
                    C.text_gray_1,
                    C.font_bold,
                    C.py_2,
                    C.px_4,
                    C.m_1,
                    C.rounded_full,
                ],
                "Save",
                ev(Ev::Click, |_| Msg::SaveTodo),
            ],
        ],
    ]
}

fn view_main(todos: &BTreeMap<Ulid, Todo>) -> Node<Msg> {
    let todo_count = todos.len();
    div![
        C!["main"],
        div![
            C![C.py_2],
            div![
                C!["delete_all"],
                span![
                    C!["todo-count"],
                    strong![todo_count],
                    format!(" item{} left  ", if todo_count == 0 | 1 { "" } else { "s" }),
                ],
                button![
                    C![
                        "delete-all"
                        C.w_3of12,
                        C.bg_blue_5,
                        C.hover__bg_blue_7,
                        C.text_gray_1,
                        C.font_bold,
                        C.py_2,
                        C.px_4,
                        C.rounded_full,
                    ],
                    "Clear All",
                    ev(Ev::Click, move |_| Msg::ClearAll,)
                ]
            ],
        ],
        ul![
            C!["todo-list", C.list_none, C.py_2, C.px_4,],
            todos.values().map(|todo| {
                let id = todo.id;
                li![
                    C![C.m_3],
                    button![
                        C![
                            "delete-todo",
                            C.bg_gray_1,
                            C.hover__bg_blue_7,
                            C.text_gray_3,
                            C.font_bold,
                            C.py_2,
                            C.px_4,
                            C.rounded_full,
                        ],
                        "x",
                        ev(Ev::Click, move |_| Msg::DeleteTodo(id)),
                    ],
                    span![C!["todo-content", C.w_3of12, C.m_5], &todo.content],
                ]
            })
        ]
    ]
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        todos: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
        new_todo_content: String::new(),
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
