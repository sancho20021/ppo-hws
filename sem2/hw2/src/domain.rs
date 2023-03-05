use async_trait::async_trait;
use chrono::prelude::*;
use cqrs_es::{Aggregate, DomainEvent};
use serde::{Deserialize, Serialize};
use std::result::Result::*;

pub type UtcDateTime = DateTime<Utc>;
pub type CardId = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardInfo {
    issued: UtcDateTime,
    until: UtcDateTime,
    in_club: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Card {
    Existent { info: CardInfo },
    Unexistent,
}

impl Default for Card {
    fn default() -> Self {
        Card::Unexistent
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardEvent {
    CardIssued {
        issued: UtcDateTime,
        until: UtcDateTime,
    },
    CardExtended {
        until: UtcDateTime,
    },
    CardEntered {
        time: UtcDateTime,
    },
    CardLeft {
        time: UtcDateTime,
    },
}

impl DomainEvent for CardEvent {
    fn event_type(&self) -> String {
        match self {
            CardEvent::CardIssued { .. } => "CardIssued",
            CardEvent::CardExtended { .. } => "CardExtended",
            CardEvent::CardEntered { .. } => "CardEntered",
            CardEvent::CardLeft { .. } => "CardEntered",
        }
        .to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardCommand {
    IssueCard {
        issued: UtcDateTime,
        until: UtcDateTime,
    },
    ExtendCard {
        until: UtcDateTime,
    },
    Enter {
        time: UtcDateTime,
    },
    Leave {
        time: UtcDateTime,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CardError {
    #[error("card not found")]
    CardNotFound,
    #[error("issued after expired")]
    UntilBeforeIssued,
    #[error("invalid extending date")]
    InvalidExtendingDate,
    #[error("card already exists")]
    CardAlreadyExists,
    #[error("card expired")]
    CardExpired,
    #[error("already in club")]
    CardAlreadyInClub,
    #[error("already left")]
    CardAlreadyLeft,
}

impl Card {
    fn expect_existent<F: FnOnce(&CardInfo) -> Result<Vec<CardEvent>, CardError>>(
        &self,
        f: F,
    ) -> Result<Vec<CardEvent>, CardError> {
        match self {
            Card::Existent { info } => f(info),
            Card::Unexistent => Err(CardError::CardNotFound),
        }
    }

    fn expect_existent_mut<F: FnOnce(&mut CardInfo)>(&mut self, f: F) {
        match self {
            Card::Existent { info } => f(info),
            Card::Unexistent => panic!("card must exist"),
        }
    }
}

#[async_trait]
impl Aggregate for Card {
    type Command = CardCommand;
    type Event = CardEvent;
    type Error = CardError;
    type Services = ();

    fn aggregate_type() -> String {
        "Card".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            CardCommand::IssueCard { issued, until } => match self {
                Card::Existent { .. } => Err(CardError::CardAlreadyExists),
                Card::Unexistent => {
                    if until < issued {
                        return Err(CardError::UntilBeforeIssued);
                    }
                    Ok(vec![CardEvent::CardIssued { issued, until }])
                }
            },
            CardCommand::ExtendCard { until } => self.expect_existent(|info| {
                if until < info.until {
                    Err(CardError::InvalidExtendingDate)
                } else {
                    Ok(vec![CardEvent::CardExtended { until }])
                }
            }),
            CardCommand::Enter { time } => self.expect_existent(|info| {
                if info.until < time {
                    Err(CardError::CardExpired)
                } else if info.in_club {
                    Err(CardError::CardAlreadyInClub)
                } else {
                    Ok(vec![CardEvent::CardEntered { time }])
                }
            }),
            CardCommand::Leave { time } => self.expect_existent(|info| {
                if !info.in_club {
                    Err(CardError::CardAlreadyLeft)
                } else {
                    Ok(vec![CardEvent::CardLeft { time }])
                }
            }),
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            CardEvent::CardIssued { issued, until } => {
                *self = Card::Existent {
                    info: CardInfo {
                        issued,
                        until,
                        in_club: false,
                    },
                }
            }
            CardEvent::CardExtended { until, .. } => self.expect_existent_mut(|card_info| {
                card_info.until = until;
            }),
            CardEvent::CardEntered { .. } => self.expect_existent_mut(|card_info| {
                card_info.in_club = true;
            }),
            CardEvent::CardLeft { .. } => self.expect_existent_mut(|card_info| {
                card_info.in_club = false;
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use cqrs_es::test::TestFramework;

    type CardTestFramework = TestFramework<Card>;

    fn datetime(millis: i64) -> UtcDateTime {
        Utc.timestamp_millis_opt(millis).unwrap()
    }

    #[test]
    fn test_card_issued_ok() {
        let issued = datetime(100);
        let until = datetime(200);
        CardTestFramework::with(())
            .given_no_previous_events()
            .when(CardCommand::IssueCard { issued, until })
            .then_expect_events(vec![CardEvent::CardIssued { issued, until }]);
    }

    #[test]
    fn test_already_issed() {
        let card_issued = CardEvent::CardIssued {
            issued: datetime(0),
            until: datetime(100),
        };
        CardTestFramework::with(())
            .given(vec![card_issued])
            .when(CardCommand::IssueCard {
                issued: datetime(50),
                until: datetime(150),
            })
            .then_expect_error(CardError::CardAlreadyExists);
    }

    #[test]
    fn test_until_before_issued() {
        CardTestFramework::with(())
            .given_no_previous_events()
            .when(CardCommand::IssueCard {
                issued: datetime(100),
                until: datetime(50),
            })
            .then_expect_error(CardError::UntilBeforeIssued);
    }

    #[test]
    fn test_extended_ok() {
        CardTestFramework::with(())
            .given(vec![CardEvent::CardIssued {
                issued: datetime(0),
                until: datetime(50),
            }])
            .when(CardCommand::ExtendCard {
                until: datetime(200),
            })
            .then_expect_events(vec![CardEvent::CardExtended {
                until: datetime(200),
            }])
    }

    #[test]
    fn test_extended_invalid_until() {
        CardTestFramework::with(())
            .given(vec![CardEvent::CardIssued {
                issued: datetime(0),
                until: datetime(50),
            }])
            .when(CardCommand::ExtendCard {
                until: datetime(20),
            })
            .then_expect_error(CardError::InvalidExtendingDate)
    }

    #[test]
    fn test_enter_ok() {
        CardTestFramework::with(())
            .given(vec![CardEvent::CardIssued {
                issued: datetime(0),
                until: datetime(50),
            }])
            .when(CardCommand::Enter { time: datetime(25) })
            .then_expect_events(vec![CardEvent::CardEntered { time: datetime(25) }])
    }

    #[test]
    fn test_enter_after_expiration() {
        CardTestFramework::with(())
            .given(vec![CardEvent::CardIssued {
                issued: datetime(0),
                until: datetime(50),
            }])
            .when(CardCommand::Enter {
                time: datetime(100),
            })
            .then_expect_error(CardError::CardExpired)
    }

    #[test]
    fn test_enter_after_enter() {
        CardTestFramework::with(())
            .given(vec![
                CardEvent::CardIssued {
                    issued: datetime(0),
                    until: datetime(50),
                },
                CardEvent::CardEntered { time: datetime(25) },
            ])
            .when(CardCommand::Enter { time: datetime(30) })
            .then_expect_error(CardError::CardAlreadyInClub)
    }

    #[test]
    fn test_leave_ok() {
        CardTestFramework::with(())
            .given(vec![
                CardEvent::CardIssued {
                    issued: datetime(0),
                    until: datetime(50),
                },
                CardEvent::CardEntered { time: datetime(25) },
            ])
            .when(CardCommand::Leave { time: datetime(30) })
            .then_expect_events(vec![CardEvent::CardLeft { time: datetime(30) }]);
    }

    #[test]
    fn test_leave_after_leave() {
        CardTestFramework::with(())
            .given(vec![
                CardEvent::CardIssued {
                    issued: datetime(0),
                    until: datetime(50),
                },
                CardEvent::CardEntered { time: datetime(25) },
                CardEvent::CardLeft { time: datetime(30) },
            ])
            .when(CardCommand::Leave { time: datetime(40) })
            .then_expect_error(CardError::CardAlreadyLeft);
    }
}
