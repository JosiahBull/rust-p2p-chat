use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::history::History;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Message,
    State,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    /// The type of message this is.
    pub message_type: MessageType,
    /// The data contained within the message, represented as a vector of bytes.
    pub data: Vec<u8>, //It would be better to use a borrowed value here, as vecs heap allocate
    /// The intended recipient of the message, a PeerId encoded as a string.
    pub addressee: Option<String>,
    /// The sender of this message, PeerId encoded as a String
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    /// The messaging history of the chat
    pub history: History<Message>,
    /// The usernames of everyone currently connected to the network.
    /// This is a mapping of `PeerId` to `String`. The `PeerId`'s are encoded
    /// as Strings.
    pub usernames: HashMap<String, String>,
}

impl State {
    /// Attempt to merge two states together.
    /// Note that if our history is up-to-date, then we will not accept this new history
    pub fn merge(&mut self, mut other: State) {
        //Merge usernames always!
        for (peer_id, username) in other.usernames.drain() {
            if !self.usernames.contains_key(&peer_id) {
                println!("{} has joined the chat!", &username);
                self.usernames.insert(peer_id, username);
            }
        }

        //Merge Messages
        //Note, we only want to merge messages in the event that we don't have any history
        //This prevents messages from being abused through spamming!
        if self.history.get_count() < 1 && other.history.get_count() > 1 {
            //Begin merging
            for message in other.history.get_all() {
                println!(
                    "{}: {}",
                    self.get_username(&message.source),
                    String::from_utf8_lossy(&message.data)
                );
                self.history.insert((*message).to_owned());
            }
        }
    }

    /// Attempt to collect the username of a user, if the user doesn't exist then the username
    /// defaults to `anon`.
    pub fn get_username(&self, usr: &String) -> String {
        self.usernames
            .get(usr)
            .unwrap_or(&String::from("anon"))
            .to_string()
    }
}