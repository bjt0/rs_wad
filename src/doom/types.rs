use num::{FromPrimitive};
use num_derive::{FromPrimitive};

#[derive(Debug, PartialEq)]
pub enum DoomThingType {
    Unknown(u16),
    DoomStart(DoomStartType),
    DoomWeapon(DoomWeaponType),
    DoomAmmo(DoomAmmoType),
    DoomItem(DoomItemType),
    DoomMonster(DoomMonsterType),
    DoomPowerup(DoomPowerupType),
    DoomKey(DoomKeyType),
    DoomObstacle(DoomObstacleType),
    DoomDecoration(DoomDecorationType),
    DoomEditor(DoomEditorType),
    DoomOther(DoomOtherType),
}

impl DoomThingType {
    pub fn from_type_number(num: u16) -> Self {
        let start_type = DoomStartType::from_u16(num);

        if start_type.is_some() { return DoomThingType::DoomStart(start_type.unwrap()) };

        let monster_type = DoomMonsterType::from_u16(num);

        if monster_type.is_some() { return DoomThingType::DoomMonster(monster_type.unwrap()) };

        let weapon_type = DoomWeaponType::from_u16(num);

        if weapon_type.is_some() { return DoomThingType::DoomWeapon(weapon_type.unwrap()) };

        let ammo_type = DoomAmmoType::from_u16(num);

        if ammo_type.is_some() { return DoomThingType::DoomAmmo(ammo_type.unwrap()) };

        let item_type = DoomItemType::from_u16(num);

        if item_type.is_some() { return DoomThingType::DoomItem(item_type.unwrap()) };

        let powerup_type = DoomPowerupType::from_u16(num);

        if powerup_type.is_some() { return DoomThingType::DoomPowerup(powerup_type.unwrap()) };
        
        let key_type = DoomKeyType::from_u16(num);

        if key_type.is_some() { return DoomThingType::DoomKey(key_type.unwrap()) };

        let obstacle_type = DoomObstacleType::from_u16(num);

        if obstacle_type.is_some() { return DoomThingType::DoomObstacle(obstacle_type.unwrap()) };

        let deco_type = DoomDecorationType::from_u16(num);

        if deco_type.is_some() { return DoomThingType::DoomDecoration(deco_type.unwrap()) };

        let other_type = DoomOtherType::from_u16(num);

        if other_type.is_some() { return DoomThingType::DoomOther(other_type.unwrap()) };

        let editor_type = DoomEditorType::from_u16(num); 

        if editor_type.is_some() { return DoomThingType::DoomEditor(editor_type.unwrap()) };

        return DoomThingType::Unknown(num);
    }
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomMonsterType {
    Arachnotron = 68, 
    Archvile = 64, 
    BaronOfHell = 3003,
    Cacodemon = 3005, 
    CommanderKeen = 72, 
    Cyberdemon = 16, 
    Demon = 3002, 
    Chaingunner = 65, 
    HellKnight = 69, 
    Imp = 3001, 
    LostSoul = 3006, 
    Mancubus = 67, 
    PainElemental = 71, 
    Revenant = 66, 
    ShotgunGuy = 9,
    Spectre = 58, 
    Spiderdemon = 7, 
    WolfensteinSS = 84, 
    Zombieman = 3004
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomWeaponType {
    Shotgun = 2001,
    Chainsaw = 2005,
    Chaingun = 2002,
    RocketLauncher = 2003,
    PlasmaRifle = 2004,
    BFG9000 = 2006
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomAmmoType {
    ShotgunShells = 2008,
    BoxOfBullets = 2048,
    BoxOfRockets = 2046,
    BoxOfShotgunShells = 2049,
    Clip = 2007,
    EnergyCell = 2047,
    EnergyPack = 17,
    Rocket = 2010
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomItemType {
    ArmourBonus = 2015,
    Berserk = 2023,
    ComputerAreaMap = 2026,
    HealthBonus = 2014,
    Invulnerability = 2022,
    LightAmplificationVisor = 2045,
    Megasphere = 83,
    Invisibility = 2024,
    Supercharge = 2013
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomPowerupType {
    GreenArmour = 2018,
    Backpack = 8,
    Medikit = 2012,
    BlueArmour = 2019,
    RadiationSuit = 2025,
    Stimpack = 2011
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomKeyType {
    RedKey = 13, 
    BlueKey = 5,
    YellowKey = 6,
    RedSkullKey = 38,
    BlueSkullKey = 40,
    YellowSkullKey = 39
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomStartType {
    Player1Start = 1,
    Player2Start = 2,
    Player3Start = 3,
    Player4Start = 4,
    DeathmatchStart = 11
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomOtherType {
    TeleportLanding = 14,
    IconOfSinSpawnSpot = 87,
    JohnRomeroHead = 88,
    IconOfSinMonsterSpawner = 89, 
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomObstacleType {
    BrownStump = 47, 
    BurningBarrel = 70, 
    BurntTree = 43, 
    Candelabra = 35, 
    EvilEye = 41, 
    ExplodingBarrel = 2035, 
    FiveSkullPole = 28,
    FloatingSkull = 42, 
    FloorLamp = 2028, 
    HangingLeg = 53, 
    HangingLegPair = 52,
    HangingTorsoNoBrain = 78, 
    HangingTorsoLookingDown = 75, 
    HangingTorsoLookingUp = 77, 
    HangingTorsoOpenSkull = 76,
    // some of these are duplicated in decoration type but these block
    HangingVictimArmsOut = 50, 
    HangingVictimNoBrainsOrGuts = 74,
    HangingVictimGutsRemoved = 73,
    HangingVictimOneLeg = 51, 
    HangingVictimTwitching = 49,
    ImpaledHuman = 25,
    LargeBrownTree = 54, 
    PileOfSkullsWithCandles = 29,
    ShortBlueFirestick = 55, 
    ShortGreenFirestick = 56, 
    ShortGreenPillar = 31,
    ShortGreenPillarWithHeart = 36, 
    ShortRedFirestick = 57, 
    ShortRedPillar = 33,
    ShortRedPillarWithSkull = 37, 
    ShortTechnoFloorLamp = 86, 
    SkullOnPole = 27,
    TallBlueFirestick = 44, 
    TallGreenFirestick = 45,
    TallGreenPillar = 30,
    TallRedFirestick = 46, 
    TallRedPillar = 32, 
    TallTechnoColumn = 48, 
    TallTechnoFloorLamp = 85, 
    TwitchingImpaledHuman = 26
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomDecorationType {
    BloodyMess1 = 10,
    BloodyMess2 = 12, 
    Candle = 34, 
    DeadCacodemon = 22,
    DeadDemon = 21,
    DeadZombieman = 18,
    DeadShotgunGuy = 19,
    DeadImp = 20, 
    DeadLostSoul = 23, // doesn't show anything
    DeadPlayer = 15, 
    HangingLeg = 62,
    HangingPairOfLegs = 60,
    HangingVictimArmsOut = 59,
    HangingVictimOneLeg = 61, 
    HangingVictimTwitching = 63,
    PoolOfBlood1 = 79, 
    PoolOfBlood2 = 80, 
    PoolOfBloodAndFlesh = 24, 
    PoolOfBrains = 81,
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoomEditorType {
    DoomBuilderCamera = 32000
}