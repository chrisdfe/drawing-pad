use macroquad::prelude::*;

fn random_color() -> Color {
  Color {
    r: rand::gen_range(0., 1.),
    g: rand::gen_range(0., 1.),
    b: rand::gen_range(0., 1.),
    a: rand::gen_range(0., 1.),
  }
}

struct UITheme {
  background_primary: Color,
  interactive_background_primary_default: Color,
  interactive_background_primary_hovered: Color,
  foreground_primary: Color,
}

impl UITheme {
  fn random() -> Self {
    Self {
      background_primary: random_color(),
      interactive_background_primary_default: random_color(),
      interactive_background_primary_hovered: random_color(),
      foreground_primary: random_color(),
    }
  }

  pub fn randomize(&mut self) {
    let r = Self::random();
    self.background_primary = r.background_primary;
    self.interactive_background_primary_default = r.interactive_background_primary_default;
    self.interactive_background_primary_hovered = r.interactive_background_primary_hovered;
    self.foreground_primary = r.foreground_primary;
  }
}

type ButtonRenderFn = fn(btn: &Button, is_hovered: bool, theme: &UITheme) -> ();

struct ButtonClickHandlerContext<'a> {
  ui_theme: &'a mut UITheme,
}

type ButtonClickHandler = fn(button: &Button, ctx: ButtonClickHandlerContext);

struct Button {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
  pub render_background: ButtonRenderFn,
  pub render_foreground: ButtonRenderFn,
  pub on_click: Option<ButtonClickHandler>,
}

fn render_rectangular_button(button: &Button, is_hovered: bool, ui_theme: &UITheme) {
  let color = if is_hovered {
    ui_theme.interactive_background_primary_hovered
  } else {
    ui_theme.interactive_background_primary_default
  };

  //
  draw_rectangle(button.x, button.y, button.width, button.height, color);
}

fn render_close_button_foreground(button: &Button, _: bool, ui_theme: &UITheme) {
  const PADDING: f32 = 10.;

  let width = button.width - (PADDING * 2.);
  let height = button.height - (PADDING * 2.);

  let x = button.x + (width / 2.) + PADDING;
  let y = button.y + (height / 2.) + PADDING;

  let radius = (button.width - (PADDING * 2.)) / 2.;

  draw_circle(x, y, radius, ui_theme.foreground_primary);
}

fn randomize_ui_theme_click_handler(_: &Button, ctx: ButtonClickHandlerContext) {
  // noop
  let ButtonClickHandlerContext { ui_theme } = ctx;
  ui_theme.randomize();
}

#[macroquad::main("DrawingPad")]
async fn main() {
  // Seed rng
  rand::srand(macroquad::miniquad::date::now() as _);

  let mut ui_theme = UITheme::random();
  let mut hovered_button_idx: i32 = -1;
  let mut clicked_button_idx: i32 = -1;
  let buttons = vec![Button {
    x: 100.,
    y: 100.,
    width: 100.,
    height: 100.,
    render_background: render_rectangular_button,
    render_foreground: render_close_button_foreground,
    on_click: Some(randomize_ui_theme_click_handler),
  }];

  loop {
    if is_key_pressed(KeyCode::R) {
      ui_theme = UITheme::random()
    }

    // mouseover/mouseout
    {
      let (mouse_x, mouse_y) = mouse_position();
      let mut has_updated_hover: bool = false;
      for (idx, button) in buttons.iter().enumerate() {
        if mouse_x > button.x
          && mouse_x < button.x + button.width
          && mouse_y > button.y
          && mouse_y < button.y + button.height
        {
          hovered_button_idx = idx as i32;
          has_updated_hover = true;
          break;
        }
      }

      if has_updated_hover {
        if is_mouse_button_pressed(MouseButton::Left) {
          clicked_button_idx = hovered_button_idx;
        } else {
          clicked_button_idx = -1;
        }
      } else {
        hovered_button_idx = -1;
        clicked_button_idx = -1;
      }
    }

    // run button event handlers
    {
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
