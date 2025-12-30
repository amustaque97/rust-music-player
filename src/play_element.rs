use gpui::{
    ClickEvent, Context, ImageSource, InteractiveElement, ParentElement, Render, Resource,
    StatefulInteractiveElement, Styled, Window, div, img,
};
use log::info;
use std::path::Path;
use std::sync::Arc;

use crate::audio_manager::AudioManager;

type SharedAudioManager = Arc<AudioManager>;

pub struct PlayElement {
    pub(crate) is_playing: bool,
    btn: ImageSource,
    pub(crate) audio_manager: SharedAudioManager,
}
impl PlayElement {
    pub fn new(audio_manager: SharedAudioManager) -> Self {
        Self {
            is_playing: false,
            btn: ImageSource::Resource(Resource::Path(Arc::from(Path::new(
                "assets/play-button.png",
            )))),
            audio_manager,
        }
    }

    fn on_click(&mut self, _: &ClickEvent, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_playing = !self.is_playing;

        if self.is_playing {
            self.btn = ImageSource::Resource(Resource::Path(Arc::from(Path::new(
                "assets/pause-button.png",
            ))));

            self.audio_manager.play();
        } else {
            self.btn = ImageSource::Resource(Resource::Path(Arc::from(Path::new(
                "assets/play-button.png",
            ))));

            self.audio_manager.pause();
        }
    }

    /// Seek forward by 10 seconds
    pub fn seek_forward(&self) {
        self.audio_manager.seek_forward();
    }

    /// Seek backward by 10 seconds
    pub fn seek_backward(&self) {
        self.audio_manager.seek_backward();
    }
}

impl Render for PlayElement {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        info!("debug value {:?}", self.is_playing);
        div()
            .id("play-button")
            .size_16()
            .on_click(cx.listener(Self::on_click))
            .child(img(self.btn.clone()).size_full())
    }
}
