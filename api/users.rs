use std::fmt::{Debug, Display, Formatter};

use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
// this is the document stored inside each user's collection.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    username: String,
    password: String,
    last_accessed: u64,       // unix timestamp
    modification_token: u128, // random 128-bit integer (or could just be unix timestamp too)
}

// this is a simple hack so that I will not need to write display code for a vector of stuff that can be displayed
// by the Display trait.
pub struct DebugToDisplay<T>(pub T);

impl<T: Display> Debug for DebugToDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
