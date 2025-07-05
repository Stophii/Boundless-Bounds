# Boundless-Bounds

## Description

A walkthrough on how to approach UI in Turbo!

## Lets talk about text

On any new Turbo project we are subject to a 256x144 canvas. This doesn't seem like a lot of space to allocate to a UI. On top of that it roughly allows 50 characters using the basic turbo font from screen to screen.

This usually amounts to about one sentence. Well when we are writing up descriptions one sentence doesn't always cut it.

I have a few tips and tricks that will remedy this early developer issue.

```rust
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
```

Introducing `wrap_textbetter` a function that will allow you to write out a string as long as you want and then set a character limit and wrap it.

It gets rid of any excess space if your sentence hit the character limit on an empty space and allows for clean text wrapping. Simple and easy.

It's even easy to call

```rust
let text = "This is a really long sentence that needs to wrap to a new line or it'll spill off screen.";
let wrapped = wrap_textbetter(text, 50);
text!(&wrapped, x = 0, y = 20, color = 0xffffffff);
```

Just set a `text` and a `wrapped`. the `wrapped` is just referencing the `text` and a character limit. You can rename `text` or `wrapped` to whatever you desire, it's flexible!

Here is an example!

<img width="880" alt="Screenshot 2025-07-05 at 2 06 58 PM" src="https://github.com/user-attachments/assets/2aa5df64-7255-42f0-b6b9-9353267e5747" />

> **Tip:** Turbo has it's own built in text wrapping with `text_box!` macro!

## UI done simply

Most games you make are going to need a UI of some sort and that UI will most likely consist of menus. A really simple way to set up a menu is through a `selection` in the gamestate intiliazed as a `usize`

```rust
struct GameState {
    menu_selection: usize,
  }
impl GameState {
    fn new() -> Self {
        Self {
            menu_selection: 0,
        }
    }
```

I've called mine `menu_selection` and with it you can easily setup a simple menu.

Combining `wrap_textbetter` and the `menu_selection` will give me a basic menu if I add in some `gamepad` inputs to change the value of `menu_selection`

Lets change the `update` to show just how simple it can be to add in UI elements

```rust
fn update(&mut self) {
    let text = "Shop Fight Run Item";
    let wrapped = wrap_textbetter(text, 5);
    text!(&wrapped, x = 0, y = 20, color = 0xffffffff);

    if gamepad(0).up.just_pressed() {
        if self.menu_selection > 0 {
            self.menu_selection -= 1
        } else {
            self.menu_selection = 3
        }
    };

    if gamepad(0).down.just_pressed() {
        if self.menu_selection < 3 {
            self.menu_selection += 1
        } else {
            self.menu_selection = 0
        }
    };

    if (0..=3).contains(&self.menu_selection) {
        rect!(
            x = 30,
            y = 20 + self.menu_selection * 9,
            w = 9,
            h = 9,
            rotation = 45,
            color = 0xff0000ff
        );
    }
}
```

![Menufast](https://github.com/user-attachments/assets/560497e4-b97d-4ac3-b5ee-0fe24e6fff7e)

A nice simple easy menu. You can highlight the text instead and use `match` instead of `contains` for more control

```rust
match self.menu_selection {
    0 => rect!(x = 30, y = 20, w = 9, h = 9, rotation = 45, color = 0xff0000ff),
    1 => rect!(x = 30, y = 29, w = 9, h = 9, rotation = 45, color = 0xff0000ff),
    2 => rect!(x = 30, y = 38, w = 9, h = 9, rotation = 45, color = 0xff0000ff),
    3 => rect!(x = 30, y = 47, w = 9, h = 9, rotation = 45, color = 0xff0000ff),
    _ => {},
}
```

Using `match` will allow us to do a number of things inside each `match` arm so it could be more helpful for a simple menu with more options.

> **Tip:** A new line of text in Turbo is roughly 9 units down using the default font, `medium`. 


## UI done smart

Turbo has your back though, not everything needs to be `wrap_textbetter` and if that is your solution then awesome, you're embracing the fast, simple, and fun style of making games in Turbo. In Turbo we have bounds which can be used on any canvas and it's automatically going to anchor and space things out to your desire.

Step one is to define the `canvas_bounds`

```rust
let canvas_bounds = bounds::canvas();
```

call it what you like but with this setup it's going to pull the resolution from your `turbo.toml` and you'll be able to use that setup a HUD of some sort.

lets approach menus again

```rust
let canvas_bounds = bounds::canvas();

let buttons = canvas_bounds
    .height(100)
    .inset_left(80)
    .inset_right(80)
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

    // === Change color if hovered ===
    let (fill_color, border_color) = if self.menu_selection == i {
        (0x82c8ffff, 0xffc0cbff) // Hovered colors
    } else {
        (0x0055ffff, 0xffc0cbff) // Default colors
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
}
```

We can add this under the `match` or `contains` from earlier. This gives us the same control functionality but on a menu written with `bounds`.

Here is a preview with the old menu still visible

![output](https://github.com/user-attachments/assets/310a32c4-ebad-42ac-8ce4-5540b3f15a74)


## Adding Sprites

Now let's get into adding some sprites! First off we need a sprites folder, this is as simple as making a new folder named `sprites` right alongside your src folder.

Sprites added to this folder in the file format of: `.png`, `.bmp`, `.jpg/.jpeg`, `.gif`, `.webp`, and `.ase/.aseprite` will be able to be used!

> **Tip:** You can make a `fonts` and `audio` folder in the same place to add sounds or sound effects as well as fonts!

```rust
sprite!("Decapod#Idle");
sprite!("Wizzy", x = 150, y = 45);
```

Sprites made in Aesprite will need to have their appropriate tag referenced, in this scenario I am choosing to display the idle animation with the `#Idle` tag.

If you wanted to use inputs to change your sprite you can use an animation key and access animation commands

```rust
    let crab = animation::get("crab");

    sprite!(
	animation_key = "crab",
	default_sprite = "Decapod#Idle",
    );

    if gamepad(0).right.just_pressed() {
	crab.use_sprite("Decapod#Ranged Attack");
	crab.set_repeat(1);
    }
```

now if I hit the right arrow key or D on the keyboard my sprite will use it's `#Ranged Attack` and then go back to `#Idle`

> **Tip:** Check out the sprite SDK for more info [here](https://docs.turbo.computer/learn/api/sprites)! 


## Power of Tween

Now we have some sprites but nothing moves other than the animations I have. let's go ahead and add some tweening so the wizzy dodged the attack!

First let's add a `Tween` into the `Gamestate`

```rust
turbo::init! {
    struct GameState {
        screen: Screen,
        wizzy_tween: Tween<f32>,
    } = {
        Self {
            screen: Screen::Title,
            wizzy_tween: Tween::new(150.0)
                .duration(120)
                .ease(Easing::EaseOutCubic),

        }
    }
}
```

with `wizzy_tween` we just need to go change our x position of the wizzy sprite to the `wizzy_tween` we can do that with `.get()`


```rust
let x = state.wizzy_tween.get();

```

and update the sprite to match

```rust
sprite!("Wizzy", x = x, y = 45);

```

and let's just use our gamepad input as the trigger to change the position

```rust
    if gamepad(0).right.just_pressed() {
	crab.use_sprite("Decapod#Ranged Attack");
	crab.set_repeat(1);
	state.wizzy_tween.set(200.0); //<-- add this
    }
```

now the wizzy dodges the attack!

If we wanted we can reset him like this

```rust
    if state.wizzy_tween.done() {
	state.wizzy_tween.set(150.0);
    }
```

## Pushing the Bounds

Alright now for the grand finale let's add in a button we can click with the mouse. We'll use `bounds` to accomplish this. let's start off by adding in our `pointer` and our `canvas_bounds`

```rust
    let p = pointer();
    let canvas_bounds = bounds::canvas();
```

Now let's build on this by adding in our button with `bounds`!

```rust
    let button = Bounds::with_size(48, 14)
	.anchor_center(&canvas_bounds)
	.translate_y(16);
    
    let is_btn_hovered = p.xy().intersects_bounds(button);

    let (regular_color, hover_color, pressed_color) = (0x33CCFFff, 0x66DDFFFF, 0x00FFFFFF);

    let button_color = if p.pressed() && is_btn_hovered {
	pressed_color
    } else if is_btn_hovered {
	hover_color
    } else {
	regular_color
    };

    rect!(
	color = button_color,
	xy = button.xy(),
	wh = button.wh(),
	border_radius = 2,
    );

    let label_bounds = button.inset_left(4).inset_top(4);
    text!("Attack!", x = label_bounds.x(), y = label_bounds.y(), font = "medium");
```

and finally let's add a reaction for clicking this button

```rust
    if is_btn_hovered && p.just_pressed() {
	crab.use_sprite("Decapod#Ranged Attack");
	crab.set_repeat(1);
	state.wizzy_tween.set(200.0);
    }
```

Now instead of key inputs we can just press the button to try to attack the Wizzy!

Thanks for following along, hopefully you understand how to use a `State Machine`, add a `Sprite`, `Tween` an object, and use `Bounds`!

>**Tip** Join our [discord](discord.gg/V5YWWvQvKW) and message `Stophy` for questions!
