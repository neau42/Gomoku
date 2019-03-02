use crate::WidgetIds;
use conrod::*;
use conrod::UiCell;
use conrod::image::Id;

pub struct WindowView {
}

impl WindowView {
    pub fn new() -> WindowView {
        WindowView {
        }
    }

    pub fn display_canvas(&self, ui: &mut UiCell, widget_ids: &WidgetIds) {
        widget::Canvas::new()
            .border(1.0)
            .pad(50.0)
            .color(color::TRANSPARENT)
            .scroll_kids()
            .set(widget_ids.window_canvas, ui);
        widget::Scrollbar::x_axis(widget_ids.window_canvas).auto_hide(true).set(widget_ids.window_canvas_y_scrollbar, ui);
        widget::Scrollbar::y_axis(widget_ids.window_canvas).auto_hide(true).set(widget_ids.window_canvas_x_scrollbar, ui);
    }

    pub fn display_background(&self, ui: &mut UiCell, widget_ids: &WidgetIds, background_id: Id) {
        widget::Image::new(background_id).wh_of(ui.window).middle_of(ui.window).set(widget_ids.background, ui);        
    }

    pub fn display_title(&self, ui: &mut UiCell, widget_ids: &WidgetIds) {
        widget::Text::new("Gomoku")
            .mid_top_of(widget_ids.window_canvas)
            .font_size(50)
            .color(color::BLACK)
            .set(widget_ids.title, ui);
    }
}