use doom::*;

pub enum DoomMonsterType {
    Unknown = -1,
    Zombieman = 18, 
    ShotgunGuy = 9,
    Imp = 3001,
    Demon = 3002
}

pub enum DoomThingType {
    DoomMonster(DoomMonsterType)
}

pub enum DoomThingFlags {
    EasyDifficulty,
    MediumDifficulty,
    HardDifficulty,
    Ambush,
    DeathmatchOnly
}

pub struct DoomThing {
    location: DoomPoint,
    direction: DoomDirection,
    thing_type: DoomThingType,
    thing_flags: DoomThingFlags
}