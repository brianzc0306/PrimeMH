use core::fmt;
use std::mem::transmute;

pub fn parse_state_flags(state_flags: [u32; 8]) -> [State; 256] {
    let mut states = [State::None; 256];
    for (n, byte) in state_flags.iter().enumerate() {
        if byte > &0 {
            for i in 0..32 {
                let bit_mask = 2_u32.pow(i as u32);
                if byte & bit_mask as u32 > 0 {
                    let index = (n * 32) + i;
                    // 安全警告解除：State 枚举现已完整覆盖 0~191，transmute 不再越界崩溃
                    let state = unsafe { transmute::<u32, State>(index as u32) };
                    states[index] = state
                }
            }
        }
    }
    states
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Ord, PartialOrd)]
pub enum State {
    None = 0,
    Freeze,
    Poison,
    ResistFire,
    ResistCold,
    ResistLight,
    ResistMagic,
    PlayerBody,
    ResistAll,
    AmplifyDamage,
    FrozenArmor,
    Cold,
    Inferno,
    Blaze,
    BoneArmor,
    Concentrate,
    Enchant,
    InnerSight,
    SkillMove,
    Weaken,
    ChillingArmor,
    Stunned,
    SpiderLay,
    DimVision,
    Slowed,
    FetishAura,
    Shout,
    Taunt,
    Conviction,
    Convicted,
    EnergyShield,
    VenomClaws,
    BattleOrders,
    Might,
    Prayer,
    HolyFire,
    Thorns,
    Defiance,
    Thunderstorm,
    LightningBolt,
    BlessedAim,
    Stamina,
    Concentration,
    HolyWind,
    HolyWindCold,
    Cleansing,
    HolyShock,
    Sanctuary,
    Meditation,
    Fanaticism,
    Redemption,
    BattleCommand,
    PreventHeal,
    Conversion,
    Uninterruptable,
    IronMaiden,
    Terror,
    Attract,
    LifeTap,
    Confuse,
    Decrepify,
    LowerResist,
    OpenWounds,
    DoppleZon,
    CriticalStrike,
    Dodge,
    Avoid,
    Penetrate,
    Evade,
    Pierce,
    Warmth,
    FireMastery,
    LightningMastery,
    ColdMastery,
    BladeMastery,
    AxeMastery,
    MaceMastery,
    PolearmMastery,
    ThrowingMastery,
    SpearMastery,
    IncreasedStamina,
    IronSkin,
    IncreasedSpeed,
    NaturalResistance,
    FingerMageCurse,
    NoManaRegen,
    JustHit,
    SlowMissiles,
    ShiverArmor,
    BattleCry,
    Blue,
    Red,
    DeathDelay,
    Valkyrie,
    Frenzy,
    Berserk,
    Revive,
    ItemFullSet,
    SourceUnit,
    Redeemed,
    HealthPot,
    HolyShield,
    JustPortaled,
    MonFrenzy,
    CorpseNoDraw,
    Alignment,
    ManaPot,
    Shatter,
    SyncWarped,
    ConversionSave,
    Pregnant,
    _111,
    Rabies,
    DefenseCurse,
    BloodMana,
    Burning,
    DragonFlight,
    Maul,
    CorpseNoSelect,
    ShadowWarrior,
    FeralRage,
    SkillDelay,
    TigerStrike,
    CobraStrike,
    PhoenixStrike,
    FistsOfFire,
    BladesOfIce,
    ClawsOfThunder,
    ShrineArmor,
    ShrineCombat,
    ShrineResistLightning,
    ShrineResistFire,
    ShrineResistCold,
    ShrineResistPoison,
    ShrineSkill,
    ShrineManaRegen,
    ShrineSTAMINA,
    ShrineExperience,
    FenrisRage,
    Wolf,
    Bear,
    Bloodlust,
    ChangeClass,
    Attached,
    Hurricane,
    Armageddon,
    Invis,
    Barbs,
    Wolverine,
    OakSage,
    VineBeast,
    CycloneArmor,
    ClawMastery,
    CloakOfShadows,
    Recycled,
    WeaponBlock,
    Cloaked,
    Quickness,
    Bladeshield,
    Fade,
    SummonResist,
    OakSageControl,
    WolverineControl,
    BarbsControl,
    DebugControl,
    ItemSet1,
    ItemSet2,
    ItemSet3,
    ItemSet4,
    ItemSet5,
    ItemSet6,
    Runeword,
    RestInPeace,
    CorpseEXP,
    Whirlwind,
    FullSetGeneric,
    MonsterSet,
    Delerium,
    Antidote,
    Thawing,
    StaminaPot,
    PassiveResistFire,
    PassiveResistCold,
    PassiveResistLtng,
    UberMinion,
    CoolDown,
    SharedStash,
    HideDead,

    // ... 前面保持不变 ...
    BindDemon,    // 188: 术士王朝 Mod 的恶魔奴役状态
    State189, 
	State190, 
	State191, 
	State192, 
	State193, 
	State194, 
	State195, 
	State196, 
	State197,
    HexBane,      // 198: 邪术：灾祸（雷达确定的位置！）
    State199,     // 199: 填充
    HexSiphon,     // 200: 邪术：虹吸
    State201,     // 201: 填充
	HexPurge,     // 202: 邪术：净化（精准命中！）
    State203,    // 203: 填充
    State204, 
	State205, 
	State206, 
	State207, 
	Consume,     // 208: 👉 术士技能：吞噬
	State209, 
	State210, 
	State211, 
	State212, 
	State213,
    State214, 
	State215, 
	State216, 
	State217, 
	State218, 
	State219, 
	State220, 
	State221, 
	State222, 
	State223,
    State224, 
	State225, 
	State226, 
	State227, 
	State228, 
	State229, 
	State230, 
	State231, 
	State232, 
	State233,
    State234, 
	State235, 
	State236, 
	State237, 
	State238, 
	State239, 
	State240, 
	State241, 
	State242, 
	State243,
    State244, 
	State245, 
	State246, 
	State247, 
	State248, 
	State249, 
	State250, 
	State251, 
	State252, 
	State253,
    State254, 
	State255,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// this requires nightly builds

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     #[test]
//     fn test_flag_parsing() {
//         let parsed_states = parse_state_flags([1024, 131073, 0, 512, 1, 0]);
//         assert_eq!(State::FrozenArmor, parsed_states[10]);
//         assert_eq!(State::BattleOrders, parsed_states[32]);
//         assert_eq!(State::Fanaticism, parsed_states[49]);
//         assert_eq!(State::Alignment, parsed_states[105]);
//         assert_eq!(State::ShrineArmor, parsed_states[128]);
//     }

//     #[bench]
//     fn calc_states(b: &mut Bencher) {
//         let state_bytes: [u32; 6] = [1024, 131073, 0, 512, 1, 0];
//         b.iter(|| parse_state_flags(state_bytes));
//     }
// }
