use crate::controllers::player::Player;
use crate::void_born::souls::{BoundToVessel, NextVessel, OwnedVessels};
use crate::void_born::vessels::{Vessel, VesselName};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_extras::{Column, TableBuilder};

pub fn vessel_switch_system(
    mut contexts: EguiContexts,
    soul: Single<(&BoundToVessel, &NextVessel, &OwnedVessels), With<Player>>,
    vessels: Query<(Entity, &VesselName), With<Vessel>>,
) {
    let vessels = vessels
        .iter()
        .filter(|(entity, _)| soul.2.0.contains(entity))
        .collect::<Vec<_>>();

    egui::Window::new("Vessels").show(contexts.ctx_mut(), |ui| {
        let mut table = TableBuilder::new(ui)
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
                for (entity, name) in vessels {
                    body.row(20.0, |mut row| {
                        row.set_selected(entity == soul.0.0);

                        if entity == soul.1.0 {
                            row.col(|mut ui| {
                                ui.label("next");
                            });
                        } else {
                            row.col(|mut ui| {
                                ui.label("");
                            });
                        }

                        let name = name.get_name();
                        row.col(|mut ui| {
                            ui.label(name);
                        });
                    });
                }
            });
    });
}
