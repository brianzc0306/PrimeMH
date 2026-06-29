use std::sync::Mutex;
use std::time::Instant;

use derivative::Derivative;
use notan::draw::*;
use notan::prelude::*;
use lazy_static::lazy_static;
use linked_hash_set::LinkedHashSet;
use sapi_lite::tts::SyncSynthesizer;
use std::thread;

use crate::memory::gamedata::GameData;
use crate::settings::Settings;
use crate::types::item::BaseItem;
use crate::types::item::ItemMode;
use crate::types::item::ItemUnit;
use crate::types::item::Quality;
use crate::types::item_filter::ItemFilters;
use super::play_sound::play_sound;
use super::util::draw_text;
use super::util::draw_text_left;

lazy_static! {
    static ref ITEM_LOG: Mutex<LinkedHashSet<ItemLogEntry>> = Mutex::new(LinkedHashSet::with_capacity(40));
}

#[derive(Derivative)]
#[derivative(Hash, PartialEq, Eq)]
pub struct ItemLogEntry {
    unit_id: u32,
    txt_file_no: BaseItem,
    is_vendor_item: bool,
    vendor_cached_item: ItemUnit,
    #[derivative(Hash = "ignore", PartialEq = "ignore")]
    time_stamp: Instant,
}

pub fn clear_item_log() {
    let mut item_log = ITEM_LOG.lock().unwrap();
    item_log.clear();
}

pub fn draw_item_log(
    draw: &mut Draw,
    game_data: &GameData,
    settings: &Settings,
    width: &f32, 
    height: &f32,
    exocet_font: &Font,
    item_frame: i32,
    item_filters: &ItemFilters
) {
    let mut item_log = ITEM_LOG.lock().unwrap();

    // apply filter
    game_data.items.iter().filter(|item| (item.mode == ItemMode::OnGround || item.mode == ItemMode::Dropping) || item.is_vendor_item()).for_each(|item| {
        
        if !item_log.iter().any(|log| log.unit_id == item.unit_id && log.txt_file_no == item.txt_file_no) { // if not already added
            let (matched, sound_file, vendor_item) = item_filters.match_filter(item);
            if matched {
                let this_item = ItemLogEntry {
                    unit_id: item.unit_id,
                    txt_file_no: item.txt_file_no.clone(),
                    is_vendor_item: item.is_vendor_item(),
                    vendor_cached_item: item.clone(),
                    time_stamp: Instant::now(),
                };
                if (item.mode == ItemMode::OnGround || item.mode == ItemMode::Dropping) || vendor_item {
                    log::info!("Checking vendor item: {} {}", item.get_item_log_name(false), item.is_vendor_item());
                    item_log.insert(this_item);
                    
                    if settings.item_log.sound_enabled {
                        if sound_file.is_some() {
                            play_sound(sound_file);
                        }
                    }
                    
                    if settings.item_log.voice_enabled {
                        let item_text_raw = item.get_tts_description().clone();
                        let item_text_cn = translate_tts_to_cn(&item_text_raw);

                        // 如果还有英文字母，说明没翻译全：不播英文，改成中文通用提示，并写入文件
                        let item_text = if has_english_letters(&item_text_cn) {
                            save_untranslated_tts(&item_text_raw);
                            format!("发现未翻译物品，{}", guess_item_type_cn(&item_text_raw))
                        } else {
                            item_text_cn
                        };

                        let speech_volume = settings.item_log.voice_volume.clone();
                        let speech_rate = settings.item_log.voice_speed.clone();
                        
                        // text to speech needs to be async
                        thread::spawn(move || {
                            sapi_lite::initialize().unwrap_or_default();
                            match SyncSynthesizer::new() {
                                Ok(synth) => {
                                    synth.set_volume(speech_volume).unwrap_or_default();
                                    synth.set_rate(speech_rate).unwrap_or_default();
                                    match synth.speak(item_text, None) {
                                        Ok(_) => (),
                                        Err(e) => log::debug!("Text to speech error: {:?}", e)
                                    }
                                    sapi_lite::finalize();
                                },
                                Err(e) => {
                                    log::debug!("Text to speech error: {:?}", e);
                                    sapi_lite::finalize();
                                }
                            };
                        });
                    }
                }
            }
        }
    });
    
    let player_pos = (game_data.player.pos_x, game_data.player.pos_y);
    let scale = settings.visual.scale;
    let itemx: f32 = 10.0;
    let mut itemy: f32 = 50.0;

    // draw vendor items seaprately, since they disappear from the unit table when you exit the vendor
    item_log.iter().filter(|p| p.is_vendor_item).for_each(|item_log_entry| {
            
        if settings.item_log.enabled {
            if item_log_entry.time_stamp.elapsed().as_secs() < settings.item_log.text_duration as u64 {
                let text_color = get_quality_color(&item_log_entry.vendor_cached_item);

                let item_text: String = item_log_entry.vendor_cached_item.get_item_log_name(settings.item_log.ground_alerts_show_suffix_prefix);
                draw_text_left(
                    draw,
                    exocet_font,
                    &item_text,
                    itemx,
                    itemy,
                    settings.item_log.text_size,
                    text_color,
                    true,
                    true,
                );
                itemy += settings.item_log.text_size + 3.0
            }
        }
    });
    

    // draw non-vendor items
    for item in game_data.items.iter().filter(|item| item.mode == ItemMode::OnGround || item.mode == ItemMode::Dropping) {
        let item_log_entry = item_log.iter().find(|p| p.unit_id == item.unit_id && p.txt_file_no == item.txt_file_no);
        match item_log_entry {
            Some(item_log_entry) => {
                
                // draw item log at top left
                if settings.item_log.enabled {
                    if item_log_entry.time_stamp.elapsed().as_secs() < settings.item_log.text_duration as u64 {
                        let text_color = get_quality_color(&item);

                        let item_text = item.get_item_log_name(settings.item_log.ground_alerts_show_suffix_prefix);
                        draw_text_left(
                            draw,
                            exocet_font,
                            &item_text,
                            itemx,
                            itemy,
                            settings.item_log.text_size,
                            text_color,
                            true,
                            true,
                        );
                        itemy += settings.item_log.text_size + 3.0
                    }
                }
                
                if settings.item_log.ground_alerts { // draw ground alert with animated dot
                    match item.mode {
                        ItemMode::OnGround | ItemMode::Dropping => {
                            let item_pos = transform_position((item.pos_x, item.pos_y), player_pos, scale, width, height);
                            let item_text_pos = (item_pos.0, item_pos.1 - (2.0 * scale));
                            let item_text = item.get_item_ground_alert_name(settings.item_log.ground_alerts_show_suffix_prefix);
                            let text_color = get_quality_color(item);
                            let (dot_size, dot_trans) = {
                                let alpha = 1.0 - (item_frame as f32 * (100.0 / 20.0) / 100.0);
                                let size = item_frame as f32 / 5.0;
                                (size, alpha)
                            };

                            draw.circle(dot_size as f32 * scale)
                                .position(item_pos.0, item_pos.1)
                                .color(text_color)
                                .alpha(dot_trans);
                            draw.circle(0.5 * scale)
                                .position(item_pos.0, item_pos.1)
                                .color(text_color);

                            let font_size = settings.item_log.ground_alerts_text_size * scale;
                            draw_text(
                                draw,
                                exocet_font,
                                &item_text,
                                item_text_pos.0,
                                item_text_pos.1,
                                font_size,
                                text_color,
                                true,
                                true,
                            );
                        }
                        _ => (),
                    }
                }
            }
            None => (),
        }
    }
}

fn get_quality_color(item: &ItemUnit) -> Color {
    match item.quality {
        Quality::Magic => Color::from_hex(0x6D6DFFFF),
        Quality::Set => Color::from_hex(0x00FC00FF),
        Quality::Rare => Color::from_hex(0xFFDD00FF),
        Quality::Unique => Color::from_hex(0xBBA45BFF),
        Quality::Crafted => Color::from_hex(0xFFFFFFFF),
        _ => {
            if item.is_ethereal() {
                Color::from_hex(0xAAAAAAFF)
            } else if item.is_rune() || item.is_key() || item.is_essence() {
                Color::from_hex(0xFFA700FF)
            } else {
                Color::from_hex(0xFFFFFFFF)
            }
        }
    }
}

fn transform_position(unit_pos: (u32, u32), player_pos: (f32, f32), scale: f32, width: &f32, height: &f32) -> (f32, f32) {
    let xdiff = unit_pos.0 as f32 - player_pos.0;
    let ydiff = unit_pos.1 as f32 - player_pos.1;

    let center_x = *width as f32 / 2.0;
    let center_y = *height as f32 / 2.0;
    let angle: f32 = std::f32::consts::FRAC_PI_4;
    let x = xdiff * angle.cos() - ydiff * angle.sin();
    let y = xdiff * angle.sin() + ydiff * angle.cos();

    let new_pos_x = center_x + (x * scale);
    let new_pos_y = center_y + (y * scale * 0.5);

    (new_pos_x, new_pos_y)
}

fn translate_tts_to_cn(text: &str) -> String {
    let mut s = text.to_string();

    // 注意：这里会自动按英文长度从长到短替换，避免 Giant Thresher 先被 Thresher 抢替换
    let mut replaces: Vec<(&str, &str)> = vec![
        // =========================
        // 从 tts_untranslated.txt 补充的暗金 / 套装 / 物品 / 底材
        // =========================
        ("Unknown ", ""), // 过滤掉未识别或内存错误读取的前缀
        ("Eschutastemper", "艾丝屈塔的脾气"),
        ("Verdugos Hearty Cord", "维尔登戈的心结"),
        ("Griswoldss Redemption", "格瑞斯华尔德的救赎"),
        ("Fathom", "深度"),
        ("Perfect Amethyst", "完美紫宝石"),
        ("Perfect Sapphire", "完美蓝宝石"),
        ("Perfect Ruby", "完美红宝石"),
        ("Baals Eye", "巴尔之眼"),
        ("Mephistos Brain", "墨菲斯托之脑"),
        ("Diablos Horn", "迪亚波罗之角"),
        ("Flame Rift", "破火"),
        ("Bone Break", "破骨"),
        ("Crack Of The Heavens", "破天"),
        ("Hellfire Torch", "地狱火炬"),
        ("Annihilus", "毁灭"),
        ("Rainbow Facet Psn 2", "彩虹刻面毒系"),
        ("Rainbow Facet Psn", "彩虹刻面毒系"), 
        ("Rainbow Facet Cold 2", "彩虹刻面冰系"),
        ("Rainbow Facet Cold", "彩虹刻面冰系"),
        ("Rainbow Facet Fire", "彩虹刻面火系"),
        ("Rainbow Facet Light", "彩虹刻面电系"), 
        ("The Reapers Toll", "死神的丧钟"),
        ("Guardian Angel", "守护天使"),
        ("Alma Negra", "阿尔玛·尼格拉"),
        ("Giantskull", "巨骷髅"),
        ("Waterwalk", "水上飘"),
        ("Shaftstop", "谢夫特斯坦布"),
        ("Aldurs Advance", "艾尔多的成长"),
        ("Horizons Tornado", "地平线的台风"),
        ("Griswolds Honor", "格瑞斯华尔德的荣耀"),
        ("Skullders Ire", "诗寇蒂的愤怒"),
        ("Nokozan Relic", "诺科蓝遗物"),
        ("Bul Kathos Tribal Guardian", "布尔凯索的部落守护"),
        ("Bul Kathos Sacred Charge", "布尔凯索的神圣代价"),
        ("Lightsabre", "光之网刃"),
        ("The Grandfather", "祖父"),
        ("Titans Revenge", "泰坦的复仇"),
        ("Marrowwalk", "骨髓行走"),
        ("Irathas Collar", "伊雷萨的项圈"),
        ("Lacerator", "撕裂者"),
        ("Mosers Blessed Circle", "摩西祝福之环"),
        ("Nagelring", "拿各的戒指"),
        ("Seraphs Hymn", "炽天使之韵"),
        ("Griswolds Valor", "格瑞斯华尔德的勇气"),
        ("Windforce", "风之力"),
        ("Arkaines Valor", "阿凯尼的荣耀"),
        ("Ormus Robes", "奥玛斯的长袍"),
        ("Baezils Vortex", "巴兹尔的涡旋"),
        ("Shadowdancer", "影舞者"),
        ("Steelrend", "碎钢"),
        ("The Gladiators Bane", "角斗士的祸根"),
        ("Gorerider", "蚀肉骑士"),
        ("Tancreds Weird", "坦克雷的怪异"),
        ("Runemaster", "符文大师"),
        ("Veil Of Steel", "钢铁面纱"),
        ("Atmas Scarab", "亚特玛的圣甲虫"),
        ("Dwarf Star", "矮人之星"),
        ("Immortal Kings Soul Cage", "不朽之王的灵魂牢笼"),
        ("Cathans Sigil", "卡珊的印记"),
        ("Thunderstroke", "雷击"),
        ("Angelic Wings", "天使之翼"),
        ("Arreats Face", "亚瑞特的面容"),
        ("Kiras Guardian", "奇拉的守护"),
        ("The Cats Eye", "猫眼"),
        ("Jadetalon", "玉爪"),
        ("Lavagout", "熔岩羊角"),
        ("Shadowkiller", "影杀者"),
        ("Stormshield", "暴风之盾"),
        ("Ironward", "铁卫"),
        ("Heavens Light", "天堂之光"),
        ("Herald Of Zakarum", "撒卡兰姆使者"),
        ("Telling Of Beads", "述说之珠"),
        ("Vidalas Snare", "维达拉的陷阱"),
        ("Warshrike", "战鸟"),
        ("Duriels Shell", "都瑞尔的壳"),
        ("The Mahim Oak Curio", "马哈姆橡木古董"),
        ("Bonehew", "骨砍"),
        ("Saracens Chance", "撒拉森的机会"),
        ("Templars Might", "圣堂武士的力量"),
        ("Mang Songs Lesson", "梅格之歌的教训"), 
        ("Jalals Mane", "加尔的长发"), 
        ("Leviathan", "利维坦"), 
        ("Darkforge Spawn", "黑暗原力重生"),
        ("Feral Claws", "猛禽爪"),
        ("Winged Axe", "翼斧"),
        ("Templar Coat", "圣堂武士外衣"),
        ("Mesh Armor", "织网战甲"),
        ("Russet Armor", "罗瑟战甲"),
        ("Mythical Sword", "神话之剑"),
        ("Ceremonial Javelin", "祭典之标枪"),
        ("Boneweave Boots", "骨织靴"),
        ("Round Shield", "圆形盾"),
        ("Balrog Skin", "炎魔皮板甲"),
        ("Dusk Shroud", "灰暮寿衣"),
        ("Myrmidon Greaves", "急速靴"),
        ("Ogre Gauntlets", "食人魔铁手套"),
        ("Zweihander", "双手剑"),
        ("Ettin Axe", "双刃斧"),
        ("Matriarchal Javelin", "女族长之标枪"),
        ("Colossus Crossbow", "巨神十字弓"),
        ("Slayer Guard", "杀手防护面甲"),
        ("Battle Gauntlets", "战斗手套"),
        ("Gilded Shield", "饰金盾牌"),
        ("Winged Knife", "飞刃"),
        ("Cuirass", "护胸甲"),
        ("Totemic Mask", "图腾面具"),
        ("Bloodlord Skull", "血王骨"),
        ("Highland Blade", "高地之剑"),

        // =========================
        // 地狱火炬钥匙 / 洗点
        // =========================
        ("Key Of Destruction", "毁灭之钥"),
        ("Key Of Hate", "憎恨之钥"),
        ("Key Of Terror", "恐惧之钥"),
        ("Token Of Absolution", "赦免徽章"),

        // =========================
        // 暗金 / 套装
        // =========================
        ("Thudergods Vigor War Belt", "雷神之力 战场腰带"),
        ("Thundergods Vigor War Belt", "雷神之力 战场腰带"),
        ("Homunculus Hierophant Trophy", "侏儒 祭司印记"),
        ("Crown Of Thieves Grand Crown", "盗贼皇冠 巨皇冠"),
        ("Civerbs Icon Amulet", "希弗柏的图案 项链"),
        ("Razortail Sharkskin Belt", "剃刀之尾 鲨皮腰带"),
        ("Najs Puzzler Elder Staff", "娜吉的解谜杖 接骨木之杖"),
        ("Blade Of Ali Baba Tulwar", "阿里巴巴之刃 土耳其剑"),
        ("Manald Heal Ring", "马纳得的治疗 戒指"),
        ("Fleshripper Fanged Knife", "裂肉者 齿缘小刀"),
        ("Trang Ouls Claws Heavy Bracers", "塔格奥之爪 重型护腕"),

        // 单独名字兜底
        ("Thudergods Vigor", "雷神之力"),
        ("Thundergods Vigor", "雷神之力"),
        ("Homunculus", "侏儒"),
        ("Crown Of Thieves", "盗贼皇冠"),
        ("Civerbs Icon", "希弗柏的图案"),
        ("Razortail", "剃刀之尾"),
        ("Najs Puzzler", "娜吉的解谜杖"),
        ("Blade Of Ali Baba", "阿里巴巴之刃"),
        ("Manald Heal", "马纳得的治疗"),
        ("Fleshripper", "裂肉者"),
        ("Trang Ouls Claws", "塔格奥之爪"),

        // 蓝色 / 亮金随机名
        ("Opalvein Amulet", "亮金项链"),
        ("Opalvein", "亮金"),

        // 新增底材
        ("Greater Claws", "巨爪"),
        ("Runic Talons", "符文爪"),
        ("Hierophant Trophy", "祭司印记"),
        ("Grand Crown", "巨皇冠"),
        ("Sharkskin Belt", "鲨皮腰带"),
        ("Tulwar", "土耳其剑"),
        ("Fanged Knife", "齿缘小刀"),
        ("Heavy Bracers", "重型护腕"),

        // 塔拉夏套装
        ("Tal Rashas Horadric Crest Death Mask", "塔拉夏的赫拉迪克纹章 死亡面具"),
        ("Tal Rashas Howling Wind Lacquered Plate", "塔拉夏的守护 漆甲"),
        ("Tal Rashas Lidless Eye Swirling Crystal", "塔拉夏的警戒之眼 涡流水晶"),
        ("Tal Rashas Adjudication Amulet", "塔拉夏的判决 项链"),
        ("Tal Rashas Horadric Crest", "塔拉夏的赫拉迪克纹章"),
        ("Tal Rashas Howling Wind", "塔拉夏的守护"),
        ("Tal Rashas Lidless Eye", "塔拉夏的警戒之眼"),
        ("Tal Rashas Adjudication", "塔拉夏的判决"),

        // 精华
        ("Festering Essence Of Destruction", "溃烂的毁灭精华"),
        ("Burning Essence Of Terror", "燃烧的恐惧精华"),
        ("Charged Essence Of Hatred", "充盈的憎恨精华"),
        ("Twisted Essence Of Suffering", "扭曲的痛苦精华"),

        // 暗金
        ("Wartraveler Battle Boots", "战争旅者 战场之靴"),
        ("Suicide Branch Burnt Wand", "自杀支系 烧焦之杖"),
        ("Natures Peace Ring", "大自然的和平 戒指"),
        ("String Of Ears Demonhide Sash", "长串之耳 魔皮勋带"),
        ("Wizardspike Bone Knife", "巫师之刺 骸骨小刀"),
        ("Gheeds Fortune Grand Charm", "暗金超大护身符"), // 屏蔽防透视修改
        ("Nosferatus Coil Vampirefang Belt", "吸血圣王之圈 吸血鬼獠牙腰带"),
        ("The Stone Of Jordan Ring", "乔丹之石 戒指"),
        ("Ondals Wisdom Elder Staff", "安戴尔的智慧 接骨木之杖"),

        // 单独暗金名兜底
        ("Wartraveler", "战争旅者"),
        ("Suicide Branch", "自杀支系"),
        ("Natures Peace", "大自然的和平"),
        ("String Of Ears", "长串之耳"),
        ("Wizardspike", "巫师之刺"),
        ("Gheeds Fortune", "暗金超大护身符"), // 屏蔽防透视修改
        ("Nosferatus Coil", "吸血圣王之圈"),
        ("The Stone Of Jordan", "乔丹之石"),
        ("Ondals Wisdom", "安戴尔的智慧"),

        // 新增底材
        ("Greater Talons", "巨鹰爪"),
        ("Giant Sword", "巨剑"),
        ("Burnt Wand", "烧焦之杖"),
        ("Death Mask", "死亡面具"),
        ("Demonhide Sash", "魔皮勋带"),
        ("Bone Knife", "骸骨小刀"),

        // =========================
        // 品质 / 状态
        // =========================
        ("Low Quality", "劣质"),
        ("Cracked", "破裂"),
        ("Crude", "粗糙"),
        ("Damaged", "损坏"),
        ("Superior", "超强"),
        ("Normal", "普通"),
        ("Magic", "蓝色"),
        ("Rare", "亮金"),
        ("Set", "绿色套装"),
        ("Unique", "暗金"),
        ("Crafted", "橙色手工"),
        ("Tempered", "淬火"),
        ("Ethereal", "无形"),
        ("Socketed", "有孔"),
        ("Sockets", "孔"),
        ("Socket", "孔"),

        // =========================
        // 符文
        // =========================
        ("El Rune", "一号 艾尔符文"),
        ("Eld Rune", "二号 艾德符文"),
        ("Tir Rune", "三号 特尔符文"),
        ("Nef Rune", "四号 那夫符文"),
        ("Eth Rune", "五号 爱斯符文"),
        ("Ith Rune", "六号 伊司符文"),
        ("Tal Rune", "七号 塔尔符文"),
        ("Ral Rune", "八号 拉尔符文"),
        ("Ort Rune", "九号 欧特符文"),
        ("Thul Rune", "十号 书尔符文"),
        ("Amn Rune", "十一号 安姆符文"),
        ("Sol Rune", "十二号 索尔符文"),
        ("Shael Rune", "十三号 夏符文"),
        ("Dol Rune", "十四号 多尔符文"),
        ("Hel Rune", "十五号 海尔符文"),
        ("Io Rune", "十六号 埃欧符文"),
        ("Lum Rune", "十七号 卢姆符文"),
        ("Ko Rune", "十八号 科符文"),
        ("Fal Rune", "十九号 法尔符文"),
        ("Lem Rune", "二十号 蓝姆符文"),
        ("Pul Rune", "二十一号 普尔符文"),
        ("Um Rune", "二十二号 乌姆符文"),
        ("Mal Rune", "二十三号 马尔符文"),
        ("Ist Rune", "二十四号 伊斯特符文"),
        ("Gul Rune", "二十五号 古尔符文"),
        ("Vex Rune", "二十六号 伐克斯符文"),
        ("Ohm Rune", "二十七号 欧姆符文"),
        ("Lo Rune", "二十八号 罗符文"),
        ("Sur Rune", "二十九号 瑟符文"),
        ("Ber Rune", "三十号 贝符文"),
        ("Jah Rune", "三十一号 乔符文"),
        ("Cham Rune", "三十二号 查姆符文"),
        ("Zod Rune", "三十三号 萨德符文"),

        // =========================
        // 首饰 / 护身符 / 珠宝
        // =========================
        ("Grand Charm", "超大护身符"),
        ("Large Charm", "大型护身符"),
        ("Small Charm", "小护身符"),
        ("Charm", "护身符"),
        ("Jewel", "珠宝"),
        ("Amulet", "项链"),
        ("Ring", "戒指"),

        // =========================
        // 常见暗金
        // =========================
        ("Harlequin Crest Shako", "谐角之冠 军帽"),
        ("Harlequin Crest", "谐角之冠"),
        ("Griffons Eye Diadem", "格里风之眼 权冠"),
        ("Griffons Eye", "格里风之眼"),
        ("Deaths Fathom Dimensional Shard", "死亡深度 次元碎片"),
        ("Deaths Fathom", "死亡深度"),
        ("The Oculus Swirling Crystal", "眼球 涡流水晶"),
        ("The Oculus", "眼球"),
        ("Arachnid Mesh Spiderweb Sash", "蜘蛛之网 蛛网腰带"),
        ("Arachnid Mesh", "蜘蛛之网"),
        ("Skin Of The Vipermagi Serpentskin Armor", "蛇魔法师之皮 海蛇皮甲"),
        ("Skin Of The Vipermagi", "蛇魔法师之皮"),
        ("Sandstorm Trek Scarabshell Boots", "沙暴之旅 圣甲虫壳靴"),
        ("Sandstorm Trek", "沙暴之旅"),
        ("Magefist Light Gauntlets", "法师之拳 轻型铁手套"),
        ("Magefist", "法师之拳"),
        ("Chance Guards Chain Gloves", "运气守护 锁链手套"),
        ("Chance Guards", "运气守护"),
        ("Maras Kaleidoscope Amulet", "马拉的万花筒 项链"),
        ("Maras Kaleidoscope", "马拉的万花筒"),
        ("Stone Of Jordan Ring", "乔丹之石 戒指"),
        ("Stone Of Jordan", "乔丹之石"),
        ("Bul Kathos Wedding Band Ring", "布尔凯索的婚戒 戒指"),
        ("Bul Kathos Wedding Band", "布尔凯索的婚戒"),
        ("Raven Frost Ring", "乌鸦之霜 戒指"),
        ("Raven Frost", "乌鸦之霜"),
        ("Highlords Wrath Amulet", "大君之怒 项链"),
        ("Highlords Wrath", "大君之怒"),
        ("Metalgrid Amulet", "金属网格 项链"),
        ("Metalgrid", "金属网格"),
        ("Wisp Projector Ring", "鬼火投射者 戒指"),
        ("Wisp Projector", "鬼火投射者"),
        ("Deaths Web Unearthed Wand", "死亡之网 破隐法杖"),
        ("Deaths Web", "死亡之网"),
        ("Crown Of Ages Corona", "年纪之冠 头冠"),
        ("Crown Of Ages", "年纪之冠"),
        ("Andariels Visage Demonhead", "安达利尔的面貌 恶魔头盖骨面具"),
        ("Andariels Visage", "安达利尔的面貌"),
        ("Nightwings Veil Spired Helm", "夜翼面纱 尖刺面甲"),
        ("Nightwings Veil", "夜翼面纱"),
        ("Draculs Grasp Vampirebone Gloves", "卓古拉之握 吸血鬼骸骨手套"),
        ("Draculs Grasp", "卓古拉之握"),

        // =========================
        // 头盔底材
        // =========================
        ("Diadem", "权冠"),
        ("Tiara", "三重冠"),
        ("Circlet", "宝冠"),
        ("Coronet", "头冠"),
        ("Shako", "军帽"),
        ("Bone Visage", "白骨面甲"),
        ("Spired Helm", "尖刺面甲"),
        ("Demonhead", "恶魔头盖骨面具"),
        ("Corona", "头冠"),
        ("Dream Spirit", "梦境之灵"),
        ("Blood Spirit", "鲜血之灵"),
        ("Sun Spirit", "太阳之灵"),
        ("Earth Spirit", "大地之灵"),
        ("Sky Spirit", "天空之灵"),
        ("Armet", "活动头盔"),
        ("Giant Conch", "巨贝头盔"),
        ("Demon Head", "恶魔头盖骨面具"),
        ("Hydraskull", "九头蛇头骨帽"),

        // =========================
        // 衣服底材
        // =========================
        ("Archon Plate", "执政官铠甲"),
        ("Mage Plate", "法师铠甲"),
        ("Wire Fleece", "线羊毛皮甲"),
        ("Great Hauberk", "巨型锁子甲"),
        ("Wyrmhide", "古龙皮"),
        ("Scarab Husk", "圣甲虫壳皮甲"),
        ("Boneweave", "骨织铠甲"),
        ("Diamond Mail", "钻石锁子甲"),
        ("Kraken Shell", "海妖壳甲"),
        ("Hellforge Plate", "地狱锻甲"),
        ("Lacquered Plate", "漆甲"),
        ("Shadow Plate", "阴影铠甲"),
        ("Sacred Armor", "神圣盔甲"),
        ("Serpentskin Armor", "海蛇皮甲"),
        ("Light Plate", "轻型铠甲"),
        ("Breast Plate", "胸甲"),
        ("Full Plate Mail", "全身板甲"),
        ("Ancient Armor", "古代装甲"),

        // =========================
        // 盾牌底材
        // =========================
        ("Monarch", "统治者大盾"),
        ("Aegis", "保护盾牌"),
        ("Ward", "保护盾"),
        ("Blade Barrier", "刀刃刺盾"),
        ("Troll Nest", "洞穴巨魔巢穴骨盾"),
        ("Hyperion", "亥伯龙盾"),
        ("Luna", "月精灵护盾"),
        ("Heater", "加热盾牌"),
        ("Sacred Targe", "神圣小盾"),
        ("Sacred Rondache", "神圣轻圆盾"),
        ("Kurast Shield", "库拉斯特盾"),
        ("Zakarum Shield", "撒卡兰姆盾"),
        ("Vortex Shield", "旋风盾"),
        ("Akaran Targe", "亚克南小盾"),
        ("Akaran Rondache", "亚克南轻圆盾"),
        ("Protector Shield", "保护者盾牌"),
        ("Royal Shield", "皇家盾牌"),

        // =========================
        // 长柄 / 矛 / 佣兵底材
        // =========================
        ("Giant Thresher", "巨型长柄镰刀"),
        ("Thresher", "长柄镰刀"),
        ("Cryptic Axe", "神秘之斧"),
        ("Colossus Voulge", "巨神之斧"),
        ("Great Poleaxe", "鲛尾巨斧"),
        ("Mancatcher", "捕人叉"),
        ("Stygian Pike", "冥河之枪"),
        ("Ghost Spear", "鬼魂之矛"),
        ("War Pike", "战枪"),
        ("Ogre Axe", "食人魔之斧"),
        ("Partizan", "长柄战斧"),
        ("Bec De Corbin", "双锋战戟"),
        ("Grim Scythe", "残酷镰刀"),
        ("Bill", "钩镰枪"),
        ("Battle Scythe", "战斗镰刀"),
        ("Poleaxe", "长柄斧"),
        ("Halberd", "戟"),

        // =========================
        // 常见武器底材
        // =========================
        ("Phase Blade", "幻化之刃"),
        ("Crystal Sword", "水晶剑"),
        ("Broad Sword", "阔剑"),
        ("Long Sword", "长剑"),
        ("Berserker Axe", "狂战士斧"),
        ("Colossus Blade", "巨神之刃"),
        ("Colossus Sword", "巨神之剑"),
        ("Balrog Blade", "炎魔之刃"),
        ("Legend Sword", "传说之剑"),
        ("Champion Sword", "冠军之剑"),
        ("Cryptic Sword", "神秘之剑"),
        ("Conquest Sword", "征服之剑"),
        ("Hydra Bow", "九头蛇弓"),
        ("Grand Matron Bow", "大院长之弓"),
        ("Blade Bow", "刀锋弓"),
        ("Great Bow", "巨弓"),
        ("Diamond Bow", "钻石弓"),
        ("Crusader Bow", "十字军之弓"),
        ("Ward Bow", "庇护之弓"),

        // =========================
        // 连枷 / 法杖 / 权杖 / 手杖
        // =========================
        ("Flail", "连枷"),
        ("Knout", "铁皮鞭"),
        ("Scourge", "天罚之锤"),
        ("Caduceus", "神使之杖"),
        ("Divine Scepter", "神属权杖"),
        ("Mighty Scepter", "强威权杖"),
        ("Seraph Rod", "炽天使法杖"),
        ("Dimensional Shard", "次元碎片"),
        ("Eldritch Orb", "怪异之球"),
        ("Swirling Crystal", "涡流水晶"),
        ("Unearthed Wand", "破隐法杖"),
        ("Lich Wand", "巫妖法杖"),
        ("Ghost Wand", "鬼魂法杖"),
        ("Polished Wand", "抛光法杖"),
        ("Elder Staff", "接骨木之杖"),
        ("Shillelagh", "树皮之杖"),
        ("Archon Staff", "执政官之杖"),
        ("Walking Stick", "手杖"),
        ("Stalagmite", "石笋之杖"),
        ("Rune Staff", "符文之杖"),

        // =========================
        // 手套 / 鞋 / 腰带
        // =========================
        ("Vampirebone Gloves", "吸血鬼骸骨手套"),
        ("Vambraces", "吸血鬼护腕"),
        ("Crusader Gauntlets", "十字军铁手套"),
        ("Chain Gloves", "锁链手套"),
        ("Light Gauntlets", "轻型铁手套"),
        ("Heavy Gloves", "重型手套"),
        ("Battle Boots", "战场之靴"),
        ("War Boots", "战靴"),
        ("Scarabshell Boots", "圣甲虫壳靴"),
        ("Mirrored Boots", "镜化靴"),
        ("Spiderweb Sash", "蛛网腰带"),
        ("Mithril Coil", "秘银腰带"),
        ("War Belt", "战场腰带"),
        ("Vampirefang Belt", "吸血鬼獠牙腰带"),
        ("Troll Belt", "洞穴巨魔腰带"),
        ("Colossus Girdle", "巨神腰带"),
    ];

    replaces.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    for (from, to) in replaces {
        s = s.replace(from, to);
    }

    s
}

fn has_english_letters(text: &str) -> bool {
    text.chars().any(|c| c.is_ascii_alphabetic())
}

fn save_untranslated_tts(text: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tts_untranslated.txt")
    {
        let _ = writeln!(file, "{}", text);
    }
}

fn guess_item_type_cn(text: &str) -> &'static str {
    if text.contains("Unique") {
        "暗金物品"
    } else if text.contains("Rare") {
        "亮金物品"
    } else if text.contains("Magic") {
        "蓝色物品"
    } else if text.contains("Set") {
        "绿色套装物品"
    } else if text.contains("Rune") {
        "符文"
    } else if text.contains("Charm") {
        "护身符"
    } else if text.contains("Jewel") {
        "珠宝"
    } else if text.contains("Ring") {
        "戒指"
    } else if text.contains("Amulet") {
        "项链"
    } else if text.contains("Superior") {
        "超强白色底材"
    } else {
        "白色或灰色底材"
    }
}
