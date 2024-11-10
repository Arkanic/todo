use eframe::egui;
use super::todo;

pub struct TodoApp {
    pub todo:todo::Todo,
    pub textbox_content:String,
    pub show_done:bool
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx:&egui::Context, _frame:&mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Todo");

            ui.horizontal(|ui| {
                let textbox = ui.text_edit_singleline(&mut self.textbox_content);
                let button = ui.button("Add");

                if (textbox.lost_focus() && textbox.ctx.input(|i| {
                    i.key_pressed(egui::Key::Enter)
                })) || button.clicked() {
                    self.todo.addtask((&self.textbox_content).to_owned());
                    self.textbox_content.clear();
                }
            });
            
            ui.checkbox(&mut self.show_done, "Show finished tasks");

            for task in self.todo.gettasks(self.show_done).iter() {
                ui.horizontal(|ui| {
                    if ui.button(match task.done {
                        true => "Redo",
                        false => "Done"
                    }).clicked() {
                        self.todo.changestatus(task.id, !task.done);
                    }
                    if task.done {
                        if ui.button("Delete").clicked() {
                            self.todo.removetask(task.id);
                        }
                    }
                    ui.label(format!("{}", task));
                });
            }
        });
    }
}