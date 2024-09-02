use macroquad::prelude::*;

fn random_color() -> Color {
  Color {
    r: rand::gen_range(0., 1.),
    g: rand::gen_range(0., 1.),
    b: rand::gen_range(0., 1.),
    a: rand::gen_range(0., 1.),
  }
}

pub struct UITheme {
  pub background_primary: Color,
  pub interactive_background_primary_default: Color,
  pub interactive_background_primary_hovered: Color,
  pub foreground_primary: Color,

  pub button_padding: f32,
}

impl UITheme {
  pub fn random() -> Self {
    Self {
      background_primary: random_color(),
      interactive_background_primary_default: random_color(),
      interactive_background_primary_hovered: random_color(),
      foreground_primary: random_color(),

      button_padding: rand::gen_range(2., 50.),
    }
  }

  pub fn randomize(&mut self) {
    let r = Self::random();

    self.background_primary = r.background_primary;
    self.interactive_background_primary_default = r.interactive_background_primary_default;
    self.interactive_background_primary_hovered = r.interactive_background_primary_hovered;
    self.foreground_primary = r.foreground_primary;

    self.button_padding = r.button_padding;
  }
}
