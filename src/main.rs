mod commands;
mod utils;

use std::sync::atomic::{AtomicBool, Ordering};

use iced::widget::{button, column, row, text};
use iced::{Subscription, Task, window};

use commands::{ping_redis, start_redis, stop_redis};

static REDIS_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
enum Message {
    StartPressed,
    StopPressed,
    PingPressed,
    WindowClosed,
    StartCompleted(String),
    StopCompleted(String),
    PingCompleted(String),
}

struct Redock {
    redis_status: String,
    logs: Vec<String>,
}

impl Redock {
    fn new() -> (Self, Task<Message>) {
        let redock = Redock {
            redis_status: "○ starting...".to_string(),
            logs: vec![],
        };
        let task = start_redis();

        (redock, task)
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _id| {
            if let iced::Event::Window(window::Event::CloseRequested) = event {
                Some(Message::WindowClosed)
            } else {
                None
            }
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartPressed => {
                self.redis_status = "○ starting...".to_string();
                start_redis()
            }
            Message::StopPressed => {
                self.redis_status = "○ stopping...".to_string();
                stop_redis()
            }
            Message::PingPressed => ping_redis(),
            Message::StartCompleted(output) => {
                self.redis_status = "● running".to_string();
                REDIS_RUNNING.store(true, Ordering::Release);
                self.logs.push(output);
                Task::none()
            }
            Message::StopCompleted(output) => {
                self.redis_status = "○ stopped".to_string();
                REDIS_RUNNING.store(false, Ordering::Release);
                self.logs.push(output);
                Task::none()
            }
            Message::PingCompleted(output) => {
                self.logs.push(format!("PING: {}", output));
                if self.logs.len() > 5 {
                    self.logs.remove(0);
                }
                Task::none()
            }
            Message::WindowClosed => {
                if REDIS_RUNNING.load(Ordering::Acquire) {
                    stop_redis()
                } else {
                    std::process::exit(0);
                }
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let logs = self
            .logs
            .iter()
            .fold(column![], |col, log| col.push(text(log).size(12)));

        column![
            text(format!("Redis: {}", self.redis_status)).size(20),
            row![
                button("START")
                    .on_press(Message::StartPressed)
                    .style(button::success)
                    .width(100f32)
                    .padding(5),
                button("STOP")
                    .on_press(Message::StopPressed)
                    .style(button::danger)
                    .width(100f32)
                    .padding(5),
                button("PING")
                    .on_press(Message::PingPressed)
                    .style(button::primary)
                    .width(100f32)
                    .padding(5),
            ]
            .spacing(20),
            text("Logs:").size(16),
            logs,
        ]
        .padding(20)
        .spacing(10)
        .into()
    }
}

pub fn main() -> Result<(), iced::Error> {
    iced::application("Redock - Redis Manager", Redock::update, Redock::view)
        .window(window::Settings {
            size: iced::Size {
                width: 400_f32,
                height: 300_f32,
            },
            min_size: Some(iced::Size {
                width: 350f32,
                height: 250_f32,
            }),
            resizable: false,
            ..window::Settings::default()
        })
        .exit_on_close_request(false)
        .subscription(Redock::subscription)
        .run_with(Redock::new)
}
