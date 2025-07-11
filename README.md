# Boundless-Bounds

## Description

A guide on [`bounds`](https://docs.turbo.computer/learn/api/bounds) with different approaches to making menus in Turbo!

## Let's Talk About Text

Every new Turbo project starts with a 256x144 canvas. That’s not a lot of space for a UI. On top of that it roughly allows 50 characters using the basic turbo font from screen to screen.

This usually amounts to about one sentence. Well, when we’re writing descriptions, one sentence doesn’t always cut it.

Turbo has the answer to this with `text_box!` which will wrap your text inside a specified `w` and `h`

It gets rid of any excess space if your sentence hit the character limit on an empty space and allows for clean text wrapping. Simple and easy.

It's even easy to call

```rust
text_box!("This is a really long sentence that needs to wrap to a new line or it'll spill off screen.", w = 256, h = 144);
```

Here is an example!

<img width="880" alt="Screenshot 2025-07-05 at 2 06 58 PM" src="https://github.com/user-attachments/assets/2aa5df64-7255-42f0-b6b9-9353267e5747" />

> **Tip:** Turbo has its own built in text wrapping with `text_box!` macro [here](https://docs.turbo.computer/learn/api/text_box/)!

## UI Done Simply

Most games you make are going to need a UI of some sort and that UI will most likely consist of menus. A really simple way to set up a menu is through a `selection` in the gamestate initialized as a `usize`

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

I've called mine `menu_selection` and with it you can easily set up a simple menu.

Combining `wrap_textbetter` and the `menu_selection` will give me a basic menu if I add in some `gamepad` inputs to change the value of `menu_selection`

Let's change the `update` to show just how simple it can be to add in UI elements

```rust
fn update(&mut self) {
    text_box!("Shop Fight Run Item", w = 30, h = 144, y = 20);


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

A nice simple easy menu. You can highlight the text instead and use `match` instead of `contains` for more control

![Simple](https://github.com/user-attachments/assets/5e3d95c7-fac9-40c4-9f9a-cd7748a9bd9e)

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


## UI Done Smart

Turbo has your back though, not everything needs to be `wrap_textbetter` and if that is your solution then awesome, you're embracing the fast, simple, and fun style of making games in Turbo. In Turbo we have bounds which can be used on any canvas and it's automatically going to anchor and space things out to your desire.

Step one is to define the `canvas_bounds`

```rust
let canvas_bounds = bounds::canvas();
```

Call it what you like, but with this setup it's going to pull the resolution from your `turbo.toml` and the first step to making a snazzy HUD is complete!

lets approach menus like we did earlier but with bounds

```rust
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

    // === Change color if hovered ===
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
}
```
We can add this under the `match` or `contains` from earlier. This gives us the same control functionality but on a menu written with `bounds`.

![Menu](https://github.com/user-attachments/assets/610f5881-80ed-4171-bc26-f456f5e57e54)

> **Tip:** Our controls don't change because we are using the same values and same amount of options! 

## Sprites & Bounds — Best of Friends

Now that we have the menu let's change out the square for a sprite. It's truly very simple to do

```rust
if self.menu_selection == i {
    let sprite = btn
        .anchor_left(&btn)
        .translate_x(-12)
        .anchor_center_y(&btn);

    sprite!(
        "Slime",
        x = sprite.x(),
        y = sprite.y(),
    )
}
```

If we make a new `bounds` by the name of `sprite` we can set it equal to `btn`

We can then alter the position of the bounds, I use `.anchor_left`, `.translate_x`, and `.anchor_center_y`. This puts it just left of the button

This allows our slimes to appear based on the `menu_selection` button after setting the sprites x and y values to `sprite.x()` and `sprite.y()`

![Sprite_menu](https://github.com/user-attachments/assets/51694b0e-e428-4549-86b7-a6cc74009ac6)

## Exploring Bounds On Your Own

Now that we've showcased what bounds can do, I'll give you a list of methods to get you started using bounds!

- `.height()` sets your bounds height the specified units.
- `.width()` sets your bounds width the specified units.
- `.translate_x()` move your bounds the specified units left or right.
- `.translate_y()` move your bounds the specified units up or down.
- `.inset_left()` shrinks your bounds the specified units on the left side. If inset **exceeds** width your bounds will disappear!
- `.inset_right()` shrinks your bounds the specified units on the right side. If inset **exceeds** width your bounds will disappear!
- `.inset_top()` adds empty space above your bounds shrinking them closer from above. If inset value is too high your bounds will disappear.
- `.inset_bottom()` adds empty space below your bounds shrinking them closer from below. If inset value is too high your bounds will disappear.
- `.anchor_left()` is the ability to anchor your bounds to the left of your specified bounds.
- `.anchor_right()` is the ability to anchor your bounds to the right of your specified bounds.
- `.anchor_center()` centers the bounds in the middle of the specified bounds.
- `.anchor_top()` anchors your bounds to the top of your specified bounds.
- `.anchor_bottom()` anchors your bounds to the bottom of your specified bounds.
- `.rows_with_gap(,)` set a number of rows and a gap size to use.
- `.columns_with_gap(,)` set a number of columns and a gap size to use.

You can try this out on something simple like this

```rs
let canvas_bounds = bounds::canvas();

let button = canvas_bounds
    .height(100)
    .width(50)
    .anchor_center(&canvas_bounds);

rect!(
    x = button.x(),
    y = button.y(),
    w = button.w(),
    h = button.h(),
    color = 0xffffffff,
);
```
this just draws a simple rect using bounds. This rect could be your pause menu, start menu, health bar overlay, you name it!

<img width="836" alt="Screenshot 2025-07-07 at 1 42 48 PM" src="https://github.com/user-attachments/assets/afed0785-9e70-4dae-9cda-fdfd490f1369" />

> **Tip:** Our specified bounds for this project are `&canvas_bounds`. You can easily set up and use a different bounds than `let canvas_bounds = bounds::canvas()`, it's just what I did!

This should be more than enough to get started with using bounds. There is a **TON** of different methods you can tap into with this Turbo macro and you can see even more of them if you `cmd + click` the `bounds` macro inside of Visual Studio Code!

> **Tip:** This works on any Turbo macro, not just `bounds`! 

## Final Notes

As usual, thanks for the read. Some final notes to keep in mind:

- Lean toward more visual or simple UIs to conserve space on smaller canvases. *(Look to old Game Boy/NES games for inspiration.)*  
- Simple does **not** mean bad.  
- Playtesting and gathering feedback on how players navigate your UI is valuable — it helps improve it to its best version.  
- `match` is your friend for smaller menus. For larger menus or scene transitions, consider a state machine approach.  
- Keep rendering logic and control input logic for menus **separate** — it keeps things cleaner and easier to debug.

