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
  interactive_background_primary: Color,
  foreground_primary: Color,
}

impl UITheme {
  fn random() -> Self {
    Self {
      background_primary: random_color(),
      interactive_background_primary: random_color(),
      foreground_primary: random_color(),
    }
  }
}

type ButtonRenderFn = fn(btn: &Button, theme: &UITheme) -> ();

struct Button {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
  pub render_background: ButtonRenderFn,
  pub render_foreground: ButtonRenderFn,
}

fn render_rectangular_button(button: &Button, ui_theme: &UITheme) {
  //
  draw_rectangle(
    button.x,
    button.y,
    button.width,
    button.height,
    ui_theme.interactive_background_primary,
  );
}

fn render_close_button_foreground(button: &Button, ui_theme: &UITheme) {
  const PADDING: f32 = 10.;

  let width = button.width - (PADDING * 2.);
  let height = button.height - (PADDING * 2.);

  let x = button.x + (width / 2.) + PADDING;
  let y = button.y + (height / 2.) + PADDING;

  let radius = (button.width - (PADDING * 2.)) / 2.;

  draw_circle(x, y, radius, ui_theme.foreground_primary);
}

#[macroquad::main("DrawingPad")]
async fn main() {
  // Seed rng
  rand::srand(macroquad::miniquad::date::now() as _);

  let mut ui_theme = UITheme::random();
  let buttons = vec![Button {
    x: 100.,
    y: 100.,
    width: 100.,
    height: 100.,
    render_background: render_rectangular_button,
    render_foreground: render_close_button_foreground,
  }];

  loop {
    if is_key_pressed(KeyCode::R) {
      ui_theme = UITheme::random()
    }

    // mouseover/mouseout

    // Render
    clear_background(ui_theme.background_primary);

    for button in buttons.iter() {
      (button.render_background)(button, &ui_theme);
      (button.render_foreground)(button, &ui_theme);
    }

    next_frame().await
  }
}
