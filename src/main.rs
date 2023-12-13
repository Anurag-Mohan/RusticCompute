use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use druid::widget::{Flex, SizedBox, TextBox, Button};
use druid::widget::BackgroundBrush;
use druid::piet::Color;

#[derive(Clone, Data, Lens)]
struct AppState {
    input: String,
    result: String,
}

fn main() {
    
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("RusticCompute"))
        .window_size((300.0, 420.0));
       
    
    let initial_state = AppState {
        input: String::new(),
        result: String::new(),
    };

    
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(SizedBox::new(TextBox::new()
            .with_placeholder("Enter expression...")
            .expand_width()
            .lens(AppState::input)))
        .with_child(build_button_grid())
        .with_spacer(5.0)
        .with_child(Button::new("Calculate")
            .on_click(|_, data: &mut AppState, _| {
                calculate_result(&mut data.input, &mut data.result);
            }))
            .with_spacer(5.0)
        .with_child(SizedBox::new(TextBox::new()
            .with_placeholder("Result")
            .lens(AppState::result)))
            .padding(10.0)
}

fn build_button_grid() -> impl Widget<AppState> {
    Flex::column()
        .with_spacer(5.0)
        .with_child(build_backspace_button())
        .with_spacer(5.0)
        .with_child(build_button_row("789/"))
        .with_child(build_button_row("456*"))
        .with_child(build_button_row("123-"))
        .with_child(build_button_row("%0+"))
        
}

fn build_backspace_button() -> impl Widget<AppState> {
    Flex::row().with_spacer(5.0)
       
        .with_child(Button::new("⌫")
            .on_click(|_, data: &mut AppState, _| {
                data.input.pop();
            })
            .fix_size(50.0, 50.0))
}
fn build_button_row(symbols: &str) -> impl Widget<AppState> {
    let mut child = Flex::row().with_spacer(5.0);

    
    for c in symbols.chars() {
        
        let button_color = Color::rgb8(200, 50, 250);

        child = child.with_child(
            Button::new(c.to_string())
                .on_click(move |_, data: &mut AppState, _| {
                    data.input.push(c);
                })
                .fix_size(50.0, 50.0)
                .background(BackgroundBrush::Color(button_color)),
        );
    }

    child
}

fn calculate_result(input: &mut String, result: &mut String) {
    
    match eval::eval(input) {
        Ok(value) => *result = value.to_string(),
        Err(e) => *result = format!("Error: {}", e),
    }
    input.clear();
}
