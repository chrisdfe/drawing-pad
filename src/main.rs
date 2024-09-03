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

  let mut buttons: Vec<Button> = Vec::new();
  create_buttons(&mut buttons, &ui_theme);

  let canvas_rect = Rect::new(500., 200., 200., 200.);
  let mut canvas_image = Image::gen_image_color(canvas_rect.w as u16, canvas_rect.h as u16, WHITE);
  let canvas_texture = Texture2D::from_image(&canvas_image);

  let brush_size: u16 = 2;

  loop {
    // if is_key_pressed(KeyCode::R) {
    //   ui_theme = UITheme::random()
    // }

    // mouseover/mouseout

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

    clicked_button_idx = if hovered_button_idx >= 0 && is_mouse_button_pressed(MouseButton::Left) {
      hovered_button_idx
    } else {
      -1
    };

    // drawing
    if is_mouse_button_down(MouseButton::Left) && canvas_rect.contains(mouse_position_vec) {
      let mouse_position_with_offset = Vec2::new(
        mouse_position_vec.x - canvas_rect.x,
        mouse_position_vec.y - canvas_rect.y,
      );

      canvas_image.set_pixel(
        mouse_position_with_offset.x as u32,
        mouse_position_with_offset.y as u32,
        RED,
      );
      canvas_texture.update(&canvas_image);
    }

    // run button event handlers

    if clicked_button_idx >= 0 {
      let clicked_button = &buttons[clicked_button_idx as usize];

      if let Some(on_click) = clicked_button.on_click {
        (on_click)(ButtonClickHandlerContext {
          buttons: &mut buttons,
          ui_theme: &mut ui_theme,
        })
      }
    }

    // Render
    clear_background(ui_theme.background_primary);

    for (idx, button) in buttons.iter().enumerate() {
      let is_hovered = idx as i32 == hovered_button_idx;
      (button.render_background)(button, is_hovered, &ui_theme);
      (button.render_foreground)(button, is_hovered, &ui_theme);
    }

    draw_texture(&canvas_texture, canvas_rect.x, canvas_rect.y, WHITE);

    next_frame().await
  }
}
