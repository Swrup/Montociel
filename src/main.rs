use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod cloud;
mod montociel;
mod score;
mod ui;

use cloud::*;
use montociel::*;
use score::*;
use ui::*;

struct Materials {
    //TODO background image or smthing
    montociel_material: Handle<ColorMaterial>,
    cloud_material: Handle<ColorMaterial>,
    earth_material: Handle<ColorMaterial>,
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let montociel_asset = asset_server.load("montociel.png");
        let cloud_asset = asset_server.load("cloud.png");
        let earth_asset = asset_server.load("earth.png");
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let montociel_material = materials.add(montociel_asset.into());
        let cloud_material = materials.add(cloud_asset.into());
        let earth_material = materials.add(earth_asset.into());
        Materials {
            montociel_material,
            cloud_material,
            earth_material,
        }
    }
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.scale = 15.;
    rapier_config.gravity = Vec2::new(0.0, 0.0).into();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    InGame,
    GameOver,
}

pub fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ClearColor(Color::rgb(1.0, 0.714, 0.757)))
        .insert_resource(AssetServerSettings {
            asset_folder: "/".to_string(),
        })
        .init_resource::<Materials>()
        .add_plugin(MontocielPlugin)
        .add_plugin(CloudPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(UIPlugin)
        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup.system()));

    app.run();
}
