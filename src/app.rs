use egui::{Ui, WidgetText};

use crate::conv::SapGuiConnection;

#[derive(Default)]
pub struct TemplateApp {
    conn: SapGuiConnection,
}

impl TemplateApp {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("Windows to Java");
        ui.add_space(12.0);
        self.render_win_to_java(ui);
        ui.add_space(12.0);
    }

    fn render_win_to_java(&mut self, ui: &mut Ui) {
        egui::Grid::new("grid").num_columns(2).show(ui, |ui| {
            ui.label("System ID");
            ui.text_edit_singleline(&mut self.conn.system_id);
            ui.end_row();
            ui.label("Application server");
            ui.text_edit_singleline(&mut self.conn.appl_server);
            ui.end_row();
            ui.label("Instance id");
            ui.text_edit_singleline(&mut self.conn.instance_id);
            ui.end_row();
            ui.label("Router string");
            ui.text_edit_singleline(&mut self.conn.router);
            ui.end_row();
            ui.label("Client");
            ui.text_edit_singleline(&mut self.conn.client);
            ui.end_row();
            ui.label("Username");
            ui.text_edit_singleline(&mut self.conn.user);
            ui.end_row();
        });
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            let conn_string = self.conn.as_connection_string();
            ui.label(WidgetText::Text(conn_string.clone()).strong().monospace());
            if ui.button("📋 copy").clicked() {
                ui.ctx().copy_text(conn_string);
            }
        });
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
        });
    }
}
