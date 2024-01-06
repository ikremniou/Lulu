use eframe::egui;
use egui_dock::{DockArea, DockState, Style};

use crate::{domain, menu};

pub(crate) trait AppTab {
  fn get_title(&self) -> String;
  fn display(&mut self, ui: &mut egui::Ui);
}

pub(crate) struct MyApp {
    context: domain::AppContext,
    tree: DockState<Box<dyn AppTab>>,
}

impl egui_dock::TabViewer for domain::AppContext {
    type Tab = Box<dyn AppTab>;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_ref().get_title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.as_mut().display(ui);
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let tree = DockState::new(vec![]);
        let context = domain::AppContext::new();
        Self {
            tree,
            context,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        menu::display_menu(&mut self.context, ctx);

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.context)
    }
}