#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate rust_gol;

use std::path::Path;
use piston_window::{EventLoop, PistonWindow, UpdateEvent, WindowSettings};
use conrod::{color, widget, Borderable, Colorable, Positionable, Widget, Labelable, Sizeable};
use rust_gol::{GameBoard, Generation, WithLiveCells, Position};
mod widgets;
use widgets::StandardButton;

struct GameState {
    board: GameBoard,
    running: bool,
    gps: f64,
    current_time: f64,
}

fn main() {
    const WIDTH: u32 = 1100;
    const HEIGHT: u32 = 560;

    let opengl = piston_window::OpenGL::V3_2;

    let mut window: PistonWindow =
    WindowSettings::new("Rust-GoL!", [WIDTH, HEIGHT])
        .opengl(opengl).exit_on_esc(true).vsync(true).build().unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
//    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
//    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(Path::new("/usr/share/fonts/noto/NotoSans-Regular.ttf")).unwrap();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache =
    conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    // Identifiers used for instantiating our widgets.
    widget_ids! {struct Ids { canvas, controls, game_display, start_stop_button, step_button, faster_button, slower_button, board }}
    let ids = Ids::new(ui.widget_id_generator());

    window.set_ups(60);

    let initial_generation_builder = Generation::build()
        .add(0, 0)
        .add(0, 1)
        .add(0, -1);
    let initial_generation_builder = initial_generation_builder
        .add(11, 11)
        .add(12, 11)
        .add(12, 13)
        .add(14, 12)
        .add(15, 11)
        .add(16, 11)
        .add(17, 11);
    let initial_generation_builder = initial_generation_builder
        .add(2, -1)
        .add(3, -1)
        .add(1, -2)
        .add(2, -2)
        .add(2, -3);
    let initial_generation = initial_generation_builder.build();

    let mut game_state = GameState {
        board: GameBoard::initialize_with(initial_generation),
        running: false,
        gps: 4.0,
        current_time: 0.0
    };

    // Poll events from the window.
    while let Some(event) = window.next() {

        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        // `Update` the widgets.
        event.update(|args| {
            let ui = &mut ui.set_widgets();

            if game_state.running {
                game_state.current_time += args.dt;
                let generation_time = 1.0/game_state.gps;
                while game_state.current_time > generation_time {
                    game_state.current_time -= generation_time;
                    game_state.board.advance_time();
                }
            }

            // Create a background canvas upon which we'll place the button.
            widget::Canvas::new()
                .pad(0.0)
                .flow_right(&[
                    (ids.controls, widget::Canvas::new().length(300.0)),
                    (ids.game_display, widget::Canvas::new().color(color::LIGHT_GRAY))
                ])
                .set(ids.canvas, ui);

            let start_stop_label = if game_state.running {
                "Stop"
            } else {
                "Start"
            };
            if widget::Button::new()
                .std_size()
                .top_left_with_margin_on(ids.controls, 40.0)
                .label(start_stop_label)
                .set(ids.start_stop_button, ui)
                .was_clicked()
                {
                    println!("Changing running to {:}...", !game_state.running);
                    game_state.running = !game_state.running;
                }

            if game_state.running {
                if widget::Button::new()
                    .std_height()
                    .w(50.0)
                    .down_from(ids.start_stop_button, 20.0)
                    .label("+")
                    .set(ids.faster_button, ui)
                    .was_clicked()
                    {
                        game_state.gps *= 1.5;
                    }

                if widget::Button::new()
                    .std_height()
                    .w(50.0)
                    .right_from(ids.faster_button, 50.0)
                    .label("-")
                    .set(ids.slower_button, ui)
                    .was_clicked()
                    {
                        game_state.gps /= 1.5;
                    }
            }
            else {
                if widget::Button::new()
                    .std_size()
                    .down_from(ids.start_stop_button, 20.0)
                    .label("One step")
                    .set(ids.step_button, ui)
                    .was_clicked()
                    {
                        game_state.board.advance_time();
                    }
            }

            // A demonstration using widget_matrix to easily draw a matrix of any kind of widget.
            let (cols, rows) = (100, 100);
            let mut board = widget::Matrix::new(cols, rows)
                .middle_of(ids.game_display)
                .w_h(600.0, 600.0)
//                .color(color::WHITE)
                .set(ids.board, ui);

            let x_zero: i64 = (cols/2) as i64;
            let y_zero: i64 = (rows/2) as i64;

            // The `Matrix` widget returns an `Elements`, which can be used similar to an `Iterator`.
            while let Some(elem) = board.next(ui) {
                let (col, row) = (elem.col as i64, elem.row as i64);
                let (x, y) = (col - x_zero, -(row - y_zero));
                let position = Position::new(x, y);

                if game_state.board.is_alive(&position) {
                    let square = widget::Rectangle::fill_with([10.0, 10.0], color::BLACK);
                    elem.set(square, ui);
                }
            }
        });

        // Draw the `Ui` if it has changed.
        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }
}
