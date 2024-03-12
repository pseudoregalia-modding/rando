#[derive(PartialEq, Clone, Copy, strum::AsRefStr)]
pub enum Ability {
    #[strum(serialize = "attack")]
    DreamBreaker,
    #[strum(serialize = "airKick")]
    SunGreaves,
    #[strum(serialize = "slide")]
    Slide,
    #[strum(serialize = "plunge")]
    Sunsetter,
    #[strum(serialize = "wallRide")]
    ClingGem(u8),
    #[strum(serialize = "light")]
    AscendantLight,
    #[strum(serialize = "projectile")]
    SoulCutter,
    #[strum(serialize = "powerBoost")]
    Indignation,
    #[strum(serialize = "SlideJump")]
    SolarWind,
    #[strum(serialize = "chargeAttack")]
    Strikebreak,
    #[strum(serialize = "map")]
    Memento,
    #[strum(serialize = "extraKick")]
    HeliacalPower,
    #[strum(serialize = "airRecovery")]
    AerialFinesse,
    #[strum(serialize = "mobileHeal")]
    Pilgrimage,
    #[strum(serialize = "magicHaste")]
    Empathy,
    #[strum(serialize = "HealBoost")]
    GoodGraces,
    #[strum(serialize = "damageBoost")]
    MartialProwess,
    #[strum(serialize = "magicPiece")]
    ClearMind,
    #[strum(serialize = "outfitPro")]
    Professional,
    #[strum(serialize = "outfitShoujo")]
    Guardian,
    #[strum(serialize = "outfitKnight")]
    Soldier,
    #[strum(serialize = "outfitPast")]
    BleedingHeart,
    #[strum(serialize = "outfitJam")]
    Xix,
    #[strum(serialize = "outfitFaith")]
    SolSister,
    #[strum(serialize = "outfitClassy")]
    Classy,
    #[strum(serialize = "outfitSweater")]
    Sleepytime,
}

impl std::fmt::Debug for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            A::DreamBreaker => f.write_str("Dream Breaker"),
            A::SunGreaves => f.write_str("Sun Greaves"),
            A::Slide => f.write_str("Slide"),
            A::Sunsetter => f.write_str("Sunsetter"),
            A::ClingGem(num) => f.write_fmt(format_args!("{num} Cling Gems")),
            A::AscendantLight => f.write_str("Ascendant Light"),
            A::SoulCutter => f.write_str("Soul Cutter"),
            A::Indignation => f.write_str("Indignation"),
            A::SolarWind => f.write_str("Solar Wind"),
            A::Strikebreak => f.write_str("Strikebreak"),
            A::Memento => f.write_str("Memento"),
            A::HeliacalPower => f.write_str("Heliacal Power"),
            A::AerialFinesse => f.write_str("Aerial Finesse"),
            A::Pilgrimage => f.write_str("Pilgrimage"),
            A::Empathy => f.write_str("Empathy"),
            A::GoodGraces => f.write_str("Good Graces"),
            A::MartialProwess => f.write_str("Martial Prowess"),
            A::ClearMind => f.write_str("Clear Mind"),
            A::Professional => f.write_str("Professional"),
            A::Guardian => f.write_str("Guardian"),
            A::Soldier => f.write_str("Soldier"),
            A::BleedingHeart => f.write_str("Bleeding Heart"),
            A::Xix => f.write_str("XIX"),
            A::SolSister => f.write_str("Sol Sister"),
            A::Classy => f.write_str("Classy"),
            A::Sleepytime => f.write_str("Sleepytime"),
        }
    }
}

use unreal_asset::types::PackageIndex;
use Ability as A;

impl Ability {
    pub fn data(
        &self,
        mut names: std::cell::RefMut<unreal_asset::containers::NameMap>,
        imports: &mut Vec<unreal_asset::Import>,
    ) -> unreal_asset::properties::struct_property::StructProperty {
        use unreal_asset::properties::*;
        let mut add = |name| names.add_fname(name);
        let mut insert = 0;
        let icon = match self {
            A::DreamBreaker => "icon_attack",
            A::SunGreaves => "icon_greaves",
            A::Slide => "icon_slide",
            A::Sunsetter => "icon_plunge",
            A::ClingGem(_) => "icon_clingGem",
            A::AscendantLight => "icon_Light",
            A::SoulCutter => "icon_Projectile",
            A::Indignation => "icon_indignationEye",
            A::SolarWind => "icon_slideJump",
            A::Strikebreak => "icon_chargeAttack",
            A::HeliacalPower => "icon_HeliacalPower",
            A::AerialFinesse => "icon_recovery",
            A::Pilgrimage => "icon_pilgrimage",
            A::Empathy => "icon_Empathy",
            A::GoodGraces => "icon_graces",
            A::MartialProwess => "icon_martialProwress",
            A::ClearMind => "icon_clearMind",
            _ => "no icon whoopsies :p",
        };
        let path = format!("/Game/MatTex/Textures/UI/icons/{icon}");
        if matches!(
            self,
            A::DreamBreaker
                | A::SunGreaves
                | A::Slide
                | A::Sunsetter
                | A::ClingGem(_)
                | A::AscendantLight
                | A::SoulCutter
                | A::Indignation
                | A::SolarWind
                | A::Strikebreak
                | A::Memento
                | A::HeliacalPower
                | A::AerialFinesse
                | A::Pilgrimage
                | A::Empathy
                | A::GoodGraces
                | A::MartialProwess
                | A::ClearMind
        ) {
            insert = -(imports.len() as i32) - 1;
            let package = unreal_asset::Import {
                class_package: add("/Script/CoreUObject"),
                class_name: add("Package"),
                outer_index: PackageIndex::default(),
                object_name: add(&path),
                optional: false,
            };
            imports.push(package);
            let texture = unreal_asset::Import {
                class_package: add("/Script/Engine"),
                class_name: add("Texture2D"),
                outer_index: PackageIndex::new(insert),
                object_name: add(icon),
                optional: false,
            };
            imports.push(texture);
            insert -= 1;
        }
        let struct_name = add("upgradeData");
        let struct_type = add("ST_UpgradeData");
        // let id = add("ID_9_B00677A74523CD5A81278D9E39D20FDE");
        let display_name = add("DisplayName_10_28DC975D40CA7D015C488485EC89AF36");
        // let model = add("Model_6_EA2BD71A4311ABD5191E05A7DF8AD20E");
        let tier = add("PowerupTier_26_A6DCAEEA473ADF8C601BC7A5C76D31FB");
        let instructions = add("Instructions_17_A2BE3D8F4B1ADC98F9B35CAADB8B61D5");
        let description = add("Description_18_B60793FD498439DB9A403CBD619DE9F0");
        let extended = add("ExtendedDescription_22_FA4653E041E5216157FF7998802317B9");
        let display = add("DisplayImage_30_ECF7EFC64CBA612E75C67B946B8DD873");
        let array_type = Some(add("Text"));
        let guid = Some(Guid([0; 16]));
        let text =
            |name, invariant: Option<&str>, value: Option<&str>| str_property::TextProperty {
                name,
                property_guid: guid,
                culture_invariant_string: invariant.map(str::to_string),
                namespace: Some(String::new()),
                table_id: None,
                flags: 0,
                history_type: match value.is_some() {
                    true => str_property::TextHistoryType::Base,
                    false => str_property::TextHistoryType::None,
                },
                value: value.map(str::to_string),
                ancestry: Ancestry { ancestry: vec![] },
                duplication_index: 0,
            };
        struct_property::StructProperty {
            name: struct_name,
            struct_type: Some(struct_type),
            struct_guid: guid,
            property_guid: guid,
            value: vec![
                // unnecessary i believe
                // Property::IntProperty(int_property::IntProperty {
                //     name: id,
                //     ..Default::default()
                // }),
                Property::TextProperty(
                    text(
                        display_name,
                        Some(match self {
                            A::DreamBreaker => "Dream Breaker",
                            A::SunGreaves => "Sun Greaves",
                            A::Slide => "Slide",
                            A::Sunsetter => "Sunsetter",
                            A::ClingGem(_) => "Cling Gem",
                            A::AscendantLight => "Ascendant Light",
                            A::SoulCutter => "Soul Cutter",
                            A::Indignation => "Indignation",
                            A::SolarWind => "Solar Wind",
                            A::Strikebreak => "Strikebreak",
                            A::Memento => "Memento",
                            A::HeliacalPower => "Heliacal Power",
                            A::AerialFinesse => "Aerial Finesse",
                            A::Pilgrimage => "Pilgrimage",
                            A::Empathy => "Empathy",
                            A::GoodGraces => "Good Graces",
                            A::MartialProwess => "Martial Prowess",
                            A::ClearMind => "Clear Mind",
                            A::Professional => "Professionalism",
                            A::Guardian => "a Guardian",
                            A::Soldier => "Chivalry",
                            A::BleedingHeart => "a Bleeding Heart",
                            A::Xix => "Nostalgia",
                            A::SolSister => "Devotion",
                            A::Classy => "Class",
                            A::Sleepytime => "Sweater",
                        }),
                        None
                )),
                // unnecessary since it's unused so not changed
                // Property::ObjectProperty(object_property::ObjectProperty {
                //     name: model,
                //     ..Default::default()
                // }),
                Property::IntProperty(int_property::IntProperty {
                    name: tier,
                    property_guid: guid,
                    value: match self {
                        A::DreamBreaker
                        | A::SunGreaves
                        | A::Slide
                        | A::Sunsetter
                        | A::ClingGem(_)
                        | A::AscendantLight
                        | A::SoulCutter
                        | A::Indignation
                        | A::SolarWind
                        | A::Strikebreak => 0,
                        _ => 1,
                    },
                    ..Default::default()
                }),
                Property::TextProperty(text(
                    instructions,
                    Some(match self {
                        A::DreamBreaker => "Press X Button/Left Click",
                        A::SunGreaves => "Press A Button/Space bar while in the air.",
                        A::Slide => "Press Left Trigger/Q Key on the ground to Slide.",
                        A::Sunsetter => "Press LT/Q key while in the air to Plunge downwards, which can then chain into a High Jump.",
                        A::ClingGem(_) => "Press RT/Left Shift while falling along walls to Ride them",
                        A::AscendantLight => "Attack objects and enemies to bounce off of them.",
                        A::SoulCutter => "Hold Attack until your weapon is ready, release to fire",
                        A::Indignation => "Higher Power gives you increased combat capabilities.",
                        A::SolarWind => "Jump while Sliding.",
                        A::Strikebreak => "Hold Attack until your weapon is ready, release to strike.",
                        A::Memento => "Press D-Pad Up/Tab to see your mental map",
                        A::HeliacalPower => "Allows you to perform an additional air kick",
                        A::AerialFinesse => "Press Jump during your knockback state to regain balance.",
                        A::Pilgrimage => "You can move slowly while channeling a Heal",
                        A::Empathy => "Bonus Power is built from attacks.",
                        A::GoodGraces => "Increases healing power by 1/2 pip",
                        A::MartialProwess => "Increases Damage dealt",
                        A::ClearMind => "Collect all 3 to hold more magical power",
                        _ => "Gained a new outfit!",
                    }),
                    None
                )),
                Property::TextProperty(text(
                    description,
                    match self {
                        A::DreamBreaker => Some(r#"
Attack freely while moving to vanquish foes.. Hiting enemies will build up magical power.

To use it, hold B Button/F Key to heal.
                        "#),
                        A::SunGreaves => Some(r#"
After a short delay, do an air kick. Hitting walls and objects with it will cause you to jump away. Can be done up to 3 times until reset.

Try kicking early to maximize distance! 
                        "#),
                        A::Slide => Some(r#"
Gain a boost of speed, and slide under tight gaps and some attacks. 
But by itself, an incomplete technique.
                        "#),
                        A::Sunsetter => Some(r#"
Use this to break through fragile floors or simply stop yourself in the air. 

Jumping right after the Plunge will result in a High Jump.
                        "#),
                        A::ClingGem(_) => Some(r#"
Hold tight to walls while you propel yourself a short distance. Can be jumped out of, and can go around curved walls and corners.
                        "#),
                        A::AscendantLight => Some(r#"
In addition, your weapon now glows brightly, even in the darkest of places. 
                        "#),
                        A::SoulCutter => Some(r#"
Enhances your charge attack with a long distance projectile. 
If you have power available, it will consume a level to enhance the damage.

Can be aimed in first person, if need be.
                        "#),
                        A::Indignation => Some(r#"
Upon reaching power level 2, the range of your attack is extended. 
At level 3, your damage is increased. 

This is reset upon spending it on a heal.
                        "#),
                        A::SolarWind => Some(r#"
Completes the technique of the slide. Cross long distances with a airy long jump. 
                        "#),
                        A::Strikebreak => Some(r#"
A more powerful attack that locks you in place. Can be used to destroy heavy barriers.
                        "#),
                        A::Memento => Some(r#"
To move the map, press LockOn + Move Camera while active.

Rooms will be revealed to you as you enter them, allowing you to more easily chart a course through an area.
                        "#),
                        _ => None
                    },
                    None
                )),
                Property::ArrayProperty(array_property::ArrayProperty {
                    name: extended,
                    property_guid: guid,
                    array_type,
                    value: match self {
                        A::DreamBreaker => vec![
                            Property::TextProperty(text(
                                FName::Dummy { value: 0.to_string(), number: 0 },
                                Some(r#"
Sybil will automatically hit whoever is closest, though you can also Lock-On with RB/Right Click.

Your weapon can be dropped, or thrown with LB/tab. If you ever lose it, you can recall it at a save point by holding down the heal button.                                "#),
                                None
                            )),
                        ],
                        A::SunGreaves => vec![
                            Property::TextProperty(text(
                                FName::Dummy { value: 0.to_string(), number: 0 },
                                Some(r#"
    The angle you come off of a wall is a reflection of the angle you are facing when you make contact. Don't try to push away from the wall until you bounce!

    If the angle is close to perpendicular, your jump will be higher but send you less far.
                                "#),
                                None
                            )),
                            Property::TextProperty(text(
                                FName::Dummy { value: 1.to_string(), number: 0 },
                                Some(r#"
Try to kick so that your foot hits the wall, not your body, for maxium distance. 

You can even kick quite early, so long as you touch the wall before the action ends.

Timing, spacing, and angle are all important!
                                "#),
                                None
                            ))
                        ],
                        A::Sunsetter => vec![
                            Property::TextProperty(text(
                                FName::Dummy { value: 0.to_string(), number: 0 },
                                Some(r#"
You can jump right after you start up the plunging action, which will cancel it into a backflip.

This can only be done once until reset, so be careful!
                                "#),
                                None
                            )),
                        ],
                        A::ClingGem(_) => vec![
                            Property::TextProperty(text(
                                FName::Dummy { value: 0.to_string(), number: 0 },
                                Some(r#"
You can Wall Ride 6 times until reset.

Activation is easier when you move along walls, not directly into them.
                                "#),
                                None
                            )),
                        ],
                        _ => vec![]
                    },
                    ..Default::default()
                }),
                Property::ObjectProperty(object_property::ObjectProperty {
                    name: display,
                    value: PackageIndex::new(match self {
                        A::Memento
                        | A::Professional
                        | A::Guardian
                        | A::Soldier
                        | A::BleedingHeart
                        | A::Xix
                        | A::SolSister
                        | A::Classy
                        | A::Sleepytime => 0,
                        _ => insert
                    }),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    }
}
