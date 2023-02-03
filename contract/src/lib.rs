use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::near_bindgen;
use near_sdk::serde::Serialize;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone)]
pub struct Message {
    content: String,
    owner_id: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

#[near_bindgen]
impl Chat {
    pub fn get_total_messages(&self) -> usize {
        return self.messages.len();
    }

    pub fn add_message(&mut self, message_content: String) {
        let account_id = env::signer_account_id();
        let message_object = Message {
            content: message_content,
            owner_id: account_id.to_string(),
        };
        self.messages.push(message_object)
    }

    pub fn get_messages(&self) -> Vec<Message> {
        return self.messages.clone();
    }
}

#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;

    use super::*;

    #[test]
    fn get_total_messages() {
        let contract = Chat::default();
        assert_eq!(contract.get_total_messages(), 0)
    }

    #[test]
    fn add_message() {
        let mut contract = Chat::default();
        contract.add_message("first message".to_string());
        assert_eq!(contract.get_total_messages(), 1)
    }
}
