use bevy::prelude::*;

#[derive(Component, Debug)]
#[require(Visibility)]
pub struct VisibleStateScoped<S: States>(S);

pub trait StateScopedVisibilityAppExt {
    fn add_state_scoped_visibility<S: States>(&mut self) -> &mut Self;
}

impl StateScopedVisibilityAppExt for App {
    fn add_state_scoped_visibility<S: States>(&mut self) -> &mut Self {

        if !self
            .world()
            .contains_resource::<Events<StateTransitionEvent<S>>>()
        {
            let name = core::any::type_name::<S>();
            warn!(state = ?name, "State transition event not found");
        }

        self.add_systems(StateTransition, change_vis_on_state_change::<S>);
        self
    }
}

fn change_vis_on_state_change<S: States>(
    mut query: Query<(&mut Visibility, &VisibleStateScoped<S>)>,
    mut transitions: EventReader<StateTransitionEvent<S>>,
) {
    let Some(transition) = transitions.read().last() else {
        return;
    };
    if transition.entered == transition.exited {
        return;
    }

    if let Some(exited) = &transition.exited {
        for (mut vis, binding) in &mut query {
            if binding.0 == *exited {
                *vis = Visibility::Hidden;
            }
        }
    };

    if let Some(entered) = &transition.entered {
        for (mut vis, binding) in &mut query {
            if binding.0 == *entered {
                *vis = Visibility::Visible;
            }
        }
    }
}
