use std::cell::RefCell;

use glam::{Quat, Vec3, Vec4};

pub struct BaseProviderContext {
    //Score
    base_combo: RefCell<f32>,
    multiplied_score: RefCell<f32>,
    immediate_max_possible_multiplied_score: RefCell<f32>,
    modified_score: RefCell<f32>,
    immediate_max_possible_modified_score: RefCell<f32>,
    relative_score: RefCell<f32>,
    multiplier: RefCell<f32>,
    energy: RefCell<f32>,
    song_time: RefCell<f32>,
    song_length: RefCell<f32>,

    //Colors
    environment_color_0: RefCell<Vec4>,
    environment_color_0_boost: RefCell<Vec4>,
    environment_color_1: RefCell<Vec4>,
    environment_color_1_boost: RefCell<Vec4>,
    environment_color_w: RefCell<Vec4>,
    environment_color_w_boost: RefCell<Vec4>,
    note_color_0: RefCell<Vec4>,
    note_color_1: RefCell<Vec4>,
    obstacles_color: RefCell<Vec4>,
    saber_color_a: RefCell<Vec4>,
    saber_color_b: RefCell<Vec4>,

    //Transforms
    head_local_position: RefCell<Vec3>,
    head_local_rotation: RefCell<Quat>,
    head_local_scale: RefCell<Vec3>,
    head_position: RefCell<Vec3>,
    head_rotation: RefCell<Quat>,
    left_hand_local_position: RefCell<Vec3>,
    left_hand_local_rotation: RefCell<Quat>,
    left_hand_local_scale: RefCell<Vec3>,
    left_hand_position: RefCell<Vec3>,
    left_hand_rotation: RefCell<Quat>,
    right_hand_local_position: RefCell<Vec3>,
    right_hand_local_rotation: RefCell<Quat>,
    right_hand_local_scale: RefCell<Vec3>,
    right_hand_position: RefCell<Vec3>,
    right_hand_rotation: RefCell<Quat>,
}

impl BaseProviderContext {
    pub fn new() -> Self {
        Self {
            base_combo: RefCell::new(0.0),
            multiplied_score: RefCell::new(0.0),
            immediate_max_possible_multiplied_score: RefCell::new(0.0),
            modified_score: RefCell::new(0.0),
            immediate_max_possible_modified_score: RefCell::new(0.0),
            relative_score: RefCell::new(0.0),
            multiplier: RefCell::new(0.0),
            energy: RefCell::new(0.0),
            song_time: RefCell::new(0.0),
            song_length: RefCell::new(0.0),
            environment_color_0: RefCell::new(Vec4::ZERO),
            environment_color_0_boost: RefCell::new(Vec4::ZERO),
            environment_color_1: RefCell::new(Vec4::ZERO),
            environment_color_1_boost: RefCell::new(Vec4::ZERO),
            environment_color_w: RefCell::new(Vec4::ZERO),
            environment_color_w_boost: RefCell::new(Vec4::ZERO),
            note_color_0: RefCell::new(Vec4::ZERO),
            note_color_1: RefCell::new(Vec4::ZERO),
            obstacles_color: RefCell::new(Vec4::ZERO),
            saber_color_a: RefCell::new(Vec4::ZERO),
            saber_color_b: RefCell::new(Vec4::ZERO),
            head_local_position: RefCell::new(Vec3::ZERO),
            head_local_rotation: RefCell::new(Quat::IDENTITY),
            head_local_scale: RefCell::new(Vec3::ONE),
            head_position: RefCell::new(Vec3::ZERO),
            head_rotation: RefCell::new(Quat::IDENTITY),
            left_hand_local_position: RefCell::new(Vec3::ZERO),
            left_hand_local_rotation: RefCell::new(Quat::IDENTITY),
            left_hand_local_scale: RefCell::new(Vec3::ONE),
            left_hand_position: RefCell::new(Vec3::ZERO),
            left_hand_rotation: RefCell::new(Quat::IDENTITY),
            right_hand_local_position: RefCell::new(Vec3::ZERO),
            right_hand_local_rotation: RefCell::new(Quat::IDENTITY),
            right_hand_local_scale: RefCell::new(Vec3::ONE),
            right_hand_position: RefCell::new(Vec3::ZERO),
            right_hand_rotation: RefCell::new(Quat::IDENTITY),
        }
    }

    pub fn get_values(&self, base: &str) -> Value {
        match base {
            "baseCombo" => vec![self.base_combo.borrow().clone()],
            "baseMultipliedScore" => vec![self.multiplied_score.borrow().clone()],
            "baseImmediateMaxPossibleMultipliedScore" => vec![
                self.immediate_max_possible_multiplied_score
                    .borrow()
                    .clone(),
            ],
            "baseModifiedScore" => vec![self.modified_score.borrow().clone()],
            "baseImmediateMaxPossibleModifiedScore" => {
                vec![self.immediate_max_possible_modified_score.borrow().clone()]
            }
            "baseRelativeScore" => vec![self.relative_score.borrow().clone()],
            "baseMultiplier" => vec![self.multiplier.borrow().clone()],
            "baseEnergy" => vec![self.energy.borrow().clone()],
            "baseSongTime" => vec![self.song_time.borrow().clone()],
            "baseSongLength" => vec![self.song_length.borrow().clone()],

            "baseEnvironmentColor0" => self
                .environment_color_0
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseEnvironmentColor0Boost" => self
                .environment_color_0_boost
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseEnvironmentColor1" => self
                .environment_color_1
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseEnvironmentColor1Boost" => self
                .environment_color_1_boost
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseEnvironmentColorW" => self
                .environment_color_w
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseEnvironmentColorWBoost" => self
                .environment_color_w_boost
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseNote0Color" => self.note_color_0.borrow().clone().to_array().to_vec(),
            "baseNote1Color" => self.note_color_1.borrow().clone().to_array().to_vec(),
            "baseObstaclesColor" => self.obstacles_color.borrow().clone().to_array().to_vec(),
            "baseSaberAColor" => self.saber_color_a.borrow().clone().to_array().to_vec(),
            "baseSaberBColor" => self.saber_color_b.borrow().clone().to_array().to_vec(),

            "baseHeadLocalPosition" => self
                .head_local_position
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseHeadLocalRotation" => self
                .head_local_rotation
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseHeadLocalScale" => self.head_local_scale.borrow().clone().to_array().to_vec(),
            "baseHeadPosition" => self.head_position.borrow().clone().to_array().to_vec(),
            "baseHeadRotation" => self.head_rotation.borrow().clone().to_array().to_vec(),
            "baseLeftHandLocalPosition" => self
                .left_hand_local_position
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseLeftHandLocalRotation" => self
                .left_hand_local_rotation
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseLeftHandLocalScale" => self
                .left_hand_local_scale
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseLeftHandPosition" => self.left_hand_position.borrow().clone().to_array().to_vec(),
            "baseLeftHandRotation" => self.left_hand_rotation.borrow().clone().to_array().to_vec(),
            "baseRightHandLocalPosition" => self
                .right_hand_local_position
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseRightHandLocalRotation" => self
                .right_hand_local_rotation
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseRightHandLocalScale" => self
                .right_hand_local_scale
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseRightHandPosition" => self
                .right_hand_position
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            "baseRightHandRotation" => self
                .right_hand_rotation
                .borrow()
                .clone()
                .to_array()
                .to_vec(),
            _ => panic!("Base provider not found"),
        }
    }

    pub fn set_values(&self, base: &str, values: Value) {
        match base {
            "baseCombo" => {
                self.base_combo.replace(values[0]);
            }
            "baseMultipliedScore" => {
                self.multiplied_score.replace(values[0]);
            }
            "baseImmediateMaxPossibleMultipliedScore" => {
                self.immediate_max_possible_multiplied_score
                    .replace(values[0]);
            }
            "baseModifiedScore" => {
                self.modified_score.replace(values[0]);
            }
            "baseImmediateMaxPossibleModifiedScore" => {
                self.immediate_max_possible_modified_score
                    .replace(values[0]);
            }
            "baseRelativeScore" => {
                self.relative_score.replace(values[0]);
            }
            "baseMultiplier" => {
                self.multiplier.replace(values[0]);
            }
            "baseEnergy" => {
                self.energy.replace(values[0]);
            }
            "baseSongTime" => {
                self.song_time.replace(values[0]);
            }
            "baseSongLength" => {
                self.song_length.replace(values[0]);
            }
            "baseEnvironmentColor0" => {
                self.environment_color_0
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseEnvironmentColor0Boost" => {
                self.environment_color_0_boost
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseEnvironmentColor1" => {
                self.environment_color_1
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseEnvironmentColor1Boost" => {
                self.environment_color_1_boost
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseEnvironmentColorW" => {
                self.environment_color_w
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseEnvironmentColorWBoost" => {
                self.environment_color_w_boost
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseNote0Color" => {
                self.note_color_0
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseNote1Color" => {
                self.note_color_1
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseObstaclesColor" => {
                self.obstacles_color
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseSaberAColor" => {
                self.saber_color_a
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseSaberBColor" => {
                self.saber_color_b
                    .replace(Vec4::from_slice(values.as_slice()));
            }
            "baseHeadLocalPosition" => {
                self.head_local_position
                    .replace(Vec3::from_slice(values.as_slice()));
            }
            "baseHeadLocalRotation" => {
                self.head_local_rotation
                    .replace(Quat::from_slice(values.as_slice()));
            }
            "baseHeadLocalScale" => {
                self.head_local_scale
                    .replace(Vec3::from_slice(values.as_slice()));
            }
            "baseHeadPosition" => {
                self.head_position
                    .replace(Vec3::from_slice(values.as_slice()));
            }
            "baseHeadRotation" => {
                self.head_rotation
                    .replace(Quat::from_slice(values.as_slice()));
            }
            "baseLeftHandLocalPosition" => {
                self.left_hand_local_position
                    .replace(Vec3::from_slice(values.as_slice()));
            }
            "baseLeftHandLocalRotation" => {
                self.left_hand_local_rotation
                    .replace(Quat::from_slice(values.as_slice()));
            }
            "baseLeftHandLocalScale" => {
                self.left_hand_local_scale
                    .replace(Vec3::from_slice(values.as_slice()));
            }
            "baseLeftHandPosition" => {
                self.left_hand_position
                    .replace(Vec3::from_slice(values.as_slice()));
            }
            "baseLeftHandRotation" => {
                self.left_hand_rotation
                    .replace(Quat::from_slice(values.as_slice()));
            }
            "baseRightHandLocalPosition" => {
                self.right_hand_local_position
                    .replace(Vec3::from_slice(values.as_slice()));
            }
            "baseRightHandLocalRotation" => {
                self.right_hand_local_rotation
                    .replace(Quat::from_slice(values.as_slice()));
            }
            "baseRightHandLocalScale" => {
                self.right_hand_local_scale
                    .replace(Vec3::from_slice(values.as_slice()));
            }
            "baseRightHandPosition" => {
                self.right_hand_position
                    .replace(Vec3::from_slice(values.as_slice()));
            }
            "baseRightHandRotation" => {
                self.right_hand_rotation
                    .replace(Quat::from_slice(values.as_slice()));
            }
            _ => panic!("Base provider not found"),
        }
    }
}
