use macroquad::prelude::*;

fn random_color() -> Color {
  Color {
    r: rand::gen_range(0., 1.),
    g: rand::gen_range(0., 1.),
    b: rand::gen_range(0., 1.),
    // a: rand::gen_range(0., 1.),
    a: 1.,
  }
}

pub struct UITheme {
  pub background_primary: Color,
  pub interactive_background_primary_default: Color,
  pub interactive_background_primary_hovered: Color,
  pub foreground_primary: Color,

  pub tools_panel_width: f32,
  pub panel_padding: f32,

  pub button_padding: f32,
  pub screen_padding: f32,
  pub element_margin: f32,
}

impl UITheme {
  // // a sensible default
  // pub fn basic() -> Self {
  //   Self {
  //     background_primary: WHITE,
  //     interactive_background_primary_default: random_color(),
  //     interactive_background_primary_hovered: random_color(),
  //     foreground_primary: random_color(),

  //     tools_panel_width: rand::gen_range(50., 100.),
  //     panel_padding: rand::gen_range(5., 25.),

  //     button_padding: rand::gen_range(2., 50.),
  //     screen_padding: rand::gen_range(2., 50.),
  //     element_margin: rand::gen_range(2., 50.),
  //   }
  // }

  pub fn random() -> Self {
    Self {
      background_primary: random_color(),
      interactive_background_primary_default: random_color(),
      interactive_background_primary_hovered: random_color(),
      foreground_primary: random_color(),

      tools_panel_width: rand::gen_range(50., 100.),
      panel_padding: rand::gen_range(5., 25.),

      button_padding: rand::gen_range(2., 50.),
      screen_padding: rand::gen_range(2., 50.),
      element_margin: rand::gen_range(2., 50.),
    }
  }

  pub fn get_tool_panel_button_size(&self) -> f32 {
    self.tools_panel_width - (self.panel_padding * 2.)
  }

  pub fn randomize(&mut self) {
    let r = Self::random();

    self.background_primary = r.background_primary;
    self.interactive_background_primary_default = r.interactive_background_primary_default;
    self.interactive_background_primary_hovered = r.interactive_background_primary_hovered;
    self.foreground_primary = r.foreground_primary;

    self.tools_panel_width = r.tools_panel_width;
    self.panel_padding = r.panel_padding;

    self.button_padding = r.button_padding;
    self.screen_padding = r.screen_padding;
    self.element_margin = r.element_margin;
  }
}
