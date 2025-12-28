use gpui::{
    ClickEvent, Context, ImageSource, InteractiveElement, Interactivity, IntoElement,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement, Render, RenderOnce, Resource,
    StatefulInteractiveElement, Styled, Window, div, img, rgb,
};
use log::info;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(crate) struct PlayActionState {
    pub(crate) is_playing: bool,
}

#[derive(Clone, Copy)]
pub struct PlayElement {
    pub(crate) state: Option<PlayActionState>,
}
impl PlayElement {
    pub fn new() -> Self {
        Self {
            state: Some(PlayActionState { is_playing: false }),
        }
    }
    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        info!("Mouse down at position: {:?}", event.position);
        if self.state.is_some() {
            self.state.as_mut().unwrap().is_playing = true;
        } else {
            self.state = Some(PlayActionState { is_playing: false });
        }
        cx.notify();
    }

    fn on_mouse_up(&mut self, event: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        info!("Mouse up at position: {:?}", event.position);
        cx.notify();
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        info!("Mouse moving at position: {:?}", event.position);
        cx.notify();
    }

    fn debug_mouse_event(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        app: &mut Context<Self>,
    ) {
        info!("debug mouse event");
        app.notify();
    }

    fn on_click(&mut self, event: &ClickEvent, window: &mut Window, app: &mut Context<Self>) {
        info!("on click event...");
        window.refresh();
        app.notify();
    }
}

impl Render for PlayElement {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        info!("Rendering PlayElement");
        let play_button_img = ImageSource::Resource(Resource::Path(Arc::from(Path::new(
            "assets/play-button.png",
        ))));

        div()
            .debug()
            .id("play-button")
            .size_16()
            .on_click(cx.listener(Self::on_click))
            .on_mouse_down(gpui::MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(gpui::MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_any_mouse_down(cx.listener(Self::debug_mouse_event))
            .child(img(play_button_img).size_full())
    }
}
