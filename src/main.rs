use macroquad::prelude::*;

mod buttons;
use buttons::*;

mod ui_theme;
use ui_theme::UITheme;

mod tools;
use tools::*;

#[macroquad::main("DrawingPad")]
async fn main() {
  // Seed rng
  rand::srand(macroquad::miniquad::date::now() as _);

  let mut ui_theme = UITheme::random();
  let mut hovered_button_idx: i32 = -1;
  let mut clicked_button_idx: i32;

  let mut tools = Tools::new();

  let mut buttons: Vec<Button> = Vec::new();
  create_buttons(
    &mut buttons,
    CreateButtonsContext {
      ui_theme: &ui_theme,
      tools: &tools,
    },
  );

  // TODO - refresh this + canvas image when UI theme updates?
  let canvas_rect = {
    let UITheme {
      screen_padding,
      tools_panel_width,
      ..
    } = ui_theme;

    let x = screen_padding + tools_panel_width;
    let y = screen_padding;

    let width = screen_width() - ((screen_padding * 2.) + tools_panel_width);
    let height = screen_height() - (screen_padding * 2.);

    Rect::new(x, y, width, height)
  };

  let mut canvas_image = Image::gen_image_color(canvas_rect.w as u16, canvas_rect.h as u16, WHITE);
  let canvas_texture = Texture2D::from_image(&canvas_image);

  let brush_size: u16 = 5;
  let brush_color = RED;

  loop {
    // listen for mouse events
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

      let mut brush_pixel_buffer: Vec<(u32, u32)> = Vec::new();
      for x in mouse_position_with_offset.x as i16 - (brush_size as i16)
        ..mouse_position_with_offset.x as i16 + (brush_size as i16)
      {
        for y in mouse_position_with_offset.y as i16 - (brush_size as i16)
          ..mouse_position_with_offset.y as i16 + (brush_size as i16)
        {
          // prevent overflow
          if x >= 0 && y >= 0 {
            brush_pixel_buffer.push((x as u32, y as u32))
          }
        }
      }

      for (x, y) in brush_pixel_buffer {
        canvas_image.set_pixel(x, y, brush_color);
      }

      canvas_texture.update(&canvas_image);
    }

    // run button event handlers

    if clicked_button_idx >= 0 {
      let clicked_button = &buttons[clicked_button_idx as usize];

      if let Some(on_click) = clicked_button.on_click {
        (on_click)(ButtonClickHandlerContext {
          buttons: &mut buttons,
          ui_theme: &mut ui_theme,
          tools: &mut tools,
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
