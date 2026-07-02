use std::time::{Duration, Instant};

use crate::memory::{gamedata::GameData, instance_manager::WindowInfo};

use super::{missile::Missile, skills::{PlayerSkill, Skill}, states::State};
use crate::gui::draw_buff_bar::BuffBarAnimationState;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuffInstance {
    pub window_info: WindowInfo,
    pub buff_timers: BuffTimers,
    pub buff_animation_state: BuffBarAnimationState,
}

impl BuffInstance {
    pub fn new(window_info: WindowInfo) -> Self {
        Self {
            window_info,
            buff_timers: BuffTimers::default(),
            buff_animation_state: BuffBarAnimationState::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuffTimers {
    pub battle_orders: BuffTimer,
    pub battle_command: BuffTimer,
}

impl Default for BuffTimers {
    fn default() -> Self {
        Self {
            battle_orders: BuffTimer::default(),
            battle_command: BuffTimer::default(),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct BuffTimer {
    pub timer: Instant,
    pub duration_secs: u64,
    pub expiration: Instant,
    pub level: u8,
    pub buff_type: Missile
}

impl BuffTimer {
    pub fn new(level: u8, buff_type: Missile, synergy1: Option<&PlayerSkill>, synergy2: Option<&PlayerSkill>) -> Self {
        let duration_secs = calculate_expiration(&buff_type, level, synergy1, synergy2);
        Self {
            timer: Instant::now(),
            duration_secs,
            expiration: Instant::now() + Duration::from_secs(duration_secs),
            level,
            buff_type
        }
    }
}

impl Default for BuffTimer {
    fn default() -> Self {
        Self {
            timer: Instant::now(),
            duration_secs: 0,
            expiration: Instant::now(),
            level: 0,
            buff_type: Missile::Unknown
        }
    }
}

pub fn check_buff_timers(game_data: &GameData, buff_timers: &mut BuffTimers) {
    let old_buff_timers = buff_timers.clone();
    
    if game_data.roster_items.is_empty() {
        return;
    }
    
    let current_player = game_data.roster_items.get(0).unwrap();
    
    for missile in game_data.missiles.iter() {
        if missile.collided && missile.missile_data.skill_level > 0 {
            if missile.missile_data.dw_owner_id != game_data.player.unit_id {
                let roster = game_data.roster_items.iter().find(|roster| roster.unit_id == missile.missile_data.dw_owner_id);
                let is_in_party = match roster {
                    Some(r) => { r.party_id == current_player.party_id && r.party_id != u16::MAX },
                    None => { false },
                };
                if !is_in_party { continue }
            }

            if missile.txt_file_no == Missile::Battleorders {
                let (shout, battle_command) = if game_data.player.unit_id == missile.missile_data.dw_owner_id {
                    (
                        game_data.player.skills.iter().find(|skill| skill.skill == Skill::Shout),
                        game_data.player.skills.iter().find(|skill| skill.skill == Skill::BattleCommand),
                    )
                } else {
                    (None, None)
                };
                buff_timers.battle_orders = BuffTimer::new(missile.missile_data.skill_level, Missile::Battleorders, shout, battle_command);
            }
            if missile.txt_file_no == Missile::Battlecommand {
                let (shout, battle_orders) = if game_data.player.unit_id == missile.missile_data.dw_owner_id {
                    (
                        game_data.player.skills.iter().find(|skill| skill.skill == Skill::Shout),
                        game_data.player.skills.iter().find(|skill| skill.skill == Skill::BattleOrders),
                    )
                } else {
                    (None, None)
                };
                buff_timers.battle_command = BuffTimer::new(missile.missile_data.skill_level, Missile::Battlecommand, shout, battle_orders);
            }
        }
    };

    if buff_timers.battle_orders != old_buff_timers.battle_orders {
        log::info!("Battle Orders timer restarted, level {:?} duration {:?}", buff_timers.battle_orders.level, buff_timers.battle_orders.duration_secs);
    }
    if buff_timers.battle_command != old_buff_timers.battle_command {
        log::info!("Battle Command timer restarted, level {:?} duration {:?}", buff_timers.battle_command.level, buff_timers.battle_command.duration_secs);
    }
}

fn calculate_expiration(buff_type: &Missile, level: u8, synergy1: Option<&PlayerSkill>, synergy2: Option<&PlayerSkill>) -> u64 {
    match buff_type {
        Missile::Battleorders => {
            let mut duration = match level {
                0 => 0,
                1 => 30, 2 => 40, 3 => 50, 4 => 60, 5 => 70, 6 => 80, 7 => 90, 8 => 100, 9 => 110, 10 => 120,
                11 => 130, 12 => 140, 13 => 150, 14 => 160, 15 => 170, 16 => 180, 17 => 190, 18 => 200, 19 => 210, 20 => 220,
                21 => 230, 22 => 240, 23 => 250, 24 => 260, 25 => 270, 26 => 280, 27 => 290, 28 => 300, 29 => 310, 30 => 320,
                31..=35 => 350, 36 => 380, 37 => 390, 38 => 400, 39 => 410, 40 => 420, 41 => 430, 42 => 440, 43 => 450, 44 => 460,
                45..=u8::MAX => 470
            };
            if synergy1.is_some() {
                let shout: &PlayerSkill = synergy1.unwrap();
                if shout.hard_points > 0 { duration = duration + (shout.hard_points as u64 * 5); }
            }
            if synergy2.is_some() {
                let battle_command: &PlayerSkill = synergy2.unwrap();
                if battle_command.hard_points > 0 { duration = duration + (battle_command.hard_points as u64 * 5); }
            }
            return duration
        },
        Missile::Battlecommand => {
            let mut duration = match level {
                0 => 0, 1 => 30, 2 => 40, 3 => 50, 4 => 60, 5 => 70, 6 => 80, 7 => 90, 8 => 100, 9 => 110, 10 => 120,
                11 => 130, 12 => 140, 13 => 150, 14 => 160, 15 => 170, 16 => 180, 17 => 190, 18 => 200, 19 => 210, 20 => 220,
                20..=24 => 220, 25..=u8::MAX => 270
            };
            if synergy1.is_some() {
                let shout: &PlayerSkill = synergy1.unwrap();
                if shout.hard_points > 0 { duration = duration + (shout.hard_points as u64 * 5); }
            }
            if synergy2.is_some() {
                let battle_orders: &PlayerSkill = synergy2.unwrap();
                if battle_orders.hard_points > 0 { duration = duration + (battle_orders.hard_points as u64 * 5); }
            }
            return duration
        },
        _ => { return 0 }
    }
}
