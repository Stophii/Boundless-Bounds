//lib.rs

use turbo::*;


#[turbo::game]
struct GameState {
    menu_selection: usize,
}

impl GameState {
    fn new() -> Self {
        Self {
            menu_selection: 0,
        }
    }
    fn update(&mut self) {
        //controls
        if gamepad::get(0).up.just_pressed() {
            if self.menu_selection > 0 {
                self.menu_selection -= 1
            } else {
                self.menu_selection = 3
            }
        };

        if gamepad::get(0).down.just_pressed() {
            if self.menu_selection < 3 {
                self.menu_selection += 1
            } else {
                self.menu_selection = 0
            }
        };
        
        //menu via bounds
        let canvas_bounds = bounds::canvas();

        let buttons = canvas_bounds
            .height(100)
            .width(50)
            .anchor_center(&canvas_bounds)
            .rows_with_gap(4, 12);

        for (i, btn) in buttons.into_iter().enumerate() {
            let label = match i {
                0 => "Shop",
                1 => "Fight",
                2 => "Run",
                3 => "Item",
                _ => "",
            };

            let (fill_color, border_color) = if self.menu_selection == i {
                if gamepad::get(0).a.just_pressed() {
                    (0xffffffff, 0xffc0cbff)
                } else {
                    (0x82c8ffff, 0xffc0cbff)
                }
            } else {
                (0x0055ffff, 0xffc0cbff)
            };

            rect!(
                color = fill_color,
                w = btn.w(),
                h = btn.h(),
                x = btn.x(),
                y = btn.y(),
                border_radius = 4,
                border_size = 1,
                border_color = border_color,
            );

            let btn_inner = btn.inset_left(2).inset_top(4);

            text_box!(
                label,
                x = btn_inner.x(),
                y = btn_inner.y(),
                w = btn.w(),
                h = btn.h(),
                font = "medium",
                color = 0xffffffff
            );


            if self.menu_selection == i {
                let sprite = btn
                    .anchor_left(&btn)
                    .translate_x(-12)
                    .anchor_center_y(&btn); // center vertically

                sprite!(
                    "Slime",
                    x = sprite.x(),
                    y = sprite.y(),
                )
            }
        }
    }
}

pub fn wrap_textbetter(text: &str, max_line_length: usize) -> String {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if !current_line.is_empty() && current_line.len() + word.len() + 1 > max_line_length {
            lines.push(current_line);
            current_line = String::new();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines.join("\n")
}