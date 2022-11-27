use crate::app::SVRaidReader;
use crate::{Game, Progress, TeraType, EVENT_FLATBUFFER_POINTER};
use eframe::egui;
use eframe::egui::panel::Side;
use eframe::egui::{Context, Widget};
use std::fs::OpenOptions;
use std::io::Write;
use sv_raid_reader::{read_raids, RAID_BLOCK_POINTER, RAID_BLOCK_SIZE};
use sysbot_rs::SysBotClient;

impl SVRaidReader {
    pub(crate) fn draw_left_panel(&mut self, ctx: &Context) {
        egui::SidePanel::new(Side::Left, "connection_grid").show(ctx, |ui| {
            ui.add_space(5.0);
            egui::Grid::new("input_grid").num_columns(2).show(ui, |ui| {
                ui.label("IP");
                ui.vertical_centered_justified(|ui| {
                    egui::TextEdit::singleline(&mut self.ip).show(ui);
                });
                ui.end_row();
                ui.label("Port");
                ui.vertical_centered_justified(|ui| {
                    egui::DragValue::new(&mut self.port)
                        .clamp_range(0..=9999)
                        .ui(ui);
                });
                ui.end_row();
            });
            ui.add_space(10.0);
            ui.vertical_centered_justified(|ui| {
                if self.client.is_some() {
                    if ui.button("Disconnect").clicked() {
                        self.client = None;
                    }
                } else if ui.button("Connect").clicked() {
                    if let Ok(client) = SysBotClient::connect(&self.ip, self.port) {
                        self.client = Some(client);
                    }
                }
            });

            ui.add_space(15.0);

            egui::Grid::new("filter_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Game:");
                    egui::ComboBox::new("cmb_game", "")
                        .selected_text(self.game.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.game,
                                Game::Scarlet,
                                Game::Scarlet.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.game,
                                Game::Violet,
                                Game::Violet.as_str(),
                            );
                        });
                    ui.end_row();
                    ui.label("Progress:");
                    egui::ComboBox::new("cmb_progress", "")
                        .selected_text(self.progress.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.progress,
                                Progress::None,
                                Progress::None.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.progress,
                                Progress::Badge3,
                                Progress::Badge3.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.progress,
                                Progress::Badge7,
                                Progress::Badge7.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.progress,
                                Progress::Credits,
                                Progress::Credits.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.progress,
                                Progress::PostGame,
                                Progress::PostGame.as_str(),
                            );
                        });
                    ui.end_row();
                    ui.label("Species:");
                    ui.vertical_centered_justified(|ui| {
                        egui::DragValue::new(&mut self.filter.species)
                            .clamp_range(0..=1010)
                            .ui(ui);
                    });
                    ui.end_row();
                    ui.label("Tera Type:");
                    egui::ComboBox::from_id_source("cmb_tera_type")
                        .selected_text(self.filter.tera_type.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Any,
                                TeraType::Any.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Normal,
                                TeraType::Normal.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Fighting,
                                TeraType::Fighting.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Flying,
                                TeraType::Flying.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Poison,
                                TeraType::Poison.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Ground,
                                TeraType::Ground.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Rock,
                                TeraType::Rock.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Bug,
                                TeraType::Bug.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Ghost,
                                TeraType::Ghost.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Steel,
                                TeraType::Steel.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Fire,
                                TeraType::Fire.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Water,
                                TeraType::Water.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Grass,
                                TeraType::Grass.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Electric,
                                TeraType::Electric.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Psychic,
                                TeraType::Psychic.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Ice,
                                TeraType::Ice.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Dragon,
                                TeraType::Dragon.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Dark,
                                TeraType::Dark.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.filter.tera_type,
                                TeraType::Fairy,
                                TeraType::Fairy.as_str(),
                            );
                        });
                    ui.end_row();
                    ui.label("Star Level:");
                    ui.vertical_centered_justified(|ui| {
                        egui::DragValue::new(&mut self.filter.star_level)
                            .clamp_range(0..=7)
                            .ui(ui);
                    });
                    ui.end_row();
                    ui.label("Shiny");
                    ui.checkbox(&mut self.filter.shiny, "");
                    ui.end_row();
                    ui.label("Event");
                    ui.checkbox(&mut self.filter.event, "");
                    ui.end_row();
                });

            ui.add_space(15.0);

            ui.vertical_centered_justified(|ui| {
                if ui
                    .add_enabled(self.client.is_some(), egui::Button::new("Read Raids"))
                    .clicked()
                {
                    if let Some(client) = self.client.as_ref() {
                        if let Ok(data) = client.pointer_peek(&RAID_BLOCK_POINTER, RAID_BLOCK_SIZE)
                        {
                            self.results = read_raids(
                                &data,
                                self.game.into(),
                                self.filter.into(),
                                self.progress.into(),
                            )
                            .into_iter()
                            .map(|r| r.into())
                            .collect();
                        }
                    }
                }
            });

            ui.vertical_centered_justified(|ui| {
                if ui
                    .add_enabled(
                        self.client.is_some(),
                        egui::Button::new("Download Event Data"),
                    )
                    .clicked()
                {
                    if let Ok(mut file) = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open("./raid_enemy_array")
                    {
                        if let Some(client) = self.client.as_ref() {
                            if let Ok(data) = client.pointer_peek(&EVENT_FLATBUFFER_POINTER, 30000)
                            {
                                if file.write_all(&data[..30000]).is_err() {
                                    self.error = "Failed to write data to file".to_string();
                                } else {
                                    self.error = "".to_string();
                                }
                            } else {
                                self.error = "Failed to retrieve data from console".to_string();
                            }
                        }
                    }
                }
            });

            ui.add_space(15.0);

            ui.label(&self.error);
        });
    }
}
