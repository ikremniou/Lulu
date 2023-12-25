use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct LuluError {
    message: String,
}
impl Display for LuluError {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return format.write_str(&self.message);
    }
}

impl From<String> for LuluError {
    fn from(error: String) -> Self {
        LuluError {
            message: error,
        }
    }
}
