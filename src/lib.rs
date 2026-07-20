#![forbid(unsafe_code)]

use std::time::Duration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetentionAction {
    Keep,
    Archive,
    Delete,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetentionRule {
    pub data_type: String,
    pub age: Duration,
    pub action: RetentionAction,
}

pub trait RetentionPolicy: Send + Sync {
    fn action_for(&self, data_type: &str, age: Duration) -> RetentionAction;
}
