use std::{fs::File, io::Read, path::PathBuf};

use crate::domain::{Buffer, AppContext};
use crate::error::GuiError;

pub(crate) fn open_file(ctx: &mut AppContext) {
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        match open_file_internal(path) {
            Ok(buffer) => {
                ctx.buffers.push(buffer);
            },
            Err(_error) => {
                //TODO: handle error in dialog
            }
        }
    }
}

pub(crate) fn open_folder(_ctx: &mut crate::domain::AppContext) {
    // TODO: open folder with the files to process
    // probably we should create a lazy buffer for that
}

fn open_file_internal(path: PathBuf) -> Result<Buffer, GuiError> {
    let mut contents = Vec::new();
    let mut f = File::open(&path).map_err(|e| format!("Error opening file: {}", e))?;
    f.read_to_end(&mut contents)
        .map_err(|e| format!("Error reading file: {}", e))?;

    Ok(Buffer::new(path, contents))
}
