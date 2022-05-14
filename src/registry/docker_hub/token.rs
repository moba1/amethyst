use std::string;

#[derive(Debug, Clone)]
pub enum Token {
    JWT(String),
    Bearer(String),
}

impl string::ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Bearer(token) => format!("Bearer {}", token),
            Self::JWT(token) => format!("JWT {}", token),
        }
    }
}
