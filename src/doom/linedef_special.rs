#[derive(Debug, PartialEq)]
pub enum LinedefSpecial {
    Unknown(u16),
    Door(TriggerType),
}

impl LinedefSpecial {
    pub fn from_type_number(num: u16) -> Self {
        LinedefSpecial::Unknown(num)
    }
}

#[derive(Debug, PartialEq)]
pub enum TriggerType {
    PushOnce,
    PushRepeatable,
    WalkoverOnce,
    WalkoverRepeatable,
    SwitchOnce,
    SwitchRepeatable,
    ShootOnce,
    ShootRepeatable
}