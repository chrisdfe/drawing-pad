use macroquad::prelude::*;

use crate::ui_theme::UITheme;

type ButtonClickHandler = fn(button: &Button, ctx: ButtonClickHandlerContext);

type ButtonRenderFn = fn(btn: &Button, is_hovered: bool, theme: &UITheme) -> ();

pub struct ButtonClickHandlerContext<'a> {
  pub ui_theme: &'a mut UITheme,
}

pub struct Button {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
  pub render_background: ButtonRenderFn,
  pub render_foreground: ButtonRenderFn,
  pub on_click: Option<ButtonClickHandler>,
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

    //
    draw_rectangle(button.x, button.y, button.width, button.height, color);
  }

  pub fn render_close_button_foreground(button: &Button, _: bool, ui_theme: &UITheme) {
    const PADDING: f32 = 10.;

    let width = button.width - (PADDING * 2.);
    let height = button.height - (PADDING * 2.);

    let x = button.x + (width / 2.) + PADDING;
    let y = button.y + (height / 2.) + PADDING;

    let radius = (button.width - (PADDING * 2.)) / 2.;

    draw_circle(x, y, radius, ui_theme.foreground_primary);
  }
}

pub mod click_handlers {
  use crate::buttons::{Button, ButtonClickHandlerContext};

  pub fn randomize_ui_theme_click_handler(_: &Button, ctx: ButtonClickHandlerContext) {
    let ButtonClickHandlerContext { ui_theme } = ctx;
    ui_theme.randomize();
  }
}
