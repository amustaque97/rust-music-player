use gpui::{
    ClickEvent, Context, ImageSource, InteractiveElement, ParentElement, Render, Resource,
    StatefulInteractiveElement, Styled, Window, div, img,
};
use log::info;
use std::path::Path;
use std::sync::{Arc, mpsc};

use crate::PlayerCommand;
use crate::audio_manager::AudioManager;

type SharedAudioManager = Arc<AudioManager>;

pub struct PlayElement {
    pub(crate) is_playing: bool,
    btn: ImageSource,
    pub(crate) audio_manager: SharedAudioManager,
    state_reciever: mpsc::Receiver<PlayerCommand>,
}
impl PlayElement {
    pub fn new(audio_manager: SharedAudioManager, rx: mpsc::Receiver<PlayerCommand>) -> Self {
        Self {
            is_playing: false,
            btn: ImageSource::Resource(Resource::Path(Arc::from(Path::new(
                "assets/play-button.png",
            )))),
            audio_manager,
            state_reciever: rx,
        }
    }

    fn on_click(&mut self, _: &ClickEvent, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_playing = !self.is_playing;
        self.update_icon();
    }

    fn update_icon(&mut self) {
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

        // todo(mustaq): fix and see if any other approach can be applied
        // this is blocking the main thread so let's use `try_recv()` and move on
        match self.state_reciever.try_recv() {
            Ok(PlayerCommand::Paused) => {
                self.is_playing = false;
                self.update_icon();
            }
            Ok(PlayerCommand::Play) => {
                self.is_playing = true;
                self.update_icon();
            }
            _ => {}
        }
        div()
            .id("play-button")
            .size_16()
            .on_click(cx.listener(Self::on_click))
            .child(img(self.btn.clone()).size_full())
    }
}
