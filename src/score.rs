use crate::AppState;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Score {
    score: u32,
}
struct ScoreUI;

impl Default for Score {
    fn default() -> Self {
        Score { score: 0 }
    }
}

impl Score {
    pub fn incr(&mut self) {
        //TODO incr more for each turn
        self.score += 1;
    }
    pub fn reset(&mut self) {
        self.score = 0;
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Score>()
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(setup_score_ui.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(update_score_ui.system()),
            );
    }
}

fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text, With<ScoreUI>>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{:.2}", score.score);
    }
}

fn setup_score_ui(
    mut commands: Commands,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
) {
    //commands.spawn_bundle(UiCameraBundle::default());
    score.reset();
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreUI);
}
