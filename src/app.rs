use std::collections::VecDeque;

use tyche::dice::{roller::FastRand};
use tyche::expr::Describe;
use tyche::Expr;

use iced::{Event, keyboard};

use crate::message::Message;

#[derive(Default)]
pub struct RollResult {
    pub value: i32,
    pub text: String,
}

//Simplified version of iced KeyPressEvent to pass to diffrent context
#[derive(Debug, Clone)]
struct KeyPress {
    pub key: keyboard::Key,
    pub text: Option<String>,
    pub modifiers: keyboard::Modifiers,
}

impl KeyPress {
    pub fn from_keyboard_event(event: keyboard::Event) -> Option<Self> {
        match event {
            keyboard::Event::KeyPressed {
                    key,
                    text,
                    modifiers,
                    ..
                }
             => Some(Self {
                key,
                text: text.map(|s| s.to_string()),
                modifiers,
            }),

            _ => None,
        }
    }
}

pub struct App {
    roller: tyche::dice::roller::FastRand,
    ip_left: String,
    ip_right: String,
    pub results: VecDeque<RollResult>,
    history_size: usize,
    err_string: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            roller: FastRand::default(),
            ip_left: "4d6rr1k3 ".to_string(),
            ip_right: String::default(),
            results: VecDeque::default(),
            history_size: 100,
            err_string: None
        }   
    } 
}

impl App {
    fn command_string(&self) -> String {
        self.ip_left.clone() + &(self.ip_right)
    }

    pub fn prompt_parts(&self) -> (&str, &str) {
        (&self.ip_left, &self.ip_right)
    } 

    fn roll(&mut self) {
        let expr: Expr = match self.command_string().parse() {
            Ok(v) => {
                self.err_string = None;
                v
            }
            Err(error) => {
                self.err_string = Some(error.to_string());
                return;
            }
        };

        let res  = match expr.eval(&mut self.roller) {
            Ok(r) => r,
            Err(error) => {
                self.ip_left = error.to_string();
                self.ip_right.clear();
                return;   
            }
        };

        let value = match res.calc()  {
            Ok(v) => v,
            Err(error) => {
                self.ip_left = error.to_string();
                self.ip_right.clear();
                return;   
            }
        };

        self.results.push_back(
            RollResult {
                value: value,
                text: res.describe(Some(100)),
            }
        );

        while self.results.len() > self.history_size {
            self.results.pop_front();
        }
        
        //self.ip_left.clear();
        //self.ip_right.clear();
    }

    pub fn get_error(&self) -> &Option<String> {
        &self.err_string
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::RollPress => self.roll(),
            Message::StrPadPressed(s) => self.update_strpad_pressed(&s),
            Message::EventOccurred(Event::Keyboard(event)) => {
                if let Some(kp) = KeyPress::from_keyboard_event(event){
                    self.update_keyboard_event(kp);
                }
            }
            _ => {}
        }
    }

    fn update_strpad_pressed(&mut self, s: &str) {
        match s {
            "ROLL" => self.roll(),
            "⌫"    => self.backspace(),
            "CLR"  => self.clear_prompt(),
            "←"    =>self.cursor_left(),
            "→"    => self.cursor_right(),
            _      => self.ip_left.push_str(s),
        }
    }

    fn update_keyboard_event(&mut self, key_press: KeyPress) 
    {
        match key_press.key.as_ref() {
            keyboard::Key::Character(_) => {
                if let Some(text) = key_press.text {
                    self.ip_left.push_str(text.as_str());
                }
            }

            keyboard::Key::Named(
                keyboard::key::Named::ArrowLeft
            ) => self.cursor_left(),

            keyboard::Key::Named(
                keyboard::key::Named::ArrowRight
            ) => self.cursor_right(),

            keyboard::Key::Named(
                keyboard::key::Named::Backspace
            ) => self.backspace(),

            keyboard::Key::Named(
                keyboard::key::Named::Enter
            ) => self.roll(),

            _ => {}
        }
    }

    fn backspace(&mut self) {
        self.ip_left.pop();
    }

    fn clear_prompt(&mut self){
        self.ip_left.clear();
        self.ip_right.clear();
    }

    fn cursor_left(&mut self) {
        if let Some(ch) = self.ip_left.pop() {
            self.ip_right.insert(0, ch)
        }
    }

    fn cursor_right(&mut self) {
        if !self.ip_right.is_empty() {
            let ch = self.ip_right.remove(0);
            self.ip_left.push(ch);
        }
    }
}