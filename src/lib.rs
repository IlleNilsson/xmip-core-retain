#![forbid(unsafe_code)]

use std::time::Duration;

/// What retention does with data of a given age. Two actions, and only two:
/// **Xmip retains and archives, it does not delete** (ADR-0040). Data is kept
/// live while young and archived once it passes its retention window; what then
/// becomes of the archive is the archive owner's decision, not Xmip's. Xmip gets
/// data, transforms it, waits for business decisions, and sends it — deleting it
/// is no part of that.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetentionAction {
    Keep,
    Archive,
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
