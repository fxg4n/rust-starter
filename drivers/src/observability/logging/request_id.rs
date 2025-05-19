use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestId(String);

impl RequestId {
    pub fn new() -> Self {
        Self(cuid::cuid2())
    }

    pub fn from_str(id: &str) -> Option<Self> {
        if id.is_empty() {
            None
        } else {
            Some(Self(id.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for RequestId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
