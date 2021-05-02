use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientSettings {
    locale: String,
    view_distance: i8,
    chat_mode: ChatMode,
    chat_colors: bool,
    skin_parts: u8,
    main_hand: MainHand,
}

#[derive(Deserialize)]
pub enum ChatMode {
    Enabled = 0,
    Commands = 1,
    Disabled = 2,
}

#[derive(Deserialize)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}
