use eframe::egui;
use egui_dock::{DockArea, DockState, Style};
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
    context: AppContext,
    tree: DockState<String>,
}

struct AppContext {
    name: String,
    buffers: Vec<String>,
}

impl egui_dock::TabViewer for AppContext {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _tab: &mut Self::Tab) {
        self.main_ui(ui);
    }
}

impl AppContext {
    fn main_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Hello World! I'm ".to_owned() + &self.name);
        if self.buffers.len() > 0 {
            ui.label("I have ".to_owned() + &self.buffers.len().to_string() + " buffers");
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let tree = DockState::new(vec!["Listing: file.luac".to_owned()]);
        let app_context = AppContext {
            name: "Ilya".to_owned(),
            buffers: vec![],
        };

        Self {
            tree,
            context: app_context,
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
