mod utils;

use iced::widget::{button, column, row, text};
use iced::{Subscription, Task, window};

use utils::{execute_command, format_output, get_brew_path};

#[derive(Debug, Clone)]
enum Message {
    StartPressed,
    StopPressed,
    PingPressed,
    WindowClosed,
}

struct Redock {
    redis_status: String,
    logs: Vec<String>,
}

impl Redock {
    fn new() -> Self {
        let mut redock = Redock {
            redis_status: "● running".to_string(),
            logs: vec![],
        };
        Redock::init_redis(&mut redock);
        redock
    }

    fn init_redis(&mut self) {
        let result = execute_command(get_brew_path(), &["services", "start", "redis"]);
        let formatted = format_output(result);
        self.logs.push(formatted);
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

    fn update(&mut self, message: Message) {
        match message {
            Message::StartPressed => {
                let result = execute_command(get_brew_path(), &["services", "start", "redis"]);
                let msg = format_output(result);
                self.logs.push(msg);
                self.redis_status = "● running".to_string();
            }
            Message::StopPressed => {
                let result = execute_command(get_brew_path(), &["services", "stop", "redis"]);
                let msg = format_output(result);
                self.logs.push(msg);

                self.redis_status = "○ stopped".to_string();
            }
            Message::PingPressed => {
                let output = execute_command("redis-cli", &["PING"]);
                let formatted = format_output(output);
                self.logs.push(format!("PING: {:?}", formatted));
                if self.logs.len() > 5 {
                    self.logs.remove(0);
                }
            }
            Message::WindowClosed => {
                self.logs.push("Stopping redis server".to_string());
                let result = execute_command(get_brew_path(), &["services", "stop", "redis"]);
                let msg = format_output(result);
                self.logs.push(msg);

                std::process::exit(0);
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
        .run_with(|| (Redock::new(), Task::none()))
}
