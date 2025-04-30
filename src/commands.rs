use crate::Message;
use crate::utils::{execute_command, format_output, get_brew_path, get_redis_cli_path};

use iced::Task;

pub fn start_redis() -> Task<Message> {
    brew_service_command("start", Message::StartCompleted)
}

pub fn stop_redis() -> Task<Message> {
    brew_service_command("stop", Message::StopCompleted)
}

pub fn ping_redis() -> Task<Message> {
    redis_cli_command(&["PING"], Message::PingCompleted)
}

fn brew_service_command(action: &'static str, success_msg: fn(String) -> Message) -> Task<Message> {
    match get_brew_path() {
        Ok(path) => Task::perform(
            async move {
                let result = execute_command(path, &["services", action, "redis"]).await;
                format_output(result)
            },
            success_msg,
        ),
        Err(e) => Task::perform(async { format_output(Err(e)) }, success_msg),
    }
}

fn redis_cli_command(
    args: &'static [&'static str],
    success_msg: fn(String) -> Message,
) -> Task<Message> {
    match get_redis_cli_path() {
        Ok(path) => Task::perform(
            async move {
                let result = execute_command(path, args).await;
                format_output(result)
            },
            success_msg,
        ),
        Err(e) => Task::perform(async { format_output(Err(e)) }, success_msg),
    }
}
