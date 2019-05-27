#![deny(warnings)]
use sauron::{
    html::{attributes::*, events::*, *},
    Cmd, Component, Node, Program,
};
use wasm_bindgen::prelude::*;

use std::sync::atomic::{AtomicUsize, Ordering};

static VIEW_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub enum Msg {
    Click(usize),
}

pub struct App {
    number: usize,
}

impl App {
    pub fn new() -> Self {
        App { number: 0 }
    }
}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        VIEW_COUNT.fetch_add(1, Ordering::SeqCst);
        let vc = VIEW_COUNT.load(Ordering::SeqCst);
        div(
            [],
            [
                div(
                    [],
                    [input(
                        [
                            r#type("button"),
                            value(format!("VIEW_COUNT: {}", vc)),
                            onclick(move |_| {
                                sauron::log(format!("Button is clicked (VIEW_COUNT = {})", vc));
                                Msg::Click(vc)
                            }),
                        ],
                        [],
                    )],
                ),
                div(
                    [],
                    [text(format!(
                        "VIEW_COUNT: {}",
                        VIEW_COUNT.load(Ordering::SeqCst)
                    ))],
                ),
                div(
                    [],
                    [text(format!(
                        "number: {}",
                        self.number
                    ))],
                ),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        sauron::log!("App is updating from msg: {:?}", msg);
        match msg {
            Msg::Click(number) => self.number = number,
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(App::new());
}
