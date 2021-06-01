// Server logger
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        use colored::Colorize;
        println!("{}", format!(":: TRUST (🔑) :: {}", format!($($arg)*).white()).green());
    })
}

/// Format new user has joined message.
pub fn user_left_message(username: &str) -> String {
    format!("{} has left<NL>", &username)
}

/// Format message from user.
pub fn new_chat_message(username: &str, message: &str) -> String {
    format!("{}: {}<NL>", username, message)
}

/// Format new user has joined message.
pub fn user_joined_message(username: &str) -> String {
    format!("{} has joined<NL>", &username)
}

/// Format new user has joined message.
pub fn error_message() -> String {
    "ERROR<NL>".to_string()
}
