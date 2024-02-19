#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
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
    ClingGem,
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
            A::ClingGem => "icon_clingGem",
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
            A::Professional => "no icon whoopsies :p",
        };
        let path = format!("/Game/MatTex/Textures/UI/icons/{icon}");
        if !matches!(self, A::Professional) {
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
                            A::ClingGem => "Cling Gem",
                            A::AscendantLight => "Ascendant Light",
                            A::SoulCutter => "Soul Cutter",
                            A::Indignation => "Indignation",
                            A::SolarWind => "Solar Wind",
                            A::Strikebreak => "Strikebreak",
                            A::HeliacalPower => "Heliacal Power",
                            A::AerialFinesse => "Aerial Finesse",
                            A::Pilgrimage => "Pilgrimage",
                            A::Empathy => "Empathy",
                            A::GoodGraces => "Good Graces",
                            A::MartialProwess => "Martial Prowess",
                            A::ClearMind => "Clear Mind",
                            A::Professional => "Professionalism",
                        }),
                        Some(match self {
                            A::DreamBreaker => "CCB981A54F25BC695627E1BCC90A104A",
                           A::SunGreaves => "F5EBEAAF4DE4C2FDBB8ECE8C3F09044F",
                           A::Slide => "93169A98407AD262918B48B6FCEEBBB8",
                           A::Sunsetter => "7713B83844855522BE804393CBF59B35",
                           A::ClingGem => "154C81074176B157AFF21D9EE30A0606",
                           A::AscendantLight => "F889BC4F42EE8304568F558DB19F73CB",
                           A::SoulCutter => "7F34D58F47D6ACA7568298B166EBE3A7",
                           A::Indignation => "79D70BDB46BE7CF0D19D63819605CC47",
                           A::SolarWind => "9B573C6C43F4B5B5B38F149367AD1680",
                           A::Strikebreak => "D678EA3947EC99FA253C388E2A0B6FC7",
                           A::HeliacalPower => "FF19B239411805411653D2A2715A2F11",
                           A::AerialFinesse => "F42BB6BA4FD3F8F0822EEE9D2BD50488",
                           A::Pilgrimage => "4EE62E7E4AB4796E14188E9FFD90FBF5",
                           A::Empathy => "6E6D1DC4448DB7EDE21FB3AA1B4D51CF",
                           A::GoodGraces => "03449EA5472B2D5E246D24A9CF49AC6A",
                           A::MartialProwess => "2CC3BE174578D26BE2FD0B8B5C66F695",
                           A::ClearMind => "12DA2B1E40028ACC4E2D829F6E793C0C",
                           A::Professional => "F6B4AB55471722FE35E629A629BB2912",
                        }),
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
                        | A::ClingGem
                        | A::AscendantLight
                        | A::SoulCutter
                        | A::Indignation
                        | A::SolarWind
                        | A::Strikebreak => 0,
                        A::HeliacalPower
                        | A::AerialFinesse
                        | A::Pilgrimage
                        | A::Empathy
                        | A::GoodGraces
                        | A::MartialProwess
                        | A::ClearMind
                        | A::Professional => 1,
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
                        A::ClingGem => "Press RT/Left Shift while falling along walls to Ride them",
                        A::AscendantLight => "Attack objects and enemies to bounce off of them.",
                        A::SoulCutter => "Hold Attack until your weapon is ready, release to fire",
                        A::Indignation => "Higher Power gives you increased combat capabilities.",
                        A::SolarWind => "Jump while Sliding.",
                        A::Strikebreak => "Hold Attack until your weapon is ready, release to strike.",
                        A::HeliacalPower => "Allows you to perform an additional air kick",
                        A::AerialFinesse => "Press Jump during your knockback state to regain balance.",
                        A::Pilgrimage => "You can move slowly while channeling a Heal",
                        A::Empathy => "Bonus Power is built from attacks.",
                        A::GoodGraces => "Increases healing power by 1/2 pip",
                        A::MartialProwess => "Increases Damage dealt",
                        A::ClearMind => "Collect all 3 to hold more magical power",
                        A::Professional => "Gained a new outfit!",
                    }),
                    Some(match self {
                        A::DreamBreaker => "ED81FDC74C228DD2A8D27B8D1C6E8539",
                        A::SunGreaves => "BE6A54A04043100F0B3A9FAF700AC4AB",
                        A::Slide => "308F9AEC4FCF7A9A78840BAA38F5CB3D",
                        A::Sunsetter => "5B587B464EA3C43149F28D9F2B03C6AC",
                        A::ClingGem => "1E99C4744CC23033138F8997D5D7B698",
                        A::AscendantLight => "392D2D8E4A14E17034C3C291F90802E3",
                        A::SoulCutter => "FB2BCAA74CA827DCE37DF0B03E07CFCC",
                        A::Indignation => "EE34C0D54B8DEA46FAE5D4AAFACE22C1",
                        A::SolarWind => "6C8EC4BD4D0EDBD20BE87E8536495A35",
                        A::Strikebreak => "4DB8B10D4DE9B03D2519F3A1A3C9C7E7",
                        A::HeliacalPower => "971AB7064048B930265BC8AE7561F450",
                        A::AerialFinesse => "ED9FB0AB4114786F867D0784B6C85F07",
                        A::Pilgrimage => "F252159C4F2BE04A564188B59E08AEC5",
                        A::Empathy => "81D2B7994D6E0A83AAB34085B5CDD016",
                        A::GoodGraces => "CE6A5F67429DF8A472C501AB5EB107E3",
                        A::MartialProwess => "7BAA2F7F42702CDCE740DBAADFE50E72",
                        A::ClearMind => "074AB9BD4AA56D42ED5EDB886CE54BA1",
                        A::Professional => "3B68A7734F06D6C661F637BF594B722E",
                    }),
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
                        A::ClingGem => Some(r#"
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
                        A::HeliacalPower |
                        A::AerialFinesse |
                        A::Pilgrimage |
                        A::Empathy |
                        A::GoodGraces |
                        A::MartialProwess |
                        A::ClearMind |
                        A::Professional => None
                    },
                    match self {
                        A::DreamBreaker => Some("57BBC1C1418BCC8D7E67C784F5C803F5"),
                        A::SunGreaves => Some("9EF681CE4028EDD178E0AC874046D521"),
                        A::Slide => Some("FD693F06487DFFD545C65EBCF8F30D35"),
                        A::Sunsetter => Some("58AA68FC48C82F69376623868F960C0F"),
                        A::ClingGem => Some("2F4E91DA4B633EB9DB393790550EBA19"),
                        A::AscendantLight => Some("0CAF88DE4124BED45A484FAC8D5DF9E4"),
                        A::SoulCutter => Some("71FE2CAB4DD98A5BE9BFAD88C9DFDFBA"),
                        A::Indignation => Some("918EEC2D441E4E18FF9243A0DD5A27DF"),
                        A::SolarWind => Some("918EEC2D441E4E18FF9243A0DD5A27DF"),
                        A::Strikebreak => Some("806DBB1F43EA6E067165E08481CB928B"),
                        A::HeliacalPower |
                        A::AerialFinesse |
                        A::Pilgrimage |
                        A::Empathy |
                        A::GoodGraces |
                        A::MartialProwess |
                        A::ClearMind |
                        A::Professional => None
                    }
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
                                Some("018BC821440C5DC63F35AEB8B29D0A30")
                            )),
                        ],
                        A::SunGreaves => vec![
                            Property::TextProperty(text(
                                FName::Dummy { value: 0.to_string(), number: 0 },
                                Some(r#"
    The angle you come off of a wall is a reflection of the angle you are facing when you make contact. Don't try to push away from the wall until you bounce!

    If the angle is close to perpendicular, your jump will be higher but send you less far.
                                "#),
                                Some("D01124844521158B8985A1B21EC0FDEE")
                            )),
                            Property::TextProperty(text(
                                FName::Dummy { value: 1.to_string(), number: 0 },
                                Some(r#"
Try to kick so that your foot hits the wall, not your body, for maxium distance. 

You can even kick quite early, so long as you touch the wall before the action ends.

Timing, spacing, and angle are all important!
                                "#),
                                Some("32C29CCB402114784889E59BC6891BB0")
                            ))
                        ],
                        A::Sunsetter => vec![
                            Property::TextProperty(text(
                                FName::Dummy { value: 0.to_string(), number: 0 },
                                Some(r#"
You can jump right after you start up the plunging action, which will cancel it into a backflip.

This can only be done once until reset, so be careful!
                                "#),
                                Some("1ED62D7A4F230567C3946DA4FE08CE2B")
                            )),
                        ],
                        A::ClingGem => vec![
                            Property::TextProperty(text(
                                FName::Dummy { value: 0.to_string(), number: 0 },
                                Some(r#"
You can Wall Ride 6 times until reset.

Activation is easier when you move along walls, not directly into them.
                                "#),
                                Some("6252D3E94E550942EC7F32AC135C0E0E")
                            )),
                        ],
                        A::Slide |
                        A::AscendantLight |
                        A::SoulCutter |
                        A::Indignation |
                        A::SolarWind |
                        A::Strikebreak |
                        A::HeliacalPower |
                        A::AerialFinesse |
                        A::Pilgrimage |
                        A::Empathy |
                        A::GoodGraces |
                        A::MartialProwess |
                        A::ClearMind |
                        A::Professional => vec![]
                    },
                    ..Default::default()
                }),
                Property::ObjectProperty(object_property::ObjectProperty {
                    name: display,
                    value: PackageIndex::new(match self {
                        A::Professional => 0,
                        _=>insert
                    }),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    }
}
