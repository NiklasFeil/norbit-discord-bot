use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DiscordBotInformation {
    application_id: String,
    public_key: String,
    token: String
}

pub struct DiscordBotInformationHandler {
    bot_info: DiscordBotInformation
} 

impl DiscordBotInformationHandler {
    pub fn new(file_path: &str) -> DiscordBotInformationHandler {
        let sensitive_data: String = fs::read_to_string(file_path).expect("Could not find or read JSON file");
        let bot_info = serde_json::from_str(&sensitive_data).expect("JSON file exists, but could not be resolved. Maybe it contains syntax errors or does not contain the correct format?");
        Self {
            bot_info
        }
    }

    pub fn get_bot_token(self) -> String {
        self.bot_info.token
    }
}
