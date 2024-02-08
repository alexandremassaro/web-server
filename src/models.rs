use serde::{Serialize, Deserialize};
use mongodb::bson::{DateTime, oid::ObjectId};

#[derive(Serialize, Deserialize)]
struct  Message {
    _id: Option<ObjectId>,
    contents: Vec<MessageContent>,
    sender: MessageSender,
    conversation: ObjectId,
    created_at: DateTime,
}

#[derive(Serialize, Deserialize)]
struct MessageContent {
    message_type: MessageType,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    test_id: Option<ObjectId>,
    test: Option<String>,
    test_date: Option<DateTime>
}

#[derive(Serialize, Deserialize)]
enum MessageType {
    Text(String),
}

#[derive(Serialize, Deserialize)]
enum MessageSender {
    TextService(TextService),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub username: String,
    pub nickname: Option<String>,
    pub created_at: Option<DateTime>,
}

impl Default for User {
    fn default() -> Self {
        let default_name = default_name();
        User { 
            _id: None, 
            username: default_name.clone(),
            nickname: Some(default_name.clone()),
            created_at: Some(DateTime::now()),
        }
    }
}

impl From<&str> for User {
    fn from(name: &str) -> Self {
        Self {
            _id: None,
            username: name.to_string(),
            nickname: Some(name.to_string()),
            created_at: Some(DateTime::now())
        }
    }
}

impl From<&Self> for User {
    fn from(value: &Self) -> Self {
        let nickname = match &value.nickname {
            Some(nickname) => nickname.to_string(),
            None => value.username.to_string(),
        };
        Self {
            _id: None,
            username: value.username.to_owned(),
            nickname: Some(nickname),
            created_at: Some(DateTime::now()),
        }
    }
}

fn default_name() -> String {
    String::from(format!("{}{}", DateTime::now().timestamp_millis(), rand::random::<u16>()))
}

// impl User {
//     pub fn new(username: &str, nickname: Option<&str>) -> User {
//         let nickname = match nickname {
//             Some(nickname) => nickname,
//             None => username,
//         };
//         User {
//             _id: None,
//             username: String::from(username),
//             nickname: String::from(nickname),
//             created_at: DateTime::now(),
//         }
//     }
// }

#[derive(Serialize, Deserialize)]
struct Conversation {
    _id: ObjectId,
    conversation_type: ConversationType,
    participants: Vec<ConversationParticipant>,
    created_at: DateTime,

}

#[derive(Serialize, Deserialize)]
enum ConversationType {
    Single,
    GroupPrivate,
    GroupPublic,
}

#[derive(Serialize, Deserialize)]
enum ConversationParticipant {
    TextService(TextService),
}

#[derive(Serialize, Deserialize)]
struct TextService {
    _id: ObjectId,
    user: ObjectId,
    service_type: TextServiceType,
}

#[derive(Serialize, Deserialize)]
enum TextServiceType {
    DonaCarlota(String),
    Whatsapp(String),
    Sms(String),
    Email(String),
    Instagram(String),
    Facebook(String),
}