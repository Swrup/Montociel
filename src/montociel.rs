use crate::AppState;
use crate::Evil;
use crate::Materials;
use crate::Score;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct Montociel;
pub struct MontocielPlugin;

impl Plugin for MontocielPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(spawn_montociel.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(input_movement.system())
                .with_system(montociel_aerodynamism.system())
                .with_system(update_forces.system())
                .with_system(clamp_velocity.system())
                .with_system(cloud_collision.system()),
        );
    }
}

fn spawn_montociel(
    mut commands: Commands,
    rapier_config: ResMut<RapierConfiguration>,
    materials: Res<Materials>,
) {
    //Spawn Montociel
    let radius = 30. / rapier_config.scale;
    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        position: Vec2::new(10., 10.).into(),
        velocity: RigidBodyVelocity {
            linvel: Vec2::new(0.0, 0.0).into(),
            angvel: 0.4,
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        //collider_type: ColliderType::Sensor,
        shape: ColliderShape::ball(radius),
        mass_properties: ColliderMassProps::Density(1.0),
        material: ColliderMaterial {
            restitution: 0.0,
            ..Default::default()
        },
        flags: (ActiveEvents::INTERSECTION_EVENTS | ActiveEvents::CONTACT_EVENTS).into(),
        ..Default::default()
    };

    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert_bundle(SpriteBundle {
            material: materials.montociel_material.clone(),
            sprite: Sprite::new(Vec2::new(
                2. * radius * rapier_config.scale,
                2. * radius * rapier_config.scale,
            )),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Montociel);
}

fn input_movement(
    mouse_input: Res<Input<MouseButton>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut montociel_info: Query<(&Montociel, &mut RigidBodyVelocity, &RigidBodyPosition)>,
) {
    for (_, mut velocity, pos) in montociel_info.iter_mut() {
        if mouse_input.get_pressed().len() > 0 {
            let x = pos.position.translation.x;
            let y = pos.position.translation.y;
            let theta = std::f32::consts::PI / 2.;
            let mut move_delta = Vec2::new(
                -x * f32::cos(theta) + y * f32::sin(theta),
                -x * f32::sin(theta) - y * f32::cos(theta),
            );

            if move_delta != Vec2::new(0., 0.) {
                // Note that the RapierConfiguration::Scale factor is also used here to transform
                // the move_delta from: 'pixels/second' to 'physics_units/second'
                move_delta.normalize();
                move_delta /= rapier_parameters.scale;
            }

            // Update the velocity on the rigid_body_component,
            // the bevy_rapier plugin will update the Sprite transform.
            // cringeee
            let power = 0.8;
            let v_x = power * move_delta.x + velocity.linvel.x;
            let v_y = power * move_delta.y + velocity.linvel.y;
            velocity.linvel = Vec2::new(v_x, v_y).into();
        }
    }
}

fn montociel_aerodynamism(mut velocities: Query<&mut RigidBodyVelocity, With<Montociel>>) {
    for mut velocity in velocities.iter_mut() {
        let v_x = velocity.linvel.x;
        let v_y = velocity.linvel.y;
        let velocity_slowed_by_air = 0.95 * Vec2::new(v_x, v_y);
        velocity.linvel = velocity_slowed_by_air.into();
    }
}

fn update_forces(
    mut rigid_bodies: Query<
        (
            &mut RigidBodyForces,
            &RigidBodyPosition,
            &RigidBodyMassProps,
        ),
        With<Montociel>,
    >,
) {
    for (mut rb_forces, rb_pos, rb_mass) in rigid_bodies.iter_mut() {
        let x = rb_pos.position.translation.x;
        let y = rb_pos.position.translation.y;
        let d2 = x * x + y * y;
        let norm = f32::sqrt(d2);
        let g = 100.;
        let eps = 0.0001;
        let nimp = -g * rb_mass.mass() * 1. / (norm + eps);
        let gravity = Vec2::new(nimp * x, nimp * y).into();
        rb_forces.force = gravity;
    }
}

fn clamp_velocity(
    mut bodies_info: Query<
        &mut RigidBodyVelocity, //, With<Montociel>
    >,
) {
    for mut velocity in bodies_info.iter_mut() {
        let v_x = velocity.linvel.x;
        let v_y = velocity.linvel.y;
        let magnitude = f32::sqrt(v_x * v_x + v_y * v_y);
        let max_magnitude = 70.;
        if magnitude > 0.01 {
            let clamped_velocity =
                Vec2::new(v_x, v_y) / magnitude * f32::min(magnitude, max_magnitude);
            velocity.linvel = clamped_velocity.into();
        }
    }
}

fn cloud_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut state: ResMut<State<AppState>>,
    mut montociel_info: Query<
        (Entity, &mut RigidBodyVelocity, &RigidBodyPosition),
        With<Montociel>,
    >,
    mut contact_events: EventReader<ContactEvent>,
    query: Query<Entity, With<Evil>>,
    rapier_config: Res<RapierConfiguration>,
) {
    for contact_event in contact_events.iter() {
        for (montociel_entity, mut vel, pos) in montociel_info.iter_mut() {
            match contact_event {
                ContactEvent::Started(collider1, collider2) => {
                    let entity1 = collider1.entity();
                    let entity2 = collider2.entity();
                    if entity1 != montociel_entity && entity2 != montociel_entity {
                        continue;
                    }
                    let entity = if entity1 == montociel_entity {
                        entity2
                    } else {
                        entity1
                    };
                    if query.get(entity).is_ok() {
                        state.set(AppState::GameOver).unwrap();
                    } else {
                        commands.entity(entity).despawn();
                        jump(pos, &mut vel, &rapier_config);
                        //increment score
                        score.incr();
                    }
                }
                ContactEvent::Stopped(_collider1, _collider2) => {}
            }
        }
    }
}

fn jump(
    pos: &RigidBodyPosition,
    vel: &mut RigidBodyVelocity,
    rapier_config: &Res<RapierConfiguration>,
) {
    let x = pos.position.translation.x;
    let y = pos.position.translation.y;
    let theta = 3. * std::f32::consts::PI / 4.;
    let mut move_delta = Vec2::new(
        -x * f32::cos(theta) + y * f32::sin(theta),
        -x * f32::sin(theta) - y * f32::cos(theta),
    );
    if move_delta != Vec2::new(0., 0.) {
        move_delta.normalize();
        move_delta /= rapier_config.scale;
    }
    let power = 70.;
    let v_x = power * move_delta.x + vel.linvel.x;
    let v_y = power * move_delta.y + vel.linvel.y;
    vel.linvel = Vec2::new(v_x, v_y).into();
}
