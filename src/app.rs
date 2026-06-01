use std::collections::VecDeque;

use tyche::dice::{roller::FastRand};
use tyche::expr::Describe;
use tyche::Expr;

use iced::{Event, keyboard, Task};
use iced::widget::operation::{self, RelativeOffset};

use crate::message::Message;
use crate::profile::Profile;
use crate::ui::RESULTS_SCROLL;

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

    profile: Profile,
}

impl Default for App {
    fn default() -> Self {
        let s = Self {
            roller: FastRand::default(),
            ip_left: "4d6rr1k3 ".to_string(),
            ip_right: String::default(),
            results: VecDeque::default(),
            history_size: 100,
            err_string: None,
            profile: Profile::default(),
        };  

        //Debug section
        println!("{:#?}", s.profile);
        return s
    } 
}

impl App {
    fn command_string(&self) -> String {
        self.ip_left.clone() + &(self.ip_right)
    }

    pub fn prompt_parts(&self) -> (&str, &str) {
        (&self.ip_left, &self.ip_right)
    } 

    fn roll(&mut self) -> Task<Message> {
        let expr: Expr = match self.command_string().parse() {
            Ok(v) => {
                self.err_string = None;
                v
            }
            Err(error) => {
                self.err_string = Some(error.to_string());
                return Task::none();
            }
        };

        let res  = match expr.eval(&mut self.roller) {
            Ok(r) => r,
            Err(error) => {
                self.ip_left = error.to_string();
                self.ip_right.clear();
                return Task::none();   
            }
        };

        let value = match res.calc()  {
            Ok(v) => v,
            Err(error) => {
                self.ip_left = error.to_string();
                self.ip_right.clear();
                return Task::none();   
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

        operation::snap_to(
            RESULTS_SCROLL.clone(),
            RelativeOffset { x: 0.0, y: 1.0}
        )
    }

    pub fn get_error(&self) -> &Option<String> {
        &self.err_string
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StrPadPressed(s) => {
                self.update_strpad_pressed(&s)
            }
            Message::EventOccurred(Event::Keyboard(event)) => {
                match KeyPress::from_keyboard_event(event)
                {
                    Some(kp) => self.update_keyboard_event(kp),
                    None => Task::none()
                }
            }
            _ => Task::none(),
        }
    }

    fn update_strpad_pressed(&mut self, s: &str) -> Task<Message>{
        match s {
            "ROLL" => self.roll(),
            "⌫"    => self.backspace(),
            "CLR"  => self.clear_prompt(),
            "←"    =>self.cursor_left(),
            "→"    => self.cursor_right(),
            _      => {
                    self.ip_left.push_str(s);
                    Task::none()
                }
        }
    }

    fn update_keyboard_event(&mut self, key_press: KeyPress) -> Task<Message>
    {
        match key_press.key.as_ref() {
            keyboard::Key::Character(_) => {
                if let Some(text) = key_press.text {
                    self.ip_left.push_str(text.as_str());
                }
                Task::none()
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

            _ => Task::none()
        }
    }

    fn backspace(&mut self) -> Task<Message> {
        self.ip_left.pop();
        Task::none()
    }

    fn clear_prompt(&mut self) -> Task<Message> {
        self.ip_left.clear();
        self.ip_right.clear();
        Task::none()
    }

    fn cursor_left(&mut self) -> Task<Message> {
        if let Some(ch) = self.ip_left.pop() {
            self.ip_right.insert(0, ch)
        }
        Task::none()
    }

    fn cursor_right(&mut self) -> Task<Message> {
        if !self.ip_right.is_empty() {
            let ch = self.ip_right.remove(0);
            self.ip_left.push(ch);
        }
        Task::none()
    }

    pub fn get_current_profile(&self) -> Option<&Profile> {
        Some(&self.profile)
    }
}