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

#[cfg(test)]
mod tests {
    use super::*;

    /// A policy of rules, the first matching data type deciding.
    struct Rules(Vec<RetentionRule>);

    impl RetentionPolicy for Rules {
        fn action_for(&self, data_type: &str, age: Duration) -> RetentionAction {
            self.0
                .iter()
                .find(|rule| rule.data_type == data_type && age >= rule.age)
                .map_or(RetentionAction::Keep, |rule| rule.action.clone())
        }
    }

    #[test]
    fn young_data_is_kept_and_aged_data_is_archived() {
        let policy = Rules(vec![RetentionRule {
            data_type: "json".to_string(),
            age: Duration::from_secs(90 * 86_400),
            action: RetentionAction::Archive,
        }]);
        assert_eq!(
            policy.action_for("json", Duration::from_secs(1)),
            RetentionAction::Keep
        );
        assert_eq!(
            policy.action_for("json", Duration::from_secs(91 * 86_400)),
            RetentionAction::Archive
        );
        assert_eq!(
            policy.action_for("xml", Duration::MAX),
            RetentionAction::Keep
        );
    }

    #[test]
    fn there_are_two_actions_and_no_third() {
        // ADR-0040: Xmip retains and archives; it does not delete. A match
        // over the enum is exhaustive with these two arms and no wildcard.
        for action in [RetentionAction::Keep, RetentionAction::Archive] {
            let name = match action {
                RetentionAction::Keep => "keep",
                RetentionAction::Archive => "archive",
            };
            assert!(!name.is_empty());
        }
    }
}
