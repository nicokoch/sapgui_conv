#![allow(dead_code)]
use egui::{Response, TextBuffer, Ui, WidgetText};

use crate::conv::{ParseError, Parser, SapGuiConnection};

struct ConnAndString {
    conn: SapGuiConnection,
    conn_string: String,
}

impl Default for ConnAndString {
    fn default() -> Self {
        let conn = SapGuiConnection::default();
        let conn_string = conn.to_connection_string();
        Self { conn, conn_string }
    }
}

#[derive(Default)]
struct JavaToWindows {
    conn_and_string: ConnAndString,
    parser: Parser,
    parse_err: Option<ParseError>,
}

#[derive(Default)]
pub struct TemplateApp {
    wtj: ConnAndString,
    jtw: JavaToWindows,
}

impl TemplateApp {
    fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("Windows to Java");
        ui.add_space(12.0);
        self.render_win_to_java(ui);
        // ui.add_space(12.0);
        // ui.heading("Java to Windows");
        // ui.add_space(12.0);
        // self.render_java_to_windows(ui);
    }

    fn render_win_to_java(&mut self, ui: &mut Ui) {
        let mut changed = false;
        ui.horizontal_top(|ui| {
            egui::Grid::new("grid_1").num_columns(2).show(ui, |ui| {
                changed |=
                    Self::add_text_input(ui, "System ID", &mut self.wtj.conn.system_id).changed();
                changed |=
                    Self::add_text_input(ui, "Application Server", &mut self.wtj.conn.appl_server)
                        .changed();
                changed |= Self::add_text_input(ui, "Instance ID", &mut self.wtj.conn.instance_id)
                    .changed();
                changed |=
                    Self::add_text_input(ui, "Router string", &mut self.wtj.conn.router).changed();
                changed |= Self::add_text_input(ui, "Client", &mut self.wtj.conn.client).changed();
                changed |= Self::add_text_input(ui, "Username", &mut self.wtj.conn.user).changed();
                changed |= Self::add_text_input(ui, "Language", &mut self.wtj.conn.lang).changed();
            });

            egui::Grid::new("grid_2").num_columns(2).show(ui, |ui| {
                changed |= Self::add_checkbox(ui, "Activate SNC", &mut self.wtj.conn.activate_snc)
                    .changed();
                changed |=
                    Self::add_text_input(ui, "SNC Name", &mut self.wtj.conn.snc_name).changed();
                changed |=
                    Self::add_checkbox(ui, "Disable SSO", &mut self.wtj.conn.disable_sso).changed();
            });
        });

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if changed {
                self.wtj.conn_string = self.wtj.conn.to_connection_string();
            }
            ui.label(
                WidgetText::Text(self.wtj.conn_string.clone())
                    .strong()
                    .monospace(),
            );
            if ui.button("📋 copy").clicked() {
                ui.ctx().copy_text(self.wtj.conn_string.clone());
            }
        });
        ui.end_row();
    }

    fn render_java_to_windows(&mut self, ui: &mut Ui) {
        let changed = Self::add_text_input(
            ui,
            "Connection string",
            &mut self.jtw.conn_and_string.conn_string,
        )
        .changed();
        ui.end_row();

        if changed {
            match self.jtw.parser.parse(&self.jtw.conn_and_string.conn_string) {
                Ok(conn) => {
                    self.jtw.conn_and_string.conn = conn;
                    self.jtw.parse_err = None;
                }
                Err(e) => {
                    self.jtw.parse_err = Some(e);
                }
            }
        }
        ui.label(
            self.jtw
                .parse_err
                .as_ref()
                .map(|e| e.to_string())
                .unwrap_or_default(),
        );
        ui.end_row();

        ui.add_space(12.0);
        egui::Grid::new("grid2").num_columns(2).show(ui, |ui| {
            ui.label("System ID");
            ui.label(&self.jtw.conn_and_string.conn.system_id);
            ui.end_row();

            ui.label("Application server");
            ui.label(&self.jtw.conn_and_string.conn.appl_server);
            ui.end_row();

            ui.label("Instance ID");
            ui.label(&self.jtw.conn_and_string.conn.instance_id);
            ui.end_row();

            ui.label("Router string");
            ui.label(&self.jtw.conn_and_string.conn.router);
            ui.end_row();

            ui.label("Client");
            ui.label(&self.jtw.conn_and_string.conn.client);
            ui.end_row();

            ui.label("Username");
            ui.label(&self.jtw.conn_and_string.conn.user);
            ui.end_row();

            ui.label("Language");
            ui.label(&self.jtw.conn_and_string.conn.lang);
            ui.end_row();
        });
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

    fn add_checkbox<L: Into<WidgetText>>(ui: &mut Ui, label: L, flag: &mut bool) -> Response {
        ui.label(label);
        let res = ui.checkbox(flag, "");
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
