use num::{FromPrimitive};
use num_derive::{FromPrimitive};

#[derive(Debug, PartialEq)]
pub enum ThingType {
    Unknown(u16),
    Start(StartType),
    Weapon(WeaponType),
    Ammo(AmmoType),
    Item(ItemType),
    Monster(MonsterType),
    Powerup(PowerupType),
    Key(KeyType),
    Obstacle(ObstacleType),
    Decoration(DecorationType),
    Editor(EditorType),
    Other(OtherType),
}

impl ThingType {
    pub fn from_type_number(num: u16) -> Self {
        let start_type = StartType::from_u16(num);

        if start_type.is_some() { return ThingType::Start(start_type.unwrap()) };

        let monster_type = MonsterType::from_u16(num);

        if monster_type.is_some() { return ThingType::Monster(monster_type.unwrap()) };

        let weapon_type = WeaponType::from_u16(num);

        if weapon_type.is_some() { return ThingType::Weapon(weapon_type.unwrap()) };

        let ammo_type = AmmoType::from_u16(num);

        if ammo_type.is_some() { return ThingType::Ammo(ammo_type.unwrap()) };

        let item_type = ItemType::from_u16(num);

        if item_type.is_some() { return ThingType::Item(item_type.unwrap()) };

        let powerup_type = PowerupType::from_u16(num);

        if powerup_type.is_some() { return ThingType::Powerup(powerup_type.unwrap()) };
        
        let key_type = KeyType::from_u16(num);

        if key_type.is_some() { return ThingType::Key(key_type.unwrap()) };

        let obstacle_type = ObstacleType::from_u16(num);

        if obstacle_type.is_some() { return ThingType::Obstacle(obstacle_type.unwrap()) };

        let deco_type = DecorationType::from_u16(num);

        if deco_type.is_some() { return ThingType::Decoration(deco_type.unwrap()) };

        let other_type = OtherType::from_u16(num);

        if other_type.is_some() { return ThingType::Other(other_type.unwrap()) };

        let editor_type = EditorType::from_u16(num); 

        if editor_type.is_some() { return ThingType::Editor(editor_type.unwrap()) };

        return ThingType::Unknown(num);
    }
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum MonsterType {
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
pub enum WeaponType {
    Shotgun = 2001,
    Chainsaw = 2005,
    Chaingun = 2002,
    RocketLauncher = 2003,
    PlasmaRifle = 2004,
    BFG9000 = 2006
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum AmmoType {
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
pub enum ItemType {
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
pub enum PowerupType {
    GreenArmour = 2018,
    Backpack = 8,
    Medikit = 2012,
    BlueArmour = 2019,
    RadiationSuit = 2025,
    Stimpack = 2011
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum KeyType {
    RedKey = 13, 
    BlueKey = 5,
    YellowKey = 6,
    RedSkullKey = 38,
    BlueSkullKey = 40,
    YellowSkullKey = 39
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum StartType {
    Player1Start = 1,
    Player2Start = 2,
    Player3Start = 3,
    Player4Start = 4,
    DeathmatchStart = 11
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum OtherType {
    TeleportLanding = 14,
    IconOfSinSpawnSpot = 87,
    JohnRomeroHead = 88,
    IconOfSinMonsterSpawner = 89, 
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum ObstacleType {
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
pub enum DecorationType {
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
pub enum EditorType {
    BuilderCamera = 32000
}