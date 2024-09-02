use macroquad::prelude::*;

use crate::ui_theme::UITheme;

type ButtonClickHandler = fn(button: &Button, ctx: ButtonClickHandlerContext);

type ButtonRenderFn = fn(btn: &Button, is_hovered: bool, theme: &UITheme) -> ();

pub struct ButtonClickHandlerContext<'a> {
  pub ui_theme: &'a mut UITheme,
}

pub struct Button {
  pub rect: Rect,
  pub render_background: ButtonRenderFn,
  pub render_foreground: ButtonRenderFn,
  pub on_click: Option<ButtonClickHandler>,
}

impl Button {
  pub fn inner_rect(&self, ui_theme: &UITheme) -> Rect {
    let x = self.rect.x + ui_theme.button_padding;
    let y = self.rect.y + ui_theme.button_padding;
    // TODO - clamping of some kind
    let width = self.rect.w - (ui_theme.button_padding * 2.);
    let height = self.rect.h - (ui_theme.button_padding * 2.);

    Rect::new(x, y, width, height)
  }
}

pub mod render_fns {
  use macroquad::prelude::*;

  use crate::buttons::Button;
  use crate::ui_theme::UITheme;

  pub fn render_rectangular_button(button: &Button, is_hovered: bool, ui_theme: &UITheme) {
    let color = if is_hovered {
      ui_theme.interactive_background_primary_hovered
    } else {
      ui_theme.interactive_background_primary_default
    };

    let Rect { x, y, w, h } = button.rect;

    draw_rectangle(x, y, w, h, color);
  }

  pub fn render_close_button_foreground(button: &Button, _: bool, ui_theme: &UITheme) {
    let Rect { x, y, w, .. } = button.inner_rect(ui_theme);

    let radius = w / 2.;
    let x_with_offset = x + radius;
    let y_with_offset = y + radius;

    draw_circle(
      x_with_offset,
      y_with_offset,
      radius,
      ui_theme.foreground_primary,
    );
  }
}

pub mod click_handlers {
  use crate::buttons::{Button, ButtonClickHandlerContext};

  pub fn randomize_ui_theme_click_handler(_: &Button, ctx: ButtonClickHandlerContext) {
    let ButtonClickHandlerContext { ui_theme } = ctx;
    ui_theme.randomize();
  }
}
