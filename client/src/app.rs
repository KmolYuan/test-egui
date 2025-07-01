use std::collections::BTreeSet;

pub(crate) struct App {
    file_list: Vec<String>,
    file_search: String,
    selected_file: BTreeSet<usize>,
    filtered_file: BTreeSet<usize>,
}

impl App {
    pub(crate) fn new(_ctx: &eframe::CreationContext) -> Self {
        Self {
            file_list: (0..100).map(|i| format!("file_{i}.txt")).collect(),
            file_search: String::new(),
            selected_file: Default::default(),
            filtered_file: Default::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🖹 File Host");
                ui.add(SearchBar::new(
                    &mut self.file_search,
                    &self.file_list,
                    &mut self.filtered_file,
                ));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(UrlBar);
            ui.horizontal(|ui| {
                ui.label("Selected files:");
                ui.label(
                    self.selected_file
                        .iter()
                        .map(|i| format!("{i},"))
                        .collect::<String>(),
                );
            });

            let space = ui.spacing().interact_size.y;
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    if !self.selected_file.is_empty() && ui.checkbox(&mut true, "").clicked() {
                        self.selected_file.clear();
                    }
                    ui.label("File Name");
                });

                let scroll_area = egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .scroll_source(egui::scroll_area::ScrollSource {
                        scroll_bar: true,
                        drag: false,
                        mouse_wheel: true,
                    });
                if self.file_search.is_empty() {
                    scroll_area.show_rows(ui, space, self.file_list.len(), |ui, rng| {
                        for i in rng {
                            ui.horizontal(|ui| self.file_list(ui, i));
                        }
                    });
                } else {
                    let filtered_file = Vec::from_iter(self.filtered_file.clone());
                    scroll_area.show_rows(ui, space, filtered_file.len(), |ui, rng| {
                        for i in rng {
                            ui.horizontal(|ui| self.file_list(ui, filtered_file[i]));
                        }
                    });
                }
            });
        });
    }
}

impl App {
    fn file_list(&mut self, ui: &mut egui::Ui, i: usize) {
        let mut checked = self.selected_file.contains(&i);
        if !self.selected_file.is_empty() && ui.checkbox(&mut checked, "").clicked() {
            if checked {
                self.selected_file.insert(i);
                if ui.input(|i| i.modifiers.matches_exact(egui::Modifiers::SHIFT)) {
                    self.shift_selected(i);
                }
            } else {
                self.selected_file.remove(&i);
            }
        }
        if ui
            .selectable_label(checked, format!("🖹 {}", self.file_list[i]))
            .clicked()
        {
            if ui.input(|i| i.modifiers.matches_exact(egui::Modifiers::SHIFT)) {
                self.shift_selected(i);
            } else {
                self.selected_file.clear();
                self.selected_file.insert(i);
            }
        }
        ui.label("Fake date");
    }

    fn shift_selected(&mut self, i: usize) {
        let min = *self.selected_file.first().unwrap();
        let max = *self.selected_file.last().unwrap();
        if i < min {
            self.selected_file.extend(i..min);
        } else if i > max {
            self.selected_file.extend(max..=i);
        } else {
            self.selected_file.extend(min..=max);
        }
    }
}

struct UrlBar;

impl egui::Widget for UrlBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            let location = web_sys::window().unwrap().location();
            let mut origin = location.origin().unwrap();

            if ui.button("🏠 Home").clicked() {
                ui.ctx().open_url(egui::OpenUrl::same_tab(&origin));
            }

            let uri = location.pathname().unwrap();
            for path in uri.trim_start_matches('/').split('/') {
                ui.label("/");
                origin.push('/');
                origin += path;
                if !path.is_empty() && ui.button(path).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::same_tab(&origin));
                }
            }
        })
        .response
    }
}

struct SearchBar<'a> {
    query: &'a mut String,
    items: &'a [String],
    filtered: &'a mut BTreeSet<usize>,
}

impl<'a> SearchBar<'a> {
    fn new(query: &'a mut String, items: &'a [String], filtered: &'a mut BTreeSet<usize>) -> Self {
        Self {
            query,
            items,
            filtered,
        }
    }
}

impl eframe::egui::Widget for SearchBar<'_> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.horizontal(|ui| {
            if ui
                .add(egui::TextEdit::singleline(self.query).hint_text("🔍 Search files..."))
                .changed()
            {
                if self.query.is_empty() {
                    self.filtered.clear();
                    return;
                }
                for (i, item) in self.items.iter().enumerate() {
                    if item
                        .to_ascii_lowercase()
                        .matches(&self.query.to_ascii_lowercase())
                        .count()
                        > 0
                    {
                        self.filtered.insert(i);
                    } else {
                        self.filtered.remove(&i);
                    }
                }
            }
            if !self.query.is_empty() && ui.button("❌").clicked() {
                self.query.clear();
            }
        })
        .response
    }
}
