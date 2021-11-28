use crate::AppState;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_button.system()))
            .add_system_set(
                SystemSet::on_enter(AppState::GameOver).with_system(setup_button.system()),
            )
            .add_system(button_system.system());
    }
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    entities: Query<Entity, Without<bevy::render::camera::Camera>>,
    button: Query<Entity, With<Button>>,
    mut state: ResMut<State<AppState>>,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                match state.current() {
                    AppState::Menu => {
                        for button in button.iter() {
                            commands.entity(button).despawn_recursive();
                        }
                        state.set(AppState::InGame).unwrap();
                    }
                    AppState::GameOver => {
                        //despawn all entities
                        for entity in entities.iter() {
                            commands.entity(entity).despawn();
                        }
                        for button in button.iter() {
                            commands.entity(button).despawn_recursive();
                        }
                        state.set(AppState::InGame).unwrap();
                    }
                    AppState::InGame => panic!(),
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<AppState>>,
) {
    let text = match state.current() {
        AppState::Menu => "Play!",
        AppState::InGame => panic!(),
        AppState::GameOver => "Revive!",
    };
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            //material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}
