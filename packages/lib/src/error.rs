use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct LuluError {}
impl Display for LuluError {
    fn fmt(&self, _format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<String> for LuluError {
    fn from(_: String) -> Self {
        LuluError {}
    }
}
