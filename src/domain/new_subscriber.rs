use crate::domain::{SubscriberEmail, SubscriberName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
