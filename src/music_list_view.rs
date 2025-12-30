use std::fs::read_dir;

use gpui::{
    InteractiveElement, ParentElement, Render, StatefulInteractiveElement, Styled, div,
    prelude::FluentBuilder, rgb, uniform_list,
};
use log::info;

pub(crate) struct ListView {
    songs_list: Vec<String>,
}

impl ListView {
    pub(crate) fn new() -> Self {
        Self {
            songs_list: Vec::new(),
        }
    }

    pub(crate) fn load_songs(&mut self) {
        self.songs_list = read_dir(".")
            .expect("Unable to list files at the given path")
            .map(|res| res.unwrap().path().canonicalize().unwrap())
            .filter(|p| p.is_file() && p.extension().is_some() && p.extension().unwrap() == "mp3")
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<String>>();
        info!("entries {:?}", self.songs_list);
    }
}

impl Render for ListView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .bg(rgb(0xC8E5EE))
            .gap_3()
            .size_full()
            .flex()
            .flex_col()
            .when_else(
                !self.songs_list.is_empty(),
                |_div| {
                    let songs_list = self.songs_list.clone();

                    _div.items_start()
                        .flex()
                        .justify_between()
                        .size_full()
                        .child(
                            div()
                                .flex()
                                .bg(gpui::green())
                                .gap_3()
                                .w_full()
                                .px_4()
                                .justify_between()
                                .child("Song name")
                                .child("Song writer")
                                .child("Singer"),
                        )
                        .child(
                            uniform_list(
                                "songs-list",
                                songs_list.len(),
                                cx.processor(move |_this, range, _window, _cx| {
                                    let mut items = Vec::new();
                                    for idx in range {
                                        let val: &String = &songs_list[idx];
                                        let text_val =
                                            Box::leak(val.clone().into_boxed_str()) as &'static str;

                                        items.push(
                                            div()
                                                .id(text_val)
                                                .px_2()
                                                .cursor_pointer()
                                                .child(text_val)
                                                .on_click(move |_, _, _| {
                                                    println!("song clicked {:?}", text_val);
                                                }),
                                        );
                                    }
                                    items
                                }),
                            )
                            .flex_1()
                            .size_full(),
                        )
                },
                |div| div.px_2().child("No songs found"),
            )
    }
}
