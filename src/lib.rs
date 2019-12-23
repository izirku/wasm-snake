mod canvas;
mod direction;
mod snake;
mod utils;

use crate::canvas::Canvas;
use crate::direction::Direction;
use crate::snake::Snake;
use crate::utils::set_panic_hook;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; // needed for unchecked_ref()

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// [wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// [wasm_bindgen]
// pub fn greet() {
//     alert("Hello, wasm-snake!");
// }

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = Canvas::new("canvas", 20, 20);
    let snake = Rc::new(RefCell::new(Snake::new(20, 20)));

    snake.borrow().draw(&canvas);

    {
        // this will get captured by closure below...
        // TODO: figure out if RC count still kept after this scope collapses?
        let snake = snake.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => snake.borrow_mut().change_direction(Direction::Left),
                "ArrowRight" => snake.borrow_mut().change_direction(Direction::Right),
                "ArrowDown" => snake.borrow_mut().change_direction(Direction::Down),
                "ArrowUp" => snake.borrow_mut().change_direction(Direction::Up),
                // TODO: figure out what's the difference between {} and unit () here:
                // _ => (),
                _ => {}
            }
        }) as Box<dyn Fn(_)>);
        document.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<Canvas>, time: i32) {
        let closure = Closure::wrap(Box::new(move || {
            game_loop(snake.clone(), canvas.clone(), time);
            snake.borrow_mut().update();
            snake.borrow().draw(&canvas);
        }) as Box<dyn Fn()>);
        let _res = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                time,
            );
        closure.forget();
    }

    game_loop(snake, Rc::new(canvas), 100);
}
