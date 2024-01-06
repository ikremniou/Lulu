pub struct GuiError {
    message: String,
}

impl std::fmt::Display for GuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GuiError: {}", self.message)
    }
}

impl From<String> for GuiError {
    fn from(error: String) -> Self {
        GuiError {
            message: error,
        }
    }
}