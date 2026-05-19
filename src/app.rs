use std::collections::VecDeque;

use tyche::dice::roller::FastRand;
use tyche::expr::Describe;
use tyche::Expr;

use iced::{Event, keyboard};

use crate::message::Message;

pub struct RollResult {
    pub value: i32,
    pub text: String,
}

impl Default for RollResult {
    fn default() -> Self {
        Self {
            value: 0,
            text: "".to_string(),
        }    
    }
}

pub struct App {
    roller: tyche::dice::roller::FastRand,
    ip_left: String,
    ip_right: String,
    pub results: VecDeque<RollResult>,
    history_size: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            roller: FastRand::default(),
            ip_left: "4d6rr1k3 ".to_string(),
            ip_right: String::default(),
            results: VecDeque::default(),
            history_size: 100
        }   
    } 
}

impl App {
    fn get_input_string(&mut self) -> String {
        let compossed = self.ip_left.clone() + &(self.ip_right);
        return compossed;
    }

    fn roll(&mut self) {
        let expr: Expr = match self.get_input_string().parse() {
            Ok(v) => v,
            Err(error) => {
                self.ip_left = error.to_string();
                self.ip_right.clear();
                return;
            }
        };

        let res  = expr.eval(&mut self.roller).unwrap();

        self.results.push_back(
            RollResult {
                value: res.calc().unwrap(),
                text: res.describe(Some(100))
            }
        );

        while self.results.len() > self.history_size {
            self.results.pop_front();
        }
        
        //self.ip_left.clear();
        //self.ip_right.clear();
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::RollPress => self.roll(),
            Message::StrPadPressed(s) => self.update_strpad_pressed(&s),
            Message::InputChanged(s) => {
                self.ip_left = s;
            },
            Message::EventOccurred(Event::Keyboard(
                keyboard::Event::KeyPressed { key, .. }
            )) => {
                match key.as_ref() {
                    keyboard::Key::Character(s) => {
                        self.ip_left.push_str(s);
                    }

                    keyboard::Key::Named(
                        keyboard::key::Named::ArrowLeft
                    ) => {
                        if let Some(ch) = self.ip_left.pop() {
                            self.ip_right.insert(0, ch);
                        }
                    }

                    keyboard::Key::Named(
                        keyboard::key::Named::ArrowRight
                    ) => {
                        if !self.ip_right.is_empty() {
                            let ch = self.ip_right.remove(0);
                            self.ip_left.push(ch);
                        }
                    }

                    keyboard::Key::Named(
                        keyboard::key::Named::Backspace
                    ) => {
                        self.ip_left.pop();
                    }

                    keyboard::Key::Named(
                        keyboard::key::Named::Enter
                    ) => {
                        self.roll();
                    }

                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn update_strpad_pressed(&mut self, s: &str) {
        match s {
            "ROLL" => self.roll(),
            "⌫" => {
                self.ip_left.pop();
            }
            "CLR" => {
                self.ip_left.clear();
                self.ip_right.clear();
            }
            "←" =>{
                if let Some(ch) = self.ip_left.pop() {
                    self.ip_right.insert(0, ch)
                }
            }
            "→" => {
                if !self.ip_right.is_empty() {
                    let ch = self.ip_right.remove(0);
                    self.ip_left.push(ch);
                }
            }
            _ => self.ip_left.push_str(s),
        }
    }

    pub fn render_output_str(&self) -> String {
        let op: String = format!("{}|{}", self.ip_left, self.ip_right);
        return op;
    } 

}