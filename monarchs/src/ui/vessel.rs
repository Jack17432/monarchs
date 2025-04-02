use crate::controllers::player::Player;
use crate::void_born::souls::{BoundToVessel, NextVessel, OwnedVessels};
use crate::void_born::vessels::VesselName;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_extras::{Column, TableBuilder};

pub fn vessel_switch_system(
    mut contexts: EguiContexts,
    soul: Single<(&BoundToVessel, &NextVessel, &OwnedVessels), With<Player>>,
    vessels: Query<&VesselName>,
) {
    let (curr_vessel, next_vessel, owned_vessels) = *soul;

    egui::Window::new("Vessels").show(contexts.ctx_mut(), |ui| {
        let table = TableBuilder::new(ui)
            .column(Column::auto())
            .column(Column::auto());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("state");
                });
                header.col(|ui| {
                    ui.strong("name");
                });
            })
            .body(|mut body| {
                for entity in &owned_vessels.0 {
                    let name = vessels.get(*entity).unwrap();

                    body.row(20.0, |mut row| {
                        row.set_selected(*entity == curr_vessel.0);

                        if *entity == next_vessel.0 {
                            row.col(|ui| {
                                ui.label("next");
                            });
                        } else {
                            row.col(|ui| {
                                ui.label("");
                            });
                        }

                        let name = name.get_name();
                        row.col(|ui| {
                            ui.label(name);
                        });
                    });
                }
            });
    });
}
