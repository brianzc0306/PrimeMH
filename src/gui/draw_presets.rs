use std::collections::HashMap;
use notan::draw::*;
use notan::prelude::*;
use crate::mapgeneration::jsondata::LevelData;
use crate::mapgeneration::pois::{POIType, POI};
use crate::memory::gamedata::GameData;
use crate::settings::Settings;
use crate::types::object::{GameObjectMode, GameObjectType, ShrineType};
use crate::LOCALISATION;
use super::Fonts;
use super::draw_objects::transform_position;

pub fn draw_presets(draw: &mut Draw, this_level: &mut LevelData, all_fonts: &Fonts, game_data: &GameData, settings: &Settings, images: &HashMap<String, Texture>, width: &f32, height: &f32) {
    let player_pos = (game_data.player.pos_x, game_data.player.pos_y);
    let current_level_id = game_data.seed_values.level;

    let shrine_image = images.get("shrine").unwrap();
    let well_image = images.get("well").unwrap();
    let super_chest_image = images.get("superchest").unwrap();

    for poi in &mut this_level.level_image.pois.iter_mut() {
        for chest in game_data.objects.iter() {
            if chest.pos_x == poi.world_x && chest.pos_y == poi.world_y {
                // 【核心除障】：只要内存加载了真实的宝箱，立刻干掉底层占位的白底原图！
                // 这样 draw_objects.rs 里的绿色宝箱就不会和它叠加冲突了。
                if chest.object_type == GameObjectType::Chest || chest.object_type == GameObjectType::SuperChest {
                    poi.poi_type = POIType::Unknown;
                }
            }
        }
    }
    
    for shrine in game_data.objects.iter() {
        let mut found = false;
        let mut is_well = false;
        for poi in &mut this_level.level_image.pois.iter_mut() {
            if shrine.object_type == GameObjectType::Shrine && shrine.pos_x == poi.world_x && shrine.pos_y == poi.world_y {
                found = true;
                poi.label = match shrine.shrine_type {
                    Some(a) => {
                        let localisation = LOCALISATION.lock().unwrap();
                        localisation.get_shrine(a as usize)
                    },
                    None => String::new(),
                };
            }
            if shrine.object_type == GameObjectType::Well && shrine.pos_x == poi.world_x && shrine.pos_y == poi.world_y {
                found = true;
                is_well = true;
                poi.label = String::new();
                poi.poi_type = POIType::Well;
            }
        }
        if !found {
            match shrine.shrine_type {
                Some(shrine_type) => {
                    if shrine_type != ShrineType::None {
                        if is_well {
                            let label = shrine_type.to_string();
                            let new_well = POI::new_well(shrine.pos_x, shrine.pos_y, &this_level.offset, label);
                            this_level.level_image.pois.push(new_well);
                        } else {
                            let label = shrine_type.to_string();
                            let new_shrine = POI::new_shrine(shrine.pos_x, shrine.pos_y, &this_level.offset, label);
                            this_level.level_image.pois.push(new_shrine);
                        }
                    }
                }
                None => (),
            }
        }
    }

    let pois = &this_level.level_image.pois;
    for poi in pois.iter() {
        match poi.poi_type {
            POIType::Waypoint => draw_waypoint(poi, player_pos, draw, settings.visual.scale, width, height),
            POIType::Shrine => {
                if settings.shrines.enabled {
                    draw_shrine(poi, player_pos, draw, settings, shrine_image, width, height, all_fonts);
                }
            }
            POIType::Well => {
                if settings.shrines.enabled {
                    draw_shrine(poi, player_pos, draw, settings, well_image, width, height, all_fonts);
                }
            }
            POIType::Chest => (),
            POIType::SuperChest => {
                if settings.chests.enabled {
                    draw_super_chest_preset(poi, player_pos, draw, settings, super_chest_image, width, height);
                }
            }
            POIType::Exit => draw_exit(poi, player_pos, this_level, draw, settings.visual.scale, current_level_id, width, height, all_fonts, settings),
            POIType::GoodExit => draw_good_exit(poi, player_pos, this_level, draw, settings.visual.scale, width, height),
            POIType::QuestItem => draw_quest_item(poi, player_pos, draw, settings.visual.scale, width, height),
            POIType::NPCSpawn => draw_npc_spawn(poi, player_pos, draw, settings.visual.scale, width, height),
            POIType::Unknown => (),
        }
    }
}

fn draw_waypoint(poi: &POI, player_pos: (f32, f32), draw: &mut Draw, scale: f32, width: &f32, height: &f32) {
    let size = (6.0 * scale, 3.0 * scale);
    let unit_pos = (poi.world_x as u32, poi.world_y as u32);
    let poi_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
    draw_diamond(draw, poi_pos, size, Color::YELLOW);
}

fn draw_exit(poi: &POI, player_pos: (f32, f32), this_level: &LevelData, draw: &mut Draw, scale: f32, current_level_id: u32, width: &f32, height: &f32, all_fonts: &Fonts, settings: &Settings) {
    if poi.id > 0 {
        let size = (6.0 * scale, 3.0 * scale);
        let unit_pos = (poi.world_x as u32, poi.world_y as u32);
        let poi_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
        draw_diamond(draw, poi_pos, size, Color::from_rgb(255.0, 0.0, 255.0));

        let localisation = LOCALISATION.lock().unwrap();
        let label: String = localisation.get_level(&poi.id);

        if current_level_id == this_level.id || poi.class != "walkable" {
            let font = all_fonts.get_safe_font(&settings.general.language);
            let text_pos = (poi_pos.0 + (size.0 / 2.0), (poi_pos.1 + (size.1 / 2.0)) - (6.0 * scale));
            draw.text(font, &label).position(text_pos.0 + 2.0, text_pos.1 + 2.0).size(settings.visual.exit_label_text_size * scale).color(Color::BLACK).h_align_center().v_align_middle();
            draw.text(font, &label).position(text_pos.0, text_pos.1).size(settings.visual.exit_label_text_size * scale).color(Color::WHITE).h_align_center().v_align_middle();
        }
    }
}

fn draw_good_exit(poi: &POI, player_pos: (f32, f32), _this_level: &LevelData, draw: &mut Draw, scale: f32, width: &f32, height: &f32) {
    let size = (8.0 * scale, 4.0 * scale);
    let unit_pos = (poi.world_x as u32, poi.world_y as u32);
    let poi_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
    draw_diamond(draw, poi_pos, size, Color::from_rgb(0.0, 255.0, 0.0));
}

fn draw_quest_item(poi: &POI, player_pos: (f32, f32), draw: &mut Draw, scale: f32, width: &f32, height: &f32) {
    let size = (1.0 * scale, 1.0 * scale);
    let unit_pos = (poi.world_x as u32, poi.world_y as u32);
    let poi_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
    draw.rect(poi_pos, size).color(Color::from_rgb(0.0, 255.0, 0.0));
}

fn draw_super_chest_preset(poi: &POI, player_pos: (f32, f32), draw: &mut Draw, settings: &Settings, super_chest_image: &Texture, width: &f32, height: &f32) {
    let scale = settings.visual.scale;
    let h = (super_chest_image.height() * settings.chests.size) * scale;
    let w = (super_chest_image.width() * settings.chests.size) * scale;
    
    let unit_pos = (poi.world_x as u32, poi.world_y as u32);
    let poi_pos = transform_position(unit_pos, (w, h), player_pos, scale, width, height);
    
    draw.image(super_chest_image).size(w, h).position(poi_pos.0, poi_pos.1);
}

fn draw_npc_spawn(poi: &POI, player_pos: (f32, f32), draw: &mut Draw, scale: f32, width: &f32, height: &f32) {
    let size = (1.0 * scale, 1.0 * scale);
    let unit_pos = (poi.world_x as u32, poi.world_y as u32);
    let poi_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
    draw.rect(poi_pos, size).color(Color::from_rgb(255.0, 0.0, 0.0));
}

fn draw_shrine(poi: &POI, player_pos: (f32, f32), draw: &mut Draw, settings: &Settings, image: &Texture, width: &f32, height: &f32, all_fonts: &Fonts) {
    let scale = settings.visual.scale;
    let unit_pos = (poi.world_x as u32, poi.world_y as u32);
    let h = (image.height() * settings.shrines.size) * scale;
    let w = (image.width() * settings.shrines.size) * scale;
    let poi_pos = transform_position(unit_pos, (w, h), player_pos, scale, width, height);
    
    draw.image(image).size(w, h).position(poi_pos.0, poi_pos.1);
            
    if poi.poi_type != POIType::Well {
        let font = all_fonts.get_safe_font(&settings.general.language);
        let text_pos = (poi_pos.0 + (w / 2.0), (poi_pos.1 - (10.0 * scale)));
        draw.text(font, &poi.label).position(text_pos.0 + 1.5, text_pos.1 + 1.5).size(settings.shrines.text_size * scale).color(Color::BLACK).h_align_center().v_align_top();
        draw.text(font, &poi.label).position(text_pos.0, text_pos.1).size(settings.shrines.text_size * scale).color(Color::from_hex(0xFFD700FF)).h_align_center().v_align_top();
    }
}

fn draw_diamond(draw: &mut Draw, poi_pos: (f32, f32), size: (f32, f32), color: Color) {
    let pos_x = poi_pos.0 + (size.0 / 2.0);
    let pos_y = poi_pos.1 + (size.1 / 2.0);
    draw.path().move_to(pos_x, pos_y + size.1).line_to(pos_x + size.0, pos_y).line_to(pos_x, pos_y - size.1).line_to(pos_x - size.0, pos_y).line_to(pos_x, pos_y + size.1).color(color).stroke(1.0).fill();
}
