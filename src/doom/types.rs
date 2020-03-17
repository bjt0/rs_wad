use num::{FromPrimitive};
use num_derive::{FromPrimitive};

#[derive(Debug)]
pub enum DoomThingType {
    Unknown,
    DoomWeapon(DoomWeaponType),
    DoomAmmo(DoomAmmoType),
    DoomItem(DoomItemType),
    DoomMonster(DoomMonsterType),
    DoomPowerup(DoomPowerupType),
    DoomKey(DoomKeyType)
}

impl DoomThingType {
    pub fn from_type_number(num: u16) -> Self {
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

        return DoomThingType::Unknown;
    }
}

#[derive(Debug, FromPrimitive)]
pub enum DoomMonsterType {
    Unknown = -1,
    Zombieman = 18, 
    ShotgunGuy = 9,
    Imp = 3001,
    Demon = 3002,
    Spectre = 58,
    Cacodemon = 3005,
    LostSoul = 3006,
    BaronOfHell = 3003,
    Cyberdemon = 16,
    SpiderMastermind = 7
}

#[derive(Debug, FromPrimitive)]
pub enum DoomWeaponType {
    Shotgun = 2001,
    Chainsaw = 2005,
    Chaingun = 2002,
    RocketLauncher = 2003,
    PlasmaRifle = 2004,
    BFG9000 = 2006
}

#[derive(Debug, FromPrimitive)]
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

#[derive(Debug, FromPrimitive)]
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

#[derive(Debug, FromPrimitive)]
pub enum DoomPowerupType {
    GreenArmour = 2018,
    Backpack = 8,
    Medikit = 2012,
    BlueArmour = 2019,
    RadiationSuit = 2025,
    Stimpack = 2011
}

#[derive(Debug, FromPrimitive)]
pub enum DoomKeyType {
    RedKey = 13, 
    BlueKey = 5,
    YellowKey = 6,
    RedSkullKey = 38,
    BlueSkullKey = 40,
    YellowSkullKey = 39
}