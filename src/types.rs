use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::utils::naive_date_time_from_str;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Service,
    Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TextObjectType {
    BankCard,
    Blockquote,
    Bold,
    BotCommand,
    Cashtag,
    Code,
    CustomEmoji,
    Email,
    Hashtag,
    Italic,
    Link,
    Mention,
    MentionName,
    Phone,
    Plain,
    Pre,
    Spoiler,
    Strikethrough,
    TextLink,
    Underline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextObject {
    pub text: String,
    pub r#type: TextObjectType,
    pub user_id: Option<i64>,
    pub href: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Action {
    BoostApply,
    CreateChannel,
    DeleteGroupPhoto,
    EditGroupPhoto,
    EditGroupTitle,
    GiveawayLaunch,
    GroupCall,
    GroupCallScheduled,
    InviteMembers,
    InviteToGroupCall,
    JoinGroupByLink,
    MigrateFromGroup,
    PinMessage,
    RemoveMembers,
    ScoreInGame,
    SetMessagesTtl,
    TopicCreated,
    TopicEdit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextEntity {
    String(String),
    Object(TextObject),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "service")]
#[skip_serializing_none]
pub struct ServiceMessage {
    pub id: i64,
    #[serde(deserialize_with = "naive_date_time_from_str")]
    pub date: NaiveDateTime,
    pub date_unixtime: String,
    pub actor: Option<String>,
    pub actor_id: String,
    pub action: Action,
    pub text: String,
    pub text_entities: Vec<TextEntity>,
    pub message_id: Option<i64>,
    pub title: Option<String>,
    pub photo: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub members: Option<Vec<Option<String>>>,
    pub schedule_date: Option<i64>,
    pub boosts: Option<i64>,
    pub duration: Option<i64>,
    pub new_icon_emoji_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineBotButton {
    pub r#type: String,
    pub text: String,
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub chosen: bool,
    pub text: String,
    pub voters: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub question: String,
    pub closed: bool,
    pub total_voters: i64,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Text {
    Plain(String),
    Rich(Vec<TextEntity>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Message {
    pub id: i64,
    #[serde(deserialize_with = "naive_date_time_from_str")]
    pub date: NaiveDateTime,
    pub date_unixtime: String,
    pub edited: Option<String>,
    pub edited_unixtime: Option<String>,
    pub from: Option<String>,
    pub from_id: String,
    pub text: Text,
    pub text_entities: Vec<TextEntity>,
    pub forwared_from: Option<String>,
    pub saved_from: Option<String>,
    pub reply_to_message_id: Option<i64>,
    pub reply_to_peer_id: Option<String>,
    pub file: Option<String>,
    pub thumbnail: Option<String>,
    pub sticker_emoji: Option<String>,
    pub via_bot: Option<String>,
    pub game_title: Option<String>,
    pub game_description: Option<String>,
    pub game_link: Option<String>,
    pub author: Option<String>,
    pub media_type: Option<String>,
    pub mime_type: Option<String>,
    pub duration_seconds: Option<i64>,
    pub photo: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub inline_bot_buttons: Option<Vec<Vec<InlineBotButton>>>,
    pub poll: Option<Poll>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatHistoryType {
    PrivateSupergroup,
    PublicSupergroup,
    PublicChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageItem {
    Message(Box<Message>),
    Service(Box<ServiceMessage>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistory {
    pub name: String,
    pub r#type: ChatHistoryType,
    pub id: i64,
    pub messages: Vec<MessageItem>,
}
