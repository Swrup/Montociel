use crate::AppState;
use crate::Materials;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
extern crate rand;

pub struct Cloud(Vec2);
struct NewCloudTimer(Timer);
pub struct Evil;
pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<NewCloudTimer>()
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(cloud_belt.system())
                    .with_system(spawn_earth.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(cloud_kinematics.system())
                    .with_system(newcloud_maker.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::GameOver)
                    .with_system(cloud_kinematics.system())
                    .with_system(newcloud_maker.system()),
            );
    }
}

impl Default for NewCloudTimer {
    fn default() -> Self {
        NewCloudTimer(Timer::from_seconds(2., true))
    }
}

fn newcloud_maker(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: ResMut<NewCloudTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let nb = 4;
        for _ in 0..nb {
            let theta = rng.gen_range(0.0..10000.) * 2. * std::f32::consts::PI / nb as f32;
            let rho = 1.;
            let speed = rng.gen_range(1.0..5.0);
            let pos = Vec2::new(f32::cos(theta), f32::sin(theta)) * rho;
            let vel = Vec2::new(pos.x, pos.y).normalize() * speed;
            let is_evil = false;
            spawn_cloud(&mut commands, &rapier_config, &materials, pos, vel, is_evil);
        }
    }
}

fn cloud_belt(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    materials: Res<Materials>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let nb = 10;
    for i in 0..nb {
        let theta = i as f32 * 2. * std::f32::consts::PI / nb as f32;
        let rho = 10.;
        let speed = rng.gen_range(0.0..5.0);
        let pos = Vec2::new(f32::cos(theta), f32::sin(theta)) * rho;
        let vel = Vec2::new(pos.x, pos.y).normalize() * speed;
        let is_evil = false;
        spawn_cloud(&mut commands, &rapier_config, &materials, pos, vel, is_evil);
    }
}

fn spawn_earth(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    materials: Res<Materials>,
) {
    //TODO sapwn mother earth the root of all evil
    let radius = 60. / rapier_config.scale;
    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Static,
        position: Vec2::new(0., 0.).into(),
        velocity: RigidBodyVelocity {
            linvel: Vec2::new(0., 0.).into(),
            angvel: 0.0,
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(radius),
        mass_properties: ColliderMassProps::Density(1.0),
        material: ColliderMaterial {
            restitution: 1.0,
            ..Default::default()
        },
        flags: (ActiveEvents::INTERSECTION_EVENTS | ActiveEvents::CONTACT_EVENTS).into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert_bundle(SpriteBundle {
            material: materials.earth_material.clone(),
            sprite: Sprite::new(Vec2::new(
                2. * radius * rapier_config.scale,
                2. * radius * rapier_config.scale,
            )),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Cloud(Vec2::new(0., 0.)))
        .insert(Evil);
}

fn spawn_cloud(
    commands: &mut Commands,
    rapier_config: &Res<RapierConfiguration>,
    materials: &Res<Materials>,
    pos: Vec2,
    vel: Vec2,
    is_evil: bool,
) {
    //Spawn a cloud
    let radius = 15. / rapier_config.scale;
    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::KinematicVelocityBased,
        position: pos.into(),
        velocity: RigidBodyVelocity {
            linvel: vel.into(),
            angvel: 0.0,
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(radius),
        mass_properties: ColliderMassProps::Density(1.0),
        material: ColliderMaterial {
            restitution: 1.0,
            ..Default::default()
        },
        flags: (ActiveEvents::INTERSECTION_EVENTS | ActiveEvents::CONTACT_EVENTS).into(),
        ..Default::default()
    };
    if is_evil {
        commands
            .spawn_bundle(rigid_body)
            .insert_bundle(collider)
            .insert_bundle(SpriteBundle {
                material: materials.cloud_material.clone(),
                sprite: Sprite::new(Vec2::new(
                    2. * radius * rapier_config.scale,
                    2. * radius * rapier_config.scale,
                )),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert(Cloud(vel))
            .insert(Evil);
    } else {
        commands
            .spawn_bundle(rigid_body)
            .insert_bundle(collider)
            .insert_bundle(SpriteBundle {
                material: materials.cloud_material.clone(),
                sprite: Sprite::new(Vec2::new(
                    2. * radius * rapier_config.scale,
                    2. * radius * rapier_config.scale,
                )),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert(Cloud(vel));
    }
}

fn cloud_kinematics(mut velocities: Query<(&Cloud, &mut RigidBodyVelocity)>) {
    for (vel, mut next_vel) in velocities.iter_mut() {
        next_vel.linvel = vel.0.into();
    }
}
