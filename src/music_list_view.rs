use std::{fs::read_dir, str::FromStr, sync::Arc, sync::mpsc};

use crate::PlayerCommand;
use crate::audio_manager::AudioManager;
use gpui::{
    InteractiveElement, ParentElement, Render, StatefulInteractiveElement, Styled, div,
    prelude::FluentBuilder, rgb, uniform_list,
};
use id3::{Error, ErrorKind, Tag, TagLike};
use log::info;

type SharedAudioManager = Arc<AudioManager>;

#[derive(Clone, Debug)]
struct SongInfo {
    name: String,
    artist: String,
    album: String,
}

pub(crate) struct ListView {
    songs_list: Vec<SongInfo>,
    pub(crate) audio_manager: SharedAudioManager,
    msg_sender: mpsc::Sender<PlayerCommand>,
}

impl ListView {
    pub(crate) fn new(
        audio_manager: SharedAudioManager,
        msg_sender: mpsc::Sender<PlayerCommand>,
    ) -> Self {
        Self {
            songs_list: Vec::new(),
            audio_manager,
            msg_sender,
        }
    }

    pub(crate) fn load_songs(&mut self) {
        let songs = read_dir(".")
            .expect("Unable to list files at the given path")
            .map(|res| res.unwrap().path())
            .filter(|p| p.is_file() && p.extension().is_some() && p.extension().unwrap() == "mp3")
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<String>>();

        for song_path in songs.iter() {
            let tag = match Tag::read_from_path(&song_path) {
                Ok(tag) => tag,
                Err(Error {
                    kind: ErrorKind::NoTag,
                    ..
                }) => Tag::new(),
                Err(err) => Tag::new(),
            };
            let title = tag.title().unwrap_or(song_path);
            let artist = tag.artist().unwrap_or("No artist info!");
            let album = tag.album().unwrap_or("No album info!");

            let song_info = SongInfo {
                name: String::from(title),
                artist: String::from(artist),
                album: String::from(album),
            };
            self.songs_list.push(song_info);
        }

        info!("entries {:?}", self.songs_list);
    }
}

impl Render for ListView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let songs_list = self.songs_list.clone();
        let audio_manager = self.audio_manager.clone();
        let msg_sender = self.msg_sender.clone();

        div()
            .bg(rgb(0xC8E5EE))
            .gap_3()
            .size_full()
            .flex()
            .flex_col()
            .when_else(
                !songs_list.is_empty(),
                move |_div| {
                    let audio_manager = audio_manager.clone(); // clone BEFORE move

                    _div.items_start()
                        .flex()
                        .justify_between()
                        .flex_col()
                        .size_full()
                        .child(
                            div()
                                .flex()
                                .w_full()
                                .px_4()
                                .border_1()
                                .bg(rgb(0x1C4A5A))
                                .text_color(rgb(0xf1f1f1))
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .overflow_hidden()
                                        .text_ellipsis()
                                        .child("Title"),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .text_center()
                                        .overflow_hidden()
                                        .text_ellipsis()
                                        .child("Artist"),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .text_center()
                                        .overflow_hidden()
                                        .text_ellipsis()
                                        .child("Album"),
                                ),
                        )
                        .child(
                            uniform_list("songs-list", songs_list.len(), {
                                let songs_list = songs_list.clone();
                                cx.processor(move |_this, range, _window, _cx| {
                                    let audio_manager = audio_manager.clone(); // clone AGAIN
                                    let mut items = Vec::new();

                                    for idx in range {
                                        let song: &SongInfo = &songs_list[idx];
                                        let text_val: &'static str =
                                            Box::leak(song.name.clone().into_boxed_str());
                                        let artist = song.artist.clone();
                                        let album = song.album.clone();

                                        let audio_manager = audio_manager.clone();
                                        let msg_sender = msg_sender.clone();
                                        items.push(
                                            div()
                                                .id(text_val)
                                                .px_2()
                                                .flex()
                                                .w_full()
                                                .border_b_1()
                                                .border_color(gpui::black())
                                                .cursor_pointer()
                                                .on_click(move |_, _, _| {
                                                    println!("song clicked {:?}", text_val);
                                                    audio_manager.load(
                                                        String::from_str(text_val)
                                                            .expect("Unable to load the song"),
                                                    );
                                                    audio_manager.play();
                                                    let res = msg_sender.send(PlayerCommand::Play);
                                                    info!(
                                                        "Playing a new song {:?}\tcommand {:?}!!!",
                                                        text_val, res
                                                    );
                                                })
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .min_w_0()
                                                        .overflow_hidden()
                                                        .text_ellipsis()
                                                        .child(text_val),
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .min_w_0()
                                                        .text_center()
                                                        .overflow_hidden()
                                                        .text_ellipsis()
                                                        .child(artist),
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .min_w_0()
                                                        .text_center()
                                                        .overflow_hidden()
                                                        .text_ellipsis()
                                                        .child(album),
                                                ),
                                        );
                                    }
                                    items
                                })
                            })
                            .flex_1()
                            .size_full(),
                        )
                },
                |div| div.px_2().child("No songs found"),
            )
    }
}
