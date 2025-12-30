use std::path::Path;
mod audio_manager;
mod music_list_view;
mod play_element;
use gpui::{
    AppContext, Application, Bounds, ClickEvent, Entity, ImageSource, InteractiveElement,
    KeyBinding, ParentElement, Render, Resource, StatefulInteractiveElement, Styled, Window,
    WindowBounds, WindowOptions, actions, div, img, px, rgb, size,
};
use log::info;
use std::sync::Arc;

use crate::music_list_view::ListView;
use play_element::PlayElement;

actions!(music_player, [Quit]);

struct MusicPlayer {
    play_btn: Entity<PlayElement>,
    songs_list: Entity<ListView>,
}

impl Render for MusicPlayer {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let play_btn = self.play_btn.clone();
        let play_btn_for_forward = self.play_btn.clone();
        let play_btn_for_backward = self.play_btn.clone();
        let songs_list_view = self.songs_list.clone();

        let next_button_img = ImageSource::Resource(Resource::Path(Arc::from(Path::new(
            "assets/right-button.png",
        ))));
        let prev_button_img = ImageSource::Resource(Resource::Path(Arc::from(Path::new(
            "assets/left-button.png",
        ))));

        div()
            .bg(rgb(0xFAF9F6))
            .flex()
            .justify_center()
            .flex_col()
            .items_center()
            .flex_1()
            .gap_3()
            .size_full()
            // App title
            .child(div().text_center().child("Music Player").text_3xl())
            // music list
            .child(songs_list_view)
            // control plane
            .child(
                div()
                    .flex()
                    .flex_1()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .bg(rgb(0x1C4A5A))
                    .w(px(800.))
                    .h(px(100.))
                    // Seek backward button
                    .child(
                        div()
                            .id("seek-backward")
                            .size_16()
                            .on_click(cx.listener(
                                move |_this, _event: &ClickEvent, _window, cx| {
                                    play_btn_for_backward.update(cx, |play_element, _cx| {
                                        play_element.seek_backward();
                                    });
                                },
                            ))
                            .child(img(prev_button_img).size_full()),
                    )
                    // Play/pause button
                    .child(play_btn)
                    // Seek forward button
                    .child(
                        div()
                            .id("seek-forward")
                            .size_16()
                            .on_click(cx.listener(
                                move |_this, _event: &ClickEvent, _window, cx| {
                                    play_btn_for_forward.update(cx, |play_element, _cx| {
                                        play_element.seek_forward();
                                    });
                                },
                            ))
                            .child(img(next_button_img).size_full()),
                    ),
            )
    }
}

fn main() {
    env_logger::init();
    info!("Music player starting...");
    let application = Application::new();

    let element = PlayElement::new();
    let mut list_view = ListView::new();
    list_view.load_songs();

    application.run(move |app| {
        let bounds = Bounds::centered(None, size(px(800.0), px(800.0)), app);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        let window = app
            .open_window(window_options, |_, cx| {
                cx.new(|cx| MusicPlayer {
                    play_btn: cx.new(|_| element),
                    songs_list: cx.new(|_| list_view),
                })
            })
            .unwrap();
        let _view = window.update(app, |_, _, cx| cx.entity()).unwrap();
        app.activate(true);
        app.on_action(|_: &Quit, app| app.quit());
        app.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        app.bind_keys([KeyBinding::new("ctrl-c", Quit, None)]);
    });
}
