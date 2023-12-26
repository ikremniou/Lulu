use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
mod menu;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Lulu Disassembler",
        options,
        Box::new(|_| Box::<MyApp>::default()),
    )
}

struct MyApp {
    tree: DockState<String>,
}

struct TabViewer;
impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec!["tab1".to_owned(), "tab2".to_owned()]);

        let [a, b] =
            tree.main_surface_mut()
                .split_left(NodeIndex::root(), 0.3, vec!["tab3".to_owned()]);
        let [_, _] = tree
            .main_surface_mut()
            .split_below(a, 0.7, vec!["tab4".to_owned()]);
        let [_, _] = tree
            .main_surface_mut()
            .split_below(b, 0.5, vec!["tab5".to_owned()]);

        Self { tree }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        menu::display_menu(ctx);

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer {})
    }
}
