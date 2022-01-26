use bevy::{asset::Asset, ecs::component::Component, prelude::*};

use crate::{
    Animate, Animator, AnimatorState, AssetAnimator, Lens, ScaleAnimator, TransformPositionLens,
    TransformScaleLens, TranslationAnimator, TweeningType,
};

/// Plugin to add systems related to tweening
#[derive(Debug, Clone, Copy)]
pub struct TweeningPlugin;

impl Plugin for TweeningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(component_animator_system::<Transform>)
            .add_system(component_animator_system::<Text>)
            .add_system(component_animator_system::<Style>)
            .add_system(component_animator_system::<Sprite>)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_system(animate_animator_system::<ScaleAnimator, TransformScaleLens, Transform>)
                    .with_system(animate_animator_system::<TranslationAnimator, TransformPositionLens, Transform>)
            )
            .add_system(asset_animator_system::<ColorMaterial>);
    }
}

pub fn animate_animator_system<T, U, V>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut V, &mut T)>,
) where
    T: Animate<U, V> + Component,
    U: Lens<V>,
    V: Component,
{
    for (entity, ref mut target, ref mut animator) in query.iter_mut() {
        if animator.get_state() == AnimatorState::Playing {
            animator.get_timer().tick(time.delta());
        }
        if *animator.is_paused() {
            if animator.get_timer().just_finished() {
                match animator.get_tweening_type() {
                    TweeningType::Once { duration } => {
                        animator.get_timer().set_duration(duration);
                    }
                    TweeningType::Loop { duration, .. } => {
                        animator.get_timer().set_duration(duration);
                    }
                    TweeningType::PingPong { duration, .. } => {
                        animator.get_timer().set_duration(duration);
                    }
                }
                animator.get_timer().reset();
                *animator.is_paused() = false;
            }
        } else {
            if animator.get_timer().duration().as_secs_f32() != 0. {
                let progress = if animator.get_direction().is_positive() {
                    animator.get_timer().percent()
                } else {
                    animator.get_timer().percent_left()
                };
                let factor = animator.get_easing_function().sample(progress);
                animator.apply(target.as_mut(), factor);
            }
            if animator.get_timer().finished() {
                match animator.get_tweening_type() {
                    TweeningType::Once { .. } => {
                        commands.entity(entity).remove::<T>();
                    }
                    TweeningType::Loop { pause, .. } => {
                        if let Some(pause) = pause {
                            animator.get_timer().set_duration(pause);
                            *animator.is_paused() = true;
                        }
                        animator.get_timer().reset();
                    }
                    TweeningType::PingPong { pause, .. } => {
                        if let Some(pause) = pause {
                            animator.get_timer().set_duration(pause);
                            *animator.is_paused() = true;
                        }
                        animator.get_timer().reset();
                        *animator.get_direction() *= -1;
                    }
                }
            }
        }
    }
}

pub fn component_animator_system<T: Component>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut T, &mut Animator<T>)>,
) {
    for (entity, ref mut target, ref mut animator) in query.iter_mut() {
        if animator.state == AnimatorState::Playing {
            animator.timer.tick(time.delta());
        }
        if animator.paused {
            if animator.timer.just_finished() {
                match animator.tweening_type {
                    TweeningType::Once { duration } => {
                        animator.timer.set_duration(duration);
                    }
                    TweeningType::Loop { duration, .. } => {
                        animator.timer.set_duration(duration);
                    }
                    TweeningType::PingPong { duration, .. } => {
                        animator.timer.set_duration(duration);
                    }
                }
                animator.timer.reset();
                animator.paused = false;
            }
        } else {
            if animator.timer.duration().as_secs_f32() != 0. {
                let progress = if animator.direction.is_positive() {
                    animator.timer.percent()
                } else {
                    animator.timer.percent_left()
                };
                let factor = animator.ease_function.sample(progress);
                animator.apply(target, factor);
            }
            if animator.timer.finished() {
                match animator.tweening_type {
                    TweeningType::Once { .. } => {
                        commands.entity(entity).remove::<Animator<T>>();
                    }
                    TweeningType::Loop { pause, .. } => {
                        if let Some(pause) = pause {
                            animator.timer.set_duration(pause);
                            animator.paused = true;
                        }
                        animator.timer.reset();
                    }
                    TweeningType::PingPong { pause, .. } => {
                        if let Some(pause) = pause {
                            animator.timer.set_duration(pause);
                            animator.paused = true;
                        }
                        animator.timer.reset();
                        animator.direction *= -1;
                    }
                }
            }
        }
    }
}

pub fn asset_animator_system<T: Asset>(
    time: Res<Time>,
    mut assets: ResMut<Assets<T>>,
    mut query: Query<&mut AssetAnimator<T>>,
) {
    for ref mut animator in query.iter_mut() {
        if animator.state == AnimatorState::Playing {
            animator.timer.tick(time.delta());
        }
        if animator.paused {
            if animator.timer.just_finished() {
                match animator.tweening_type {
                    TweeningType::Once { duration } => {
                        animator.timer.set_duration(duration);
                    }
                    TweeningType::Loop { duration, .. } => {
                        animator.timer.set_duration(duration);
                    }
                    TweeningType::PingPong { duration, .. } => {
                        animator.timer.set_duration(duration);
                    }
                }
                animator.timer.reset();
                animator.paused = false;
            }
        } else {
            if animator.timer.duration().as_secs_f32() != 0. {
                let progress = if animator.direction.is_positive() {
                    animator.timer.percent()
                } else {
                    animator.timer.percent_left()
                };
                let factor = animator.ease_function.sample(progress);
                if let Some(target) = assets.get_mut(animator.handle()) {
                    animator.apply(target, factor);
                }
            }
            if animator.timer.finished() {
                match animator.tweening_type {
                    TweeningType::Once { .. } => {
                        // commands.entity(entity).remove::<Animator>();
                    }
                    TweeningType::Loop { pause, .. } => {
                        if let Some(pause) = pause {
                            animator.timer.set_duration(pause);
                            animator.paused = true;
                        }
                        animator.timer.reset();
                    }
                    TweeningType::PingPong { pause, .. } => {
                        if let Some(pause) = pause {
                            animator.timer.set_duration(pause);
                            animator.paused = true;
                        }
                        animator.timer.reset();
                        animator.direction *= -1;
                    }
                }
            }
        }
    }
}
