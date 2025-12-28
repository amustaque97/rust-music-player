use std::path::{self, Path};
mod play_element;
use gpui::{
    AppContext, Application, Asset, Bounds, Context, Entity, Image, ImageAssetLoader, ImageSource,
    InteractiveElement, KeyBinding, MouseMoveEvent, ParentElement, Pixels, Render, RenderImage,
    Resource, Size, Styled, Window, WindowBounds, WindowOptions, actions, div, img, px, rgb, size,
};
use log::{error, info};
use std::sync::Arc;

use play_element::PlayElement;

use crate::play_element::PlayActionState;

actions!(music_player, [Quit]);

struct MusicPlayer {
    play_btn: Entity<PlayElement>,
}

impl MusicPlayer {
    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        app: &mut Context<Self>,
    ) {
        info!("Inside mouse movement {:?}", event);
    }
}

impl Render for MusicPlayer {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
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
            .child(div().bg(rgb(0xC8E5EE)).gap_3().size_full())
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
                    .child(div().size_16().bg(rgb(0xff0000)))
                    .child(
                        div()
                            .size_16()
                            .bg(rgb(0xff0000))
                            .debug()
                            .child(self.play_btn.clone()),
                    )
                    .child(div().size_16().bg(rgb(0xff0000))),
            )
    }
}

fn main() {
    env_logger::init();
    info!("Music player starting...");
    let application = Application::new();

    let element = PlayElement::new();

    application.run(move |app| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), app);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        let window = app
            .open_window(window_options, |_, cx| {
                cx.new(|cx| MusicPlayer {
                    play_btn: cx.new(|_| element),
                })
            })
            .unwrap();
        let view = window.update(app, |_, _, cx| cx.entity()).unwrap();
        app.activate(true);
        app.on_action(|_: &Quit, app| app.quit());
        app.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    });
}
