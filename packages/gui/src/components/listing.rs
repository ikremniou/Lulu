use eframe::egui;
use std::rc::Rc;

use crate::domain::{AppTab, Buffer};

pub(crate) struct Listing {
    buffer: Rc<Buffer>,
}

impl Listing {
    pub(crate) fn new(buffer: Rc<Buffer>) -> Self {
        Self { buffer }
    }

    fn show_grid_line(&mut self, ui: &mut egui::Ui) {
        ui.set_max_width(1000.0);
        ui.horizontal(|ui| {
            ui.centered_and_justified(|ui| {
                ui.label("*                       THUNK FUNCTION                       *");
            })
        });

        ui.horizontal(|ui| {
            ui.columns(6, |columns| {
                columns[0].label("0x000000");
                columns[1].label("48 3b 0d 71\n 82 00 00");
                columns[2].label("CMP");
                columns[3].label("param_1, qword ptr [rbp + 0x10]");
                columns[4].label("// comment");
            });
        });
    }
}

impl AppTab for Listing {
    fn get_title(&self) -> String {
        self.buffer.as_ref().get_file_name()
    }

    fn display(&mut self, ui: &mut eframe::egui::Ui) {
        // let len = self.buffer.as_ref().get_contents().len();
        egui::ScrollArea::new([true, true])
            .auto_shrink(false)
            .show(ui, |ui| {
                for i in 0..100 {
                    self.show_grid_line(ui);
                }
            });
    }
}
