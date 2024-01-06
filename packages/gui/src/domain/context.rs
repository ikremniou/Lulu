use super::buffer::Buffer;

pub(crate) struct AppContext {
    pub buffers: Vec<Buffer>,
}

impl AppContext {
    pub(crate) fn new() -> Self {
        Self {
            buffers: Vec::new(),
        }
    }
}