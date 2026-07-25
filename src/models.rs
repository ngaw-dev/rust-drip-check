use std::fmt;
use diesel::prelude::*;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, AsRefStr)]
pub enum Duration {
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

impl Duration {
    pub fn from_index(index: usize) -> Option<Duration> {
        if index == 0 {
            return None;
        }
        Duration::iter().nth(index - 1)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::subscriptions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Subscription {
    pub id: i32,
    pub title: String,
    pub price: i32,
    pub duration: String,
    pub start_date: String,
}

use crate::schema::subscriptions;

#[derive(Insertable)]
#[diesel(table_name = subscriptions)]
pub struct NewSubscription {
    pub title: String,
    pub price: i32,
    pub duration: String,
    pub start_date: String,
}
