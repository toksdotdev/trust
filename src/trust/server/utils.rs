/// Is the username args specified valid?
pub fn valid_username(username_arg: Option<&str>) -> bool {
    match username_arg {
        Some(username) if username.chars().count() > 0 && username.chars().count() < 20 => true,
        _ => false,
    }
}

/// Is the chatroom args specified valid?
pub fn valid_chatroom_name(room_name_arg: Option<&str>) -> bool {
    match room_name_arg {
        Some(room_name) if room_name.chars().count() > 0 && room_name.chars().count() < 20 => true,
        _ => false,
    }
}
