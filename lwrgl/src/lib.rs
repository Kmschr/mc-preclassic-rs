#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub extern crate glfw;
use glfw::{Glfw, MouseButton, Window, WindowEvent};

use self::glfw::{Action, Context, Key};

pub extern crate glu_sys;

use glu_sys::glu::glViewport;

use std::{
    collections::{HashSet, VecDeque},
    sync::mpsc::Receiver,
};

pub struct LWRGL {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    is_closed_requested: bool,
    last_mouse_x: Option<f64>,
    last_mouse_y: Option<f64>,
    keys_pressed: HashSet<Key>,
    mouse_button_events: VecDeque<(i32, bool)>,
    cur_button_event: Option<(i32, bool)>,
}

impl LWRGL {
    pub fn new(width: u32, height: u32) -> LWRGL {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(1, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Any,
        ));

        let (mut window, events) = glfw
            .create_window(width, height, "Game", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);
        glfw.set_swap_interval(glfw::SwapInterval::None);

        LWRGL {
            glfw,
            window,
            events,
            is_closed_requested: false,
            last_mouse_x: None,
            last_mouse_y: None,
            keys_pressed: HashSet::new(),
            mouse_button_events: VecDeque::new(),
            cur_button_event: None,
        }
    }

    pub fn get_display_width(&self) -> i32 {
        self.window.get_size().0
    }

    pub fn get_display_height(&self) -> i32 {
        self.window.get_size().1
    }

    pub fn update(&mut self) {
        self.process_events();
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    glViewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(key, _, Action::Press, _) => {
                    self.keys_pressed.insert(key);
                }
                glfw::WindowEvent::Key(key, _, Action::Release, _) => {
                    self.keys_pressed.remove(&key);
                }
                glfw::WindowEvent::MouseButton(button, Action::Press, _) => {
                    let button = match button {
                        MouseButton::Button1 => 0,
                        MouseButton::Button2 => 1,
                        MouseButton::Button3 => 2,
                        MouseButton::Button4 => 3,
                        MouseButton::Button5 => 4,
                        MouseButton::Button6 => 5,
                        MouseButton::Button7 => 6,
                        MouseButton::Button8 => 7,
                    };
                    self.mouse_button_events.push_back((button, true));
                }
                glfw::WindowEvent::MouseButton(button, Action::Release, _) => {
                    let button = match button {
                        MouseButton::Button1 => 0,
                        MouseButton::Button2 => 1,
                        MouseButton::Button3 => 2,
                        MouseButton::Button4 => 3,
                        MouseButton::Button5 => 4,
                        MouseButton::Button6 => 5,
                        MouseButton::Button7 => 6,
                        MouseButton::Button8 => 7,
                    };
                    self.mouse_button_events.push_back((button, false));
                }
                _ => {}
            }
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_close_requested(&self) -> bool {
        self.is_closed_requested
    }

    pub fn grab_mouse(&mut self) {
        self.window.focus();
        self.window.set_cursor_mode(glfw::CursorMode::Disabled);
    }

    pub fn mouse_dx(&mut self) -> i32 {
        let cursor_pos: (f64, f64) = self.window.get_cursor_pos();
        let ret = if let Some(mouse_x) = self.last_mouse_x {
            (cursor_pos.0 - mouse_x) as i32
        } else {
            0
        };
        self.last_mouse_x = Some(cursor_pos.0);
        ret
    }

    pub fn mouse_dy(&mut self) -> i32 {
        let cursor_pos = self.window.get_cursor_pos();
        let ret = if let Some(mouse_y) = self.last_mouse_y {
            (cursor_pos.1 - mouse_y) as i32
        } else {
            0
        };
        self.last_mouse_y = Some(cursor_pos.1);
        ret
    }

    pub fn mouse_next(&mut self) -> bool {
        if self.mouse_button_events.is_empty() {
            return false;
        }

        self.cur_button_event = self.mouse_button_events.pop_front();

        true
    }

    pub fn mouse_event_button(&self) -> i32 {
        if let Some(event) = self.cur_button_event {
            event.0
        } else {
            -1
        }
    }

    pub fn mouse_event_button_state(&self) -> bool {
        if let Some(event) = self.cur_button_event {
            event.1
        } else {
            false
        }
    }
}
