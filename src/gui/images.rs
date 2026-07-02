use std::collections::HashMap;

use notan::prelude::{Graphics, Texture};

pub fn load_images(gfx: &mut Graphics) -> HashMap<String, Texture> {
    let mut image_data_list: HashMap<String, &'static [u8]> = HashMap::new();
    image_data_list.insert(String::from("shrine"), include_bytes!("./images/shrine.png"));
    image_data_list.insert(String::from("well"), include_bytes!("./images/well.png"));
    image_data_list.insert(String::from("chest"), include_bytes!("./images/chest.png"));
    image_data_list.insert(String::from("superchest"), include_bytes!("./images/superchest.png"));

    image_data_list.insert(String::from("key"), include_bytes!("./images/key.png"));
    image_data_list.insert(String::from("identify_scroll"), include_bytes!("./images/identify_scroll.png"));
    image_data_list.insert(String::from("town_portal_scroll"), include_bytes!("./images/town_portal_scroll.png"));

    //buff icons
    image_data_list.insert(String::from("AmplifyDamage"), include_bytes!("./images/icons/AmplifyDamage.png"));
    image_data_list.insert(String::from("Armageddon"), include_bytes!("./images/icons/Armageddon.png"));
    image_data_list.insert(String::from("Attract"), include_bytes!("./images/icons/Attract.png"));
    image_data_list.insert(String::from("Avoid"), include_bytes!("./images/icons/Avoid.png"));
    image_data_list.insert(String::from("AxeMastery"), include_bytes!("./images/icons/AxeMastery.png"));
    image_data_list.insert(String::from("Barbs"), include_bytes!("./images/icons/Barbs.png"));
    image_data_list.insert(String::from("BattleCommand"), include_bytes!("./images/icons/BattleCommand.png"));
    image_data_list.insert(String::from("BattleOrders"), include_bytes!("./images/icons/BattleOrders.png"));
    image_data_list.insert(String::from("Bear"), include_bytes!("./images/icons/Bear.png"));
    image_data_list.insert(String::from("Berserk"), include_bytes!("./images/icons/Berserk.png"));
    image_data_list.insert(String::from("BladeMastery"), include_bytes!("./images/icons/BladeMastery.png"));
    image_data_list.insert(String::from("BladeShield"), include_bytes!("./images/icons/BladeShield.png"));
    image_data_list.insert(String::from("BladesOfIce"), include_bytes!("./images/icons/BladesOfIce.png"));
    image_data_list.insert(String::from("Blaze"), include_bytes!("./images/icons/Blaze.png"));
    image_data_list.insert(String::from("BlessedAim"), include_bytes!("./images/icons/BlessedAim.png"));
    image_data_list.insert(String::from("BloodMana"), include_bytes!("./images/icons/BloodMana.png"));
    image_data_list.insert(String::from("BoneArmor"), include_bytes!("./images/icons/BoneArmor.png"));
    image_data_list.insert(String::from("ChillingArmor"), include_bytes!("./images/icons/ChillingArmor.png"));
    image_data_list.insert(String::from("ClawMastery"), include_bytes!("./images/icons/ClawMastery.png"));
    image_data_list.insert(String::from("ClawsOfThunder"), include_bytes!("./images/icons/ClawsOfThunder.png"));
    image_data_list.insert(String::from("Cleansing"), include_bytes!("./images/icons/Cleansing.png"));
    image_data_list.insert(String::from("Cloaked"), include_bytes!("./images/icons/Cloaked.png"));
    image_data_list.insert(String::from("CloakofShadows"), include_bytes!("./images/icons/CloakofShadows.png"));
    image_data_list.insert(String::from("CobraStrike"), include_bytes!("./images/icons/CobraStrike.png"));
    image_data_list.insert(String::from("Cold"), include_bytes!("./images/icons/Cold.png"));
    image_data_list.insert(String::from("ColdMastery"), include_bytes!("./images/icons/ColdMastery.png"));
    image_data_list.insert(String::from("Concentration"), include_bytes!("./images/icons/Concentration.png"));
    image_data_list.insert(String::from("Confuse"), include_bytes!("./images/icons/Confuse.png"));
    image_data_list.insert(String::from("Conversion"), include_bytes!("./images/icons/Conversion.png"));
    image_data_list.insert(String::from("Convicted"), include_bytes!("./images/icons/Convicted.png"));
    image_data_list.insert(String::from("Conviction"), include_bytes!("./images/icons/Conviction.png"));
    image_data_list.insert(String::from("CriticalStrike"), include_bytes!("./images/icons/CriticalStrike.png"));
    image_data_list.insert(String::from("CycloneArmor"), include_bytes!("./images/icons/CycloneArmor.png"));
    image_data_list.insert(String::from("Decoy"), include_bytes!("./images/icons/Decoy.png"));
    image_data_list.insert(String::from("Decrepify"), include_bytes!("./images/icons/Decrepify.png"));
    image_data_list.insert(String::from("DefenseCurse"), include_bytes!("./images/icons/DefenseCurse.png"));
    image_data_list.insert(String::from("Defiance"), include_bytes!("./images/icons/Defiance.png"));
    image_data_list.insert(String::from("DimVision"), include_bytes!("./images/icons/DimVision.png"));
    image_data_list.insert(String::from("Dodge"), include_bytes!("./images/icons/Dodge.png"));
    image_data_list.insert(String::from("Enchant"), include_bytes!("./images/icons/Enchant.png"));
    image_data_list.insert(String::from("EnergyShield"), include_bytes!("./images/icons/EnergyShield.png"));
    image_data_list.insert(String::from("Evade"), include_bytes!("./images/icons/Evade.png"));
    image_data_list.insert(String::from("Fade"), include_bytes!("./images/icons/Fade.png"));
    image_data_list.insert(String::from("Fanaticism"), include_bytes!("./images/icons/Fanaticism.png"));
    image_data_list.insert(String::from("FenrisRage"), include_bytes!("./images/icons/FenrisRage.png"));
    image_data_list.insert(String::from("FeralRage"), include_bytes!("./images/icons/FeralRage.png"));
    image_data_list.insert(String::from("FireMastery"), include_bytes!("./images/icons/FireMastery.png"));
    image_data_list.insert(String::from("FistsOfFire"), include_bytes!("./images/icons/FistsOfFire.png"));
    image_data_list.insert(String::from("Frenzy"), include_bytes!("./images/icons/Frenzy.png"));
    image_data_list.insert(String::from("FrozenArmor"), include_bytes!("./images/icons/FrozenArmor.png"));
    
    // 👇 === 新增三大邪术的图片加载 === 👇
    image_data_list.insert(String::from("HexBane"), include_bytes!("./images/icons/HexBane.png"));
    image_data_list.insert(String::from("HexPurge"), include_bytes!("./images/icons/HexPurge.png"));
    image_data_list.insert(String::from("HexSiphon"), include_bytes!("./images/icons/HexSiphon.png"));
	image_data_list.insert(String::from("Consume"), include_bytes!("./images/icons/Consume.png"));
    // 👆 ============================ 👆

    image_data_list.insert(String::from("HolyFire"), include_bytes!("./images/icons/HolyFire.png"));
    image_data_list.insert(String::from("HolyShield"), include_bytes!("./images/icons/HolyShield.png"));
    image_data_list.insert(String::from("HolyShock"), include_bytes!("./images/icons/HolyShock.png"));
    image_data_list.insert(String::from("HolyWind"), include_bytes!("./images/icons/HolyWind.png"));
    image_data_list.insert(String::from("Hurricane"), include_bytes!("./images/icons/Hurricane.png"));
    image_data_list.insert(String::from("Impale"), include_bytes!("./images/icons/Impale.png"));
    image_data_list.insert(String::from("IncreasedSpeed"), include_bytes!("./images/icons/IncreasedSpeed.png"));
    image_data_list.insert(String::from("IncreasedStamina"), include_bytes!("./images/icons/IncreasedStamina.png"));
    image_data_list.insert(String::from("Inferno"), include_bytes!("./images/icons/Inferno.png"));
    image_data_list.insert(String::from("InnerSight"), include_bytes!("./images/icons/InnerSight.png"));
    image_data_list.insert(String::from("IronMaiden"), include_bytes!("./images/icons/IronMaiden.png"));
    image_data_list.insert(String::from("IronSkin"), include_bytes!("./images/icons/IronSkin.png"));
    image_data_list.insert(String::from("LifeTap"), include_bytes!("./images/icons/LifeTap.png"));
    image_data_list.insert(String::from("LightningMastery"), include_bytes!("./images/icons/LightningMastery.png"));
    image_data_list.insert(String::from("LowerResist"), include_bytes!("./images/icons/LowerResist.png"));
    image_data_list.insert(String::from("MaceMastery"), include_bytes!("./images/icons/MaceMastery.png"));
    image_data_list.insert(String::from("Meditation"), include_bytes!("./images/icons/Meditation.png"));
    image_data_list.insert(String::from("Might"), include_bytes!("./images/icons/Might.png"));
    image_data_list.insert(String::from("NaturalResistance"), include_bytes!("./images/icons/NaturalResistance.png"));
    image_data_list.insert(String::from("OakSage"), include_bytes!("./images/icons/OakSage.png"));
    image_data_list.insert(String::from("Penetrate"), include_bytes!("./images/icons/Penetrate.png"));
    image_data_list.insert(String::from("PhoenixStrike"), include_bytes!("./images/icons/PhoenixStrike.png"));
    image_data_list.insert(String::from("Pierce"), include_bytes!("./images/icons/Pierce.png"));
    image_data_list.insert(String::from("Poison"), include_bytes!("./images/icons/Poison.png"));
    image_data_list.insert(String::from("PolearmMastery"), include_bytes!("./images/icons/PolearmMastery.png"));
    image_data_list.insert(String::from("Prayer"), include_bytes!("./images/icons/Prayer.png"));
    image_data_list.insert(String::from("Quickness"), include_bytes!("./images/icons/Quickness.png"));
    image_data_list.insert(String::from("Redemption"), include_bytes!("./images/icons/Redemption.png"));
    image_data_list.insert(String::from("ResistAll"), include_bytes!("./images/icons/ResistAll.png"));
    image_data_list.insert(String::from("ResistCold"), include_bytes!("./images/icons/ResistCold.png"));
    image_data_list.insert(String::from("ResistFire"), include_bytes!("./images/icons/ResistFire.png"));
    image_data_list.insert(String::from("ResistLight"), include_bytes!("./images/icons/ResistLight.png"));
    image_data_list.insert(String::from("Salvation"), include_bytes!("./images/icons/Salvation.png"));
    image_data_list.insert(String::from("Sanctuary"), include_bytes!("./images/icons/Sanctuary.png"));
    image_data_list.insert(String::from("ShadowMaster"), include_bytes!("./images/icons/ShadowMaster.png"));
    image_data_list.insert(String::from("ShadowWarrior"), include_bytes!("./images/icons/ShadowWarrior.png"));
    image_data_list.insert(String::from("ShiverArmor"), include_bytes!("./images/icons/ShiverArmor.png"));
    image_data_list.insert(String::from("Shout"), include_bytes!("./images/icons/Shout.png"));
    image_data_list.insert(String::from("SlowMissiles"), include_bytes!("./images/icons/SlowMissiles.png"));
    image_data_list.insert(String::from("Slowed"), include_bytes!("./images/icons/Slowed.png"));
    image_data_list.insert(String::from("SpearMastery"), include_bytes!("./images/icons/SpearMastery.png"));
    image_data_list.insert(String::from("Stamina"), include_bytes!("./images/icons/Stamina.png"));
    image_data_list.insert(String::from("Teleport"), include_bytes!("./images/icons/Teleport.png"));
    image_data_list.insert(String::from("Terror"), include_bytes!("./images/icons/Terror.png"));
    image_data_list.insert(String::from("Thorns"), include_bytes!("./images/icons/Thorns.png"));
    image_data_list.insert(String::from("ThrowingMastery"), include_bytes!("./images/icons/ThrowingMastery.png"));
    image_data_list.insert(String::from("Thunderstorm"), include_bytes!("./images/icons/ThunderStorm.png"));
    image_data_list.insert(String::from("TigerStrike"), include_bytes!("./images/icons/TigerStrike.png"));
    image_data_list.insert(String::from("Valkyrie"), include_bytes!("./images/icons/Valkyrie.png"));
    image_data_list.insert(String::from("VenomClaws"), include_bytes!("./images/icons/VenomClaws.png"));
    image_data_list.insert(String::from("Vigor"), include_bytes!("./images/icons/Vigor.png"));
    image_data_list.insert(String::from("Warmth"), include_bytes!("./images/icons/Warmth.png"));
    image_data_list.insert(String::from("Weaken"), include_bytes!("./images/icons/Weaken.png"));
    image_data_list.insert(String::from("WeaponBlock"), include_bytes!("./images/icons/WeaponBlock.png"));
    image_data_list.insert(String::from("Whirlwind"), include_bytes!("./images/icons/Whirlwind.png"));
    image_data_list.insert(String::from("Wolf"), include_bytes!("./images/icons/Wolf.png"));
    image_data_list.insert(String::from("Wolverine"), include_bytes!("./images/icons/Wolverine.png"));

    let mut images: HashMap<String, Texture> = HashMap::new();

    for (name, bytes) in image_data_list.iter() {
        images.insert(name.to_string(), gfx.create_texture().from_image(*bytes).build().unwrap());
    }
    images
}
