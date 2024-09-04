use macroquad::prelude::*;

pub struct ToolMouseCallbackFnContext<'a> {
  mouse_position: &'a Vec2,
}

pub type ToolMouseCallbackFn = fn(tool: &Tool, ctx: ToolMouseCallbackFnContext);

pub struct Tools {
  pub tools: Vec<Tool>,
  pub active_tool_idx: usize,
}

impl Tools {
  pub fn new() -> Self {
    Self {
      tools: vec![
        Tool {
          name: "Pencil".to_string(),
          on_mouse_down: None,
          on_mouse_up: None,
          // settings: vec![],
        },
        Tool {
          name: "Shape".to_string(),
          on_mouse_down: None,
          on_mouse_up: None,
          // settings: vec![],
        },
        Tool {
          name: "Erase".to_string(),
          on_mouse_down: None,
          on_mouse_up: None,
          // settings: vec![],
        },
      ],
      active_tool_idx: 0,
    }
  }
}

pub struct Tool {
  pub name: String,
  pub on_mouse_down: Option<ToolMouseCallbackFn>,
  pub on_mouse_up: Option<ToolMouseCallbackFn>,
  // pub settings: Vec<ToolSetting>,
}

// pub struct ToolSetting {
//   pub name: String,
//   pub value: ToolSettingValueType,
// }

// pub enum ToolSettingValueType {
//   Color(Color),
//   Integer(i32),
// }
