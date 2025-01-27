use macroquad::prelude::*;

use crate::tools::*;
use crate::ui_theme::*;

type ButtonClickHandler = fn(ctx: ButtonClickHandlerContext);

type ButtonRenderFn = fn(button: &Button, is_hovered: bool, theme: &UITheme) -> ();

pub struct ButtonClickHandlerContext<'a> {
  pub ui_theme: &'a mut UITheme,
  pub buttons: &'a mut Vec<Button>,
  pub tools: &'a mut Tools,
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

pub struct CreateButtonsContext<'a> {
  pub ui_theme: &'a UITheme,
  pub tools: &'a Tools,
}

pub fn create_buttons(buttons: &mut Vec<Button>, ctx: CreateButtonsContext) {
  let CreateButtonsContext { ui_theme, tools } = ctx;

  buttons.clear();

  let button_size = ui_theme.get_tool_panel_button_size();

  for (i, _tool) in tools.tools.iter().enumerate() {
    buttons.push(Button {
      rect: Rect::new(
        // x
        ui_theme.panel_padding,
        // y
        ui_theme.panel_padding + ((button_size + ui_theme.element_margin) * (i as f32)),
        button_size,
        button_size,
      ),
      render_background: render_fns::render_rectangular_button_background,
      render_foreground: render_fns::render_circle_button_foreground,
      on_click: Some(click_handlers::randomize_ui_theme_click_handler),
    });
  }

  // buttons.push(Button {
  //   rect: Rect::new(
  //     ui_theme.screen_padding,
  //     ui_theme.screen_padding,
  //     ui_theme.standard_button_size,
  //     ui_theme.standard_button_size,
  //   ),
  //   render_background: render_fns::render_rectangular_button_background,
  //   render_foreground: render_fns::render_circle_button_foreground,
  //   on_click: Some(click_handlers::randomize_ui_theme_click_handler),
  // });

  // buttons.push(Button {
  //   rect: Rect::new(
  //     ui_theme.screen_padding,
  //     ui_theme.screen_padding + ui_theme.standard_button_size + ui_theme.element_margin,
  //     ui_theme.standard_button_size,
  //     ui_theme.standard_button_size,
  //   ),
  //   render_background: render_fns::render_rectangular_button_background,
  //   render_foreground: render_fns::render_draw_text_button_foreground,
  //   on_click: Some(click_handlers::randomize_ui_theme_click_handler),
  // });
}

pub mod render_fns {
  use macroquad::prelude::*;

  use crate::buttons::Button;
  use crate::ui_theme::UITheme;

  pub fn render_rectangular_button_background(
    button: &Button,
    is_hovered: bool,
    ui_theme: &UITheme,
  ) {
    let color = if is_hovered {
      ui_theme.interactive_background_primary_hovered
    } else {
      ui_theme.interactive_background_primary_default
    };

    let Rect { x, y, w, h } = button.rect;

    draw_rectangle(x, y, w, h, color);
  }

  pub fn render_circle_button_foreground(button: &Button, _: bool, ui_theme: &UITheme) {
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

  pub fn render_draw_text_button_foreground(button: &Button, _: bool, ui_theme: &UITheme) {
    let Rect {
      x: button_x,
      y: button_y,
      w: button_width,
      h: button_height,
    } = button.inner_rect(ui_theme);

    let text = &"draw";
    let font_size = 14;
    let TextDimensions {
      width: text_width, ..
    } = measure_text(text, None, font_size, 2.);

    // Center text in rect
    let x = button_x + ((button_width - (text_width / 2.)) / 2.);
    let y = button_y + (button_height / 2.);

    draw_text(text, x, y, font_size as f32, ui_theme.foreground_primary);
  }
}

pub mod click_handlers {
  use crate::buttons::ButtonClickHandlerContext;

  use super::create_buttons;

  pub fn randomize_ui_theme_click_handler(ctx: ButtonClickHandlerContext) {
    let ButtonClickHandlerContext {
      ui_theme,
      buttons,
      tools,
    } = ctx;
    ui_theme.randomize();
    create_buttons(buttons, super::CreateButtonsContext { ui_theme, tools })
  }
}
