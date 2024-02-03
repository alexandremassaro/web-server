use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct  Message {
    uuid: Uuid,
    contents: Vec<MessageContent>,
    sender: MessageSender,
    conversation: Conversation,
    created_at: String,
}

#[derive(Serialize, Deserialize)]
enum MessageContent {
    Text(String),
}

#[derive(Serialize, Deserialize)]
struct  MessageSender {
    text_service: TextService,
}

#[derive(Serialize, Deserialize)]
struct User {
    uuid: Uuid,
    username: String,
    nickname: String,
    created_at: String,
    conversations: Vec<Conversation>,
    text_services: Vec<TextService>,
}

#[derive(Serialize, Deserialize)]
struct Conversation {
    uuid: Uuid,
    conversation_type: ConversationType,
    participants: Vec<ConversationParticipant>,
    messages: Vec<Message>

}

#[derive(Serialize, Deserialize)]
enum ConversationType {
    Single,
    GroupPrivate,
    GroupPublic,
}

#[derive(Serialize, Deserialize)]
struct ConversationParticipant {
    text_service: TextService,
}

#[derive(Serialize, Deserialize)]
struct TextService {
    text_service_type: TextServiceType,
    user: User,
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