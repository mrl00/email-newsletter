use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Result<Self, String> {
        let is_empty_or_whitespace = name.trim().is_empty();

        let is_too_long = name.graphemes(true).count() > 128;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        let forbidden_characters_in_name = name.chars().any(|c| forbidden_characters.contains(&c));

        if is_empty_or_whitespace || is_too_long || forbidden_characters_in_name {
            Err("Invalid name".to_string())
        } else {
            Ok(Self(name))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for SubscriberName {
    fn as_mut(&mut self) -> &mut str {
        self.0.as_mut()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
