use egui::{Response, TextBuffer, Ui, WidgetText};

use crate::conv::SapGuiConnection;

pub struct TemplateApp {
    conn: SapGuiConnection,
    conn_string: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let conn = SapGuiConnection::default();
        let conn_string = conn.as_connection_string();
        Self { conn, conn_string }
    }
}

impl TemplateApp {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("Windows to Java");
        ui.add_space(12.0);
        self.render_win_to_java(ui);
        ui.add_space(12.0);
    }

    fn render_win_to_java(&mut self, ui: &mut Ui) {
        let mut changed = false;
        egui::Grid::new("grid").num_columns(2).show(ui, |ui| {
            changed |= Self::add_text_input(ui, "System ID", &mut self.conn.system_id).changed();
            changed |= Self::add_text_input(ui, "Application Server", &mut self.conn.appl_server)
                .changed();
            changed |=
                Self::add_text_input(ui, "Instance ID", &mut self.conn.instance_id).changed();
            changed |= Self::add_text_input(ui, "Router string", &mut self.conn.router).changed();
            changed |= Self::add_text_input(ui, "Client", &mut self.conn.client).changed();
            changed |= Self::add_text_input(ui, "Username", &mut self.conn.user).changed();
        });
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if changed {
                self.conn_string = self.conn.as_connection_string();
            }
            ui.label(
                WidgetText::Text(self.conn_string.clone())
                    .strong()
                    .monospace(),
            );
            if ui.button("📋 copy").clicked() {
                ui.ctx().copy_text(self.conn_string.clone());
            }
        });
        ui.end_row();
    }

    fn add_text_input<L: Into<WidgetText>, S: TextBuffer>(
        ui: &mut Ui,
        label: L,
        text: &mut S,
    ) -> Response {
        ui.label(label);
        let res = ui.text_edit_singleline(text);
        ui.end_row();
        res
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_ui(ui);
            ui.separator();
            ui.add(egui::github_link_file!(
                "https://github.com/nicokoch/sapgui_conv/blob/main/",
                "Source code."
            ));
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
