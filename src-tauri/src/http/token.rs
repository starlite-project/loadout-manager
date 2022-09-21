use std::time::SystemTime;

use serde::{Serialize, Deserialize};


pub struct Token {
	value: String,
	expires: SystemTime,
    name: TokenType,
    inception: SystemTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TokenType {
	Access,
	Refresh,
}