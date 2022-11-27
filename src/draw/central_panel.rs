use crate::app::SVRaidReader;
use eframe::egui;
use eframe::egui::Context;
use egui_extras::{Size, TableBuilder};

impl SVRaidReader {
    pub(crate) fn draw_central_panel(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::centered_and_justified(
                    egui::Direction::RightToLeft,
                ))
                .column(Size::initial(150.0))
                .column(Size::initial(40.0))
                .columns(Size::initial(100.0), 2)
                .columns(Size::initial(50.0), 2)
                .columns(Size::initial(100.0), 2)
                .columns(Size::initial(40.0), 6)
                .columns(Size::initial(100.0), 2)
                .column(Size::initial(60.0))
                .resizable(true)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Area");
                    });
                    header.col(|ui| {
                        ui.heading("Stars");
                    });
                    header.col(|ui| {
                        ui.heading("Tera");
                    });
                    header.col(|ui| {
                        ui.heading("Seed");
                    });
                    header.col(|ui| {
                        ui.heading("Event");
                    });
                    header.col(|ui| {
                        ui.heading("Shiny");
                    });
                    header.col(|ui| {
                        ui.heading("Species");
                    });
                    header.col(|ui| {
                        ui.heading("PID");
                    });
                    header.col(|ui| {
                        ui.heading("HP");
                    });
                    header.col(|ui| {
                        ui.heading("ATK");
                    });
                    header.col(|ui| {
                        ui.heading("DEF");
                    });
                    header.col(|ui| {
                        ui.heading("SpA");
                    });
                    header.col(|ui| {
                        ui.heading("SpD");
                    });
                    header.col(|ui| {
                        ui.heading("Spe");
                    });
                    header.col(|ui| {
                        ui.heading("Nature");
                    });
                    header.col(|ui| {
                        ui.heading("Abilty");
                    });
                    header.col(|ui| {
                        ui.heading("Gender");
                    });
                })
                .body(|body| {
                    body.rows(18.0, self.results.len(), |index, mut row| {
                        let result = self.results.get(index).unwrap();
                        row.col(|ui| {
                            ui.label(result.area);
                        });
                        row.col(|ui| {
                            ui.label(&result.star_level);
                        });
                        row.col(|ui| {
                            ui.label(result.tera);
                        });
                        row.col(|ui| {
                            ui.label(&result.seed);
                        });
                        row.col(|ui| {
                            ui.label(result.event);
                        });
                        row.col(|ui| {
                            ui.label(result.shiny);
                        });
                        row.col(|ui| {
                            ui.label(result.species);
                        });
                        row.col(|ui| {
                            ui.label(&result.pid);
                        });
                        row.col(|ui| {
                            ui.label(&result.hp);
                        });
                        row.col(|ui| {
                            ui.label(&result.atk);
                        });
                        row.col(|ui| {
                            ui.label(&result.def);
                        });
                        row.col(|ui| {
                            ui.label(&result.spa);
                        });
                        row.col(|ui| {
                            ui.label(&result.spd);
                        });
                        row.col(|ui| {
                            ui.label(&result.spe);
                        });
                        row.col(|ui| {
                            ui.label(result.nature);
                        });
                        row.col(|ui| {
                            ui.label(result.ability);
                        });
                        row.col(|ui| {
                            ui.label(&result.gender);
                        });
                    })
                });
        });
    }
}
