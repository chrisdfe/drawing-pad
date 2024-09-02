use macroquad::prelude::*;

mod buttons;
use buttons::*;

mod ui_theme;
use ui_theme::UITheme;

#[macroquad::main("DrawingPad")]
async fn main() {
  // Seed rng
  rand::srand(macroquad::miniquad::date::now() as _);

  let mut ui_theme = UITheme::random();
  let mut hovered_button_idx: i32 = -1;
  let mut clicked_button_idx: i32;

  let buttons = vec![Button {
    rect: Rect::new(0., 0., 100., 100.),
    render_background: render_fns::render_rectangular_button,
    render_foreground: render_fns::render_close_button_foreground,
    on_click: Some(click_handlers::randomize_ui_theme_click_handler),
  }];

  loop {
    if is_key_pressed(KeyCode::R) {
      ui_theme = UITheme::random()
    }

    // mouseover/mouseout
    {
      let (mouse_x, mouse_y) = mouse_position();
      let mouse_position_vec = Vec2::new(mouse_x, mouse_y);

      let mut has_updated_hover: bool = false;
      for (idx, button) in buttons.iter().enumerate() {
        if button.rect.contains(mouse_position_vec) {
          hovered_button_idx = idx as i32;
          has_updated_hover = true;
          break;
        }
      }

      if !has_updated_hover {
        hovered_button_idx = -1;
      }

      clicked_button_idx = if hovered_button_idx >= 0 && is_mouse_button_pressed(MouseButton::Left)
      {
        hovered_button_idx
      } else {
        -1
      };
    }

    // run button event handlers

    if clicked_button_idx >= 0 {
      let clicked_button = &buttons[clicked_button_idx as usize];

      if let Some(on_click) = clicked_button.on_click {
        (on_click)(
          &clicked_button,
          ButtonClickHandlerContext {
            ui_theme: &mut ui_theme,
          },
        )
      }
    }

    // Render
    clear_background(ui_theme.background_primary);

    for (idx, button) in buttons.iter().enumerate() {
      let is_hovered = idx as i32 == hovered_button_idx;
      (button.render_background)(button, is_hovered, &ui_theme);
      (button.render_foreground)(button, is_hovered, &ui_theme);
    }

    next_frame().await
  }
}
