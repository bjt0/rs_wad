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
    SlowOpenStaySwitchSingle = 103
}