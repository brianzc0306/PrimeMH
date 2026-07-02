use std::mem::transmute;

use derivative::Derivative;

use crate::memory::{
    process::D2RInstance,
    structs::{Unit, StatsList},
};

use super::{get_position, skills::{get_player_skills, PlayerSkill}, states::{self, State}, stats::{read_stats, Stat, StatEnum}};

#[allow(dead_code)]
#[derive(Derivative, Debug, Clone)]
#[derivative(Default)]
pub struct PlayerUnit {
    pub unit_id: u32,
    pub mode: PlayerMode,
    pub player_name: String,
    pub pos_x: f32,
    pub pos_y: f32,
    // 👇 扩容：将 192 改为 256，支持 Mod 的超长状态数组
    #[derivative(Default(value = "[State::None; 256]"))]
    pub states: [State; 256],
    pub stats: Vec<Stat>,
    pub player_class: PlayerClass,
    pub skills: Vec<PlayerSkill>,
    pub is_corpse: bool,
    pub raw: Unit,
}

impl PlayerUnit {
    pub fn new(d2rprocess: &D2RInstance, unit: Unit) -> Self {
        let mode: PlayerMode = unsafe { transmute::<u32, PlayerMode>(unit.mode as u32) };
        let player_class: PlayerClass = unsafe { transmute::<u32, PlayerClass>(unit.player_class as u32) };
        let (pos_x, pos_y) = get_position(d2rprocess, unit);
        let states = Self::get_states(d2rprocess, unit);
        let stats = Self::get_stats(d2rprocess, unit);
        let mut skills = vec![];    
        if states[State::SharedStash as usize] != State::SharedStash {
            skills = get_player_skills(d2rprocess, unit.p_skills);
        }
        
        let player_arr1 = d2rprocess.read_mem::<[u8; 24]>(unit.p_unit_data);
        let player_arr2 = d2rprocess.read_mem::<[u8; 24]>(unit.p_unit_data + 24);
        let mut player_arr: [u8; 48] = [0; 48];
        player_arr[0..player_arr1.len()].copy_from_slice(&player_arr1);
        player_arr[player_arr1.len()..48].copy_from_slice(&player_arr2);
        let player_name = d2rprocess.parse_arr_to_string(&player_arr);
        
        PlayerUnit {
            unit_id: unit.unit_id,
            player_name,
            mode,
            pos_x,
            pos_y,
            states,
            stats,
            player_class,
            skills,
            is_corpse: unit.is_corpse != 0,
            raw: unit,
        }
    }

    // 👇 扩容：将返回值 192 改为 256
    pub fn get_states(d2rprocess: &D2RInstance, unit: Unit) -> [State; 256] {
        if unit.p_stats_list_ex == 0 {
            [State::None; 256] // 👇 扩容：兜底值 192 改为 256
        } else {
            let stat_list: StatsList = d2rprocess.read_mem::<StatsList>(unit.p_stats_list_ex);
            let state_flags = stat_list.state_flags;
            states::parse_state_flags(state_flags)
        }
    }

    #[allow(unused)]
    pub fn has_state(&self, state: State) -> bool {
        self.states[state as usize] == state
    }

    pub fn get_stats(d2rprocess: &D2RInstance, unit: Unit) -> Vec<Stat> {
        if unit.p_stats_list_ex == 0 {
            vec![]
        } else {
            read_stats(d2rprocess, &unit)
        }
    }

    #[allow(unused)]
    pub fn get_health(&self) -> Option<(u16, u16)> {
        let health: Vec<&Stat> = self.stats.iter().filter(|s: &&Stat| s.stat == StatEnum::Life).collect();
        let max_health: Vec<&Stat> = self.stats.iter().filter(|s: &&Stat| s.stat == StatEnum::MaxLife).collect();
        let actual_health: u16 = match health.iter().map(|s| ((s.value2 as u16) << 8) ^ (s.value as u8) as u16).max() {
            Some(actual_health) => actual_health,
            None => return None,
        };
        let actual_max_health: u16 = match max_health.iter().map(|s| ((s.value2 as u16) << 8) ^ (s.value as u8) as u16).max() {
            Some(actual_max_health) => actual_max_health,
            None => return None,
        };
        Some((actual_health, actual_max_health))
    }
    
}


// play matches the first player unit id in the roster
pub fn get_current_player(players: &[PlayerUnit], player_unit_id: u32) -> Option<&PlayerUnit> {
    players
        .iter()
        .find(|&p| p.pos_x > 0.0 && p.pos_y > 0.0 && p.unit_id == player_unit_id as u32)
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Clone, Default)]
pub enum PlayerClass {
    Amazon,
    Sorceress,
    Necromancer,
    Paladin,
    Barbarian,
    Expansion,
    Druid,
    Assassin,
    #[default]
    Unknown,
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Clone, Default)]
pub enum PlayerMode {
    Death,
    Neutral,
    Walk,
    Run,
    GetHit,
    TownNeutral,
    TownWalk,
    Attack1,
    Attack2,
    Block,
    Cast,
    Throw,
    Kick,
    Skill1,
    Skill2,
    Skill3,
    Skill4,
    Dead,
    Sequence,
    Knockback,
    #[default]
    Unknown,
}


impl From<(&D2RInstance, Unit)> for PlayerUnit {
    fn from(data: (&D2RInstance, Unit)) -> Self {
        PlayerUnit::new(&data.0, data.1)
    }
}
