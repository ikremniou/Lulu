use std::path::PathBuf;
use std::rc::Rc;

use egui_dock::DockState;

use super::super::components::Listing;
use super::AppTab;
use super::AppTabs;
use super::Buffer;

pub(crate) struct AppContext {
    pub tag_viewer: AppTabs,
    pub buffers: Vec<Rc<Buffer>>,
    pub tree: DockState<Box<dyn AppTab>>,
}

impl AppContext {
    pub(crate) fn new() -> Self {
        let test_buffer = Rc::new(Buffer::new(PathBuf::from("test"), vec![]));
        let test_tab = Box::new(Listing::new(test_buffer));
        Self {
            tag_viewer: AppTabs,
            buffers: Vec::new(),
            tree: DockState::new(vec![test_tab]),
        }
    }

    pub(crate) fn add_buffer(&mut self, buffer: Buffer) {
        self.buffers.push(Rc::new(buffer));
        self.tree
            .main_surface_mut()
            .root_node_mut()
            .unwrap()
            .append_tab(Box::new(Listing::new(self.buffers.last().unwrap().clone())))
    }
}
