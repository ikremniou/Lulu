use eframe::egui;

pub(crate) trait AppTab {
    fn get_title(&self) -> String;
    fn display(&mut self, ui: &mut egui::Ui);
}

pub(crate) struct AppTabs;
impl egui_dock::TabViewer for AppTabs {
    type Tab = Box<dyn AppTab>;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_ref().get_title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.as_mut().display(ui);
    }
}
