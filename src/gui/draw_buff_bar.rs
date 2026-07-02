use std::collections::HashMap;
use std::time::Instant;
use std::time::SystemTime;

use notan::draw::*;
use notan::prelude::*;

use crate::memory::gamedata::GameData;
use crate::settings::Settings;
use crate::types::buffs::BuffInstance;
use crate::types::buffs::BuffTimer;
use crate::types::player::PlayerUnit;
use crate::types::states::State;
use crate::types::stats::Stat;
use crate::types::stats::StatEnum;

use super::Fonts;

pub fn draw_buff_bar(draw: &mut Draw, game_data: &GameData, settings: &Settings, all_fonts: &Fonts, buff_bar: &mut BuffInstance, skill_popover_visible: bool, width: &u32, height: &u32, images: &HashMap<String, Texture>) {
    if !settings.buffbar.enabled {
        return;
    }
    if skill_popover_visible {
        return;
    }
    let width = *width as f32;
    let height = *height as f32;
    let icon_size = (1.0 / settings.buffbar.icon_scale) * height;

    
    buff_bar.buff_animation_state.update(&game_data.player);

    if !buff_bar.buff_animation_state.buff_icons.is_empty() {
        let mut x = ((width / 2.0) - (buff_bar.buff_animation_state.buff_icons.len() as f32 * icon_size) / 2.0) * (settings.buffbar.horizontal_pos * 2.0);
        let y = height * settings.buffbar.vertical_pos;
        for state_icon in buff_bar.buff_animation_state.buff_icons.iter() {
            let color = match state_icon.buff_group {
                BuffGroup::Debuff => Color::RED,
                BuffGroup::Buff => Color::GREEN,
                BuffGroup::Aura => Color::YELLOW,
                BuffGroup::Passive => Color::GRAY,
                BuffGroup::Charge => Color::PURPLE,
            };
            match images.get(&state_icon.image_name) {
                Some(image) => {
                    if state_icon.charges > 0 {
                        draw_charge_text(draw, settings, x, y, icon_size, all_fonts, &state_icon.charges);
                    }
                    if state_icon.removing == true && state_icon.buff_group == BuffGroup::Buff {
                        // do the removal animation here
                        if state_icon.animation_frame % 2 == 0 {
                            draw.rect((x - 2.5, y - 2.5), (icon_size + 5.0, icon_size + 5.0)).color(Color::RED);
                        }
                        draw.image(image).position(x, y).size(icon_size, icon_size).alpha(0.8);
                    } else if state_icon.removing == false {
                        
                        draw.rect((x - 1.0, y - 1.0), (icon_size + 2.0, icon_size + 2.0)).color(color);
                        draw.image(image).position(x, y).size(icon_size, icon_size);
                        
                        if state_icon.state == State::BattleOrders { draw_timer_text(draw, settings, x, y, icon_size, all_fonts, &mut buff_bar.buff_timers.battle_orders); }
                        if state_icon.state == State::BattleCommand { draw_timer_text(draw, settings, x, y, icon_size, all_fonts, &mut buff_bar.buff_timers.battle_command); }

                    }
                    x = x + icon_size + 3.0;
                }
                None => {
                    // 如果看到了这一行报错，说明你的图片名字没改对或者没放对位置！
                    log::info!("Found unknown state icon {}", &state_icon.image_name);
                },
            };
            
        }
    }
}

pub fn draw_timer_text(draw: &mut Draw, settings: &Settings, x: f32, y: f32, icon_size: f32, all_fonts: &Fonts, buff_timer: &BuffTimer) {
    if buff_timer.expiration > Instant::now() {
        let seconds_remaining = buff_timer.expiration.duration_since(Instant::now()).as_secs_f32().round();
        if seconds_remaining > 0.0 {
            let (font_size, font_color) = if seconds_remaining < 10.0 { (settings.buffbar.timer_font_size * 1.5, Color::RED) } else { (settings.buffbar.timer_font_size, Color::WHITE) };
            draw.text(&all_fonts.formal_font, &seconds_remaining.to_string()).position(x + (icon_size / 2.0) + 1.0, y - font_size - 3.0 + 1.0).size(font_size).h_align_center().color(Color::BLACK);
            draw.text(&all_fonts.formal_font, &seconds_remaining.to_string()).position(x + (icon_size / 2.0), y - font_size - 3.0).size(font_size).h_align_center().color(font_color);
        }
    }
}

pub fn draw_charge_text(draw: &mut Draw, settings: &Settings, x: f32, y: f32, icon_size: f32, all_fonts: &Fonts, charges: &i16) {
    let font_size = settings.buffbar.charges_font_size;
    let font_color = Color::WHITE;
    draw.text(&all_fonts.formal_font, &charges.to_string()).position(x + (icon_size / 2.0) + 1.0, y - font_size - 3.0 + 1.0).size(font_size).h_align_center().color(Color::BLACK);
    draw.text(&all_fonts.formal_font, &charges.to_string()).position(x + (icon_size / 2.0), y - font_size - 3.0).size(font_size).h_align_center().color(font_color);
}

#[derive(Default, Debug, Clone)]
pub struct BuffBarAnimationState {
    pub buff_icons: Vec<BuffIcon>
}

impl BuffBarAnimationState {
    pub fn update(&mut self, player: &PlayerUnit) {
        for state in player.states.iter() {
            if state != &State::None {
                match get_buff_bar_icon(state) {
                    Some(new_icon) => {
                        if self.buff_icons.iter_mut().find(|icon| icon.state == *state).is_none() {
                            self.buff_icons.push(new_icon);
                        }
                    },
                    None => ()
                }
            }
        }
        for stat in player.stats.iter() {
            match get_buff_bar_icon_from_stat(&stat) {
                Some((new_icon, state)) => {
                    match self.buff_icons.iter_mut().find(|icon| icon.state == state) {
                        Some(icon) => icon.charges = stat.value, 
                        None => self.buff_icons.push(new_icon)  
                    }
                },
                None => ()
            }
        }
        self.buff_icons.sort_unstable_by(|a, b| (a.buff_group, &a.image_name).cmp(&(b.buff_group, &b.image_name)));
        self.buff_icons.dedup();
        for buff_icon in &mut self.buff_icons {
            if buff_icon.removing { buff_icon.animation_frame += 1; }
        }
        self.buff_icons.retain(|buff_icon| buff_icon.animation_frame <= 100);
        
        for buff_icon in self.buff_icons.iter_mut() {
            if player.states.iter().find(|state| *state == &buff_icon.state).is_none() {
                if buff_icon.removing == false {
                    buff_icon.removing = true;
                    buff_icon.started = SystemTime::now();
                }
            } else {
                if buff_icon.removing == true {
                    buff_icon.removing = false;
                    buff_icon.animation_frame = 0;
                    buff_icon.started = SystemTime::now();
                }
            }
        }
        self.buff_icons.retain(|buff_icon| (buff_icon.buff_group != BuffGroup::Buff && buff_icon.removing == false) || buff_icon.buff_group == BuffGroup::Buff);
    }
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct BuffIcon {
    pub image_name: String,
    pub buff_group: BuffGroup,
    pub removing: bool,
    pub state: State,
    pub animation_frame: u32,
    pub charges: i16,
    pub started: SystemTime,
}

impl BuffIcon {
    pub fn new(state: State, buff_group: BuffGroup, removing: bool, charges: i16) -> Self {
        Self {
            image_name: state.to_string(),
            buff_group,
            removing,
            state,
            animation_frame: 0,
            charges,
            started: SystemTime::now(),
        }
    }
}

pub fn get_buff_bar_icon_from_stat(stat: &Stat) -> Option<(BuffIcon, State)> {
    match stat.stat {
        StatEnum::ProgressiveDamage => Some((BuffIcon::new(State::TigerStrike, BuffGroup::Charge, false, stat.value), State::TigerStrike)),
        StatEnum::ProgressiveSteal => Some((BuffIcon::new(State::CobraStrike, BuffGroup::Charge, false, stat.value), State::CobraStrike)),
        StatEnum::ProgressiveOther => Some((BuffIcon::new(State::PhoenixStrike, BuffGroup::Charge, false, stat.value), State::PhoenixStrike)),
        StatEnum::ProgressiveFire => Some((BuffIcon::new(State::FistsOfFire, BuffGroup::Charge, false, stat.value), State::FistsOfFire)),
        StatEnum::ProgressiveCold => Some((BuffIcon::new(State::BladesOfIce, BuffGroup::Charge, false, stat.value), State::BladesOfIce)),
        StatEnum::ProgressiveLightning => Some((BuffIcon::new(State::ClawsOfThunder, BuffGroup::Charge, false, stat.value), State::ClawsOfThunder)),
        StatEnum::SkillFrenzy => Some((BuffIcon::new(State::FeralRage, BuffGroup::Charge, false, stat.value), State::FeralRage)),
        _ => None
    }
}

pub fn get_buff_bar_icon(state: &State) -> Option<BuffIcon> {
    match state {
        State::ResistFire | State::ResistCold | State::ResistLight | State::ResistAll |
        State::Conviction | State::Might | State::Prayer | State::HolyFire |
        State::Thorns | State::Defiance | State::BlessedAim | State::Stamina |
        State::Concentration | State::HolyWind | State::HolyWindCold | State::Cleansing |
        State::HolyShock | State::Sanctuary | State::Meditation | State::Fanaticism |
        State::Redemption | State::Barbs | State::Wolverine | State::OakSage => {
            Some(BuffIcon::new(state.clone(), BuffGroup::Aura, false, 0))
        },
        State::FrozenArmor | State::Inferno | State::Blaze | State::BoneArmor |
        State::Enchant | State::InnerSight | State::ChillingArmor | State::Shout |
        State::EnergyShield | State::VenomClaws | State::BattleOrders | State::Thunderstorm |
        State::BattleCommand | State::SlowMissiles | State::ShiverArmor | State::Valkyrie |
        State::Frenzy | State::Berserk | State::HolyShield | State::ShadowWarrior |
        State::FeralRage | State::Wolf | State::Bear | State::Hurricane |
        State::Armageddon | State::CycloneArmor | State::CloakOfShadows | State::Cloaked |
        State::Quickness | State::Bladeshield | State::Fade |
        // 👇 注册增益图标 👇
        State::HexBane | State::HexPurge | State::HexSiphon | State::Consume => { 
        Some(BuffIcon::new(state.clone(), BuffGroup::Buff, false, 0))
    },
        State::Poison | State::AmplifyDamage | State::Cold | State::Weaken |
        State::DimVision | State::Slowed | State::Convicted | State::Conversion |
        State::IronMaiden | State::Terror | State::Attract | State::LifeTap |
        State::Confuse | State::Decrepify | State::LowerResist | State::DefenseCurse |
        State::BloodMana => {
            Some(BuffIcon::new(state.clone(), BuffGroup::Debuff, false, 0))
        },
        _ => None
    }
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum BuffGroup {
    Debuff, Buff, Aura, Passive, Charge
}
