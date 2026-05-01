use anyhow::Result;
use log::info;

pub struct UI {
    title: String,
}

pub struct BrowserUIData {
    browser_name: String,
    version: String,
    current_tab: String,
    active_window: String,
}

impl UI {
    pub async fn new(title: &str) -> Result<Self> {
        info!("Initializing UI: {}", title);

        Ok(Self {
            title: title.to_string(),
        })
    }

    pub async fn start(&self, app: &crate::app::BrowserApp) -> Result<()> {
        info!("Starting egui UI...");

        // Create initial window
        let window_id = app.browser_engine.create_window().await?;
        info!("Created main window: {}", window_id);

        // Create initial tab
        let tab = app
            .browser_engine
            .create_tab(&window_id, "about:home")
            .await?;
        info!("Created initial tab: {}", tab.id);

        // Prepare UI data
        let ui_data = BrowserUIData {
            browser_name: app.name.clone(),
            version: app.version.clone(),
            current_tab: tab.id.clone(),
            active_window: window_id.clone(),
        };

        // Run egui application
        let options = eframe::NativeOptions::default();
        let _ = eframe::run_native(
            &self.title,
            options,
            Box::new(move |_cc| Box::new(BrowserUI::new(ui_data))),
        );

        Ok(())
    }

    pub fn render_settings_panel(&self) {}

    pub fn render_permissions_panel(&self) {}

    pub fn render_extensions_panel(&self) {}
}

pub struct BrowserUI {
    data: BrowserUIData,
    address_bar: String,
}

impl BrowserUI {
    pub fn new(data: BrowserUIData) -> Self {
        Self {
            data,
            address_bar: "about:home".to_string(),
        }
    }
}

impl eframe::App for BrowserUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&format!("🌐 {}", self.data.browser_name));
            ui.label(format!("v{}", self.data.version));
            
            ui.horizontal(|ui| {
                ui.label("Address Bar:");
                ui.text_edit_singleline(&mut self.address_bar);
            });

            ui.separator();

            ui.label(format!("Active Tab: {}", self.data.current_tab));
            ui.label(format!("Active Window: {}", self.data.active_window));

            ui.separator();

            ui.label("Welcome to 4 Browser!");
            ui.label("A customizable browser engine built with Rust");

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Settings").clicked() {
                    info!("Settings clicked");
                }
                if ui.button("Extensions").clicked() {
                    info!("Extensions clicked");
                }
                if ui.button("Permissions").clicked() {
                    info!("Permissions clicked");
                }
            });
        });
    }
}

