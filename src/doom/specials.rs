use num::{FromPrimitive};
use num_derive::{FromPrimitive};

#[derive(Debug, PartialEq)]
pub enum LinedefSpecial {
    Unknown(u16),
    Door(DoorLineSpecial)
}

impl LinedefSpecial {
    pub fn from_type_number(num: u16) -> Self {
        let door_type = DoorLineSpecial::from_u16(num);

        if door_type.is_some() { return LinedefSpecial::Door(door_type.unwrap()) };

        LinedefSpecial::Unknown(num)
    }
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DoorLineSpecial {
    SlowOpenWaitClose = 1,
    FastOpenWaitClose = 117,
    SlowOpenWaitCloseSwitch = 63,
    FastOpenWaitCloseSwitch = 114,
    SlowOpenWaitCloseSwitchSingle = 29,
    FastOpenWaitCloseSwitchSingle = 111,
    SlowOpenWaitCloseSwitchWalkover = 90,
    FastOpenWaitCloseSwitchWalkover = 105,
    SlowOpenWaitCloseSwitchWalkoverSingle = 4,
    FastOpenWaitCloseSwitchWalkoverSingle = 108,
    SlowOpenStaySingle = 31,
    FastOpenStaySingle = 118,
    SlowOpenStaySwitch = 61,
    FastOpenStaySwitch = 115,
    SlowOpenStaySwitchSingle = 103,
    FastOpenStaySwitchSingle = 112,
    SlowOpenStayWalkover = 86,
    FastOpenStayWalkover = 106,
    SlowOpenStayWalkoverSingle = 2,
    FastOpenStayWalkoverSingle = 109,
    SlowOpenStayShootable = 46,
    SlowCloseStaySwitch = 42,
    FastCloseStaySwitch = 116,
    SlowCloseStaySwitchSingle = 50,
    FastCloseStaySwitchSingle = 113,
    SlowCloseStayWalkover = 75,
    FastCloseStayWalkover = 107,
    SlowCloseStayWalkoverSingle = 3,
    FastCloseStayWalkoverSingle = 110,
    SlowCloseWaitOpenWalkover = 76,
    SlowCloseWaitOpenWalkoverSingle = 16,
    // locked door types
    SlowOpenStayBlueKeySingle = 32,
    SlowOpenStayRedKeySingle  = 33,
    SlowOpenStayYellowKeySingle = 34,
    // fast lock
    FastOpenStayBlueKeySwitch = 99,
    FastOpenStayRedKeySwitch  = 134,
    FastOpenStayYellowKeySwitch = 136,
    // fast lock switch
    FastOpenStayBlueKeySwitchSingle = 133,
    FastOpenStayRedKeySwitchSingle  = 135,
    FastOpenStayYellowKeySwitchSingle = 137,
    // fast press repeatable
    SlowOpenWaitCloseBlueKey = 26,
    SlowOpenWaitCloseRedKey  = 28,
    SlowOpenWaitCloseYellowKey = 27,
}