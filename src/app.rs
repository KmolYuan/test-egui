pub(crate) struct App {
    file_list: Vec<String>,
    selected_file: std::collections::BTreeSet<usize>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_list: (0..100).map(|i| format!("file_{i}.txt")).collect(),
            selected_file: Default::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🖹 File Host");
            ui.separator();
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

                egui::ScrollArea::vertical()
                    .drag_to_scroll(false)
                    .auto_shrink([false; 2])
                    .show_rows(ui, space, self.file_list.len(), |ui, rng| {
                        for i in rng {
                            ui.horizontal(|ui| self.file_list(ui, i));
                        }
                    });
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
