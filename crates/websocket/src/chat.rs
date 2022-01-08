use serde::Deserialize;


/// This object represents a Telegram user or bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: u32,
    /// User‘s or bot’s name.
    pub name: String,
    /// User‘s or bot’s user name.
    pub username: Option<String>,
    /// True, if this user is a bot.
    pub is_bot: bool,
    /// IETF language tag of the user's language
    pub language_code: Option<String>,
}


/// This object represents a group.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct Group {
    /// Unique identifier for this chat.
    pub id: usize,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: bool,
    /// Invite link for this group, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    pub invite_link: Option<String>,
}

/// This object represents a supergroup.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct Supergroup {
    /// Unique identifier for this chat.
    pub id: usize,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// Username for supergroup.
    pub username: Option<String>,
    /// Invite link for this supergroup, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    pub invite_link: Option<String>,
}


/// This object represents a channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct Channel {
    /// Unique identifier for this chat.
    pub id: usize,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// Username for channel.
    pub username: Option<String>,
    /// Invite link for this channel, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    pub invite_link: Option<String>,
}