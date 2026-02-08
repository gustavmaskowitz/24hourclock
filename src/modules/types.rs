use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Timezone {
    Dallas,
    Connecticut,
    London,
}

impl Timezone {
    pub fn offset(&self) -> i32 {
        match self {
            Timezone::Dallas => -6,
            Timezone::Connecticut => -5,
            Timezone::London => 0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Timezone::Dallas => "Dallas",
            Timezone::Connecticut => "Connecticut",
            Timezone::London => "London",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Timezone::Dallas => "DAL",
            Timezone::Connecticut => "CT",
            Timezone::London => "LON",
        }
    }

    pub const ALL: [Timezone; 3] = [Timezone::Dallas, Timezone::Connecticut, Timezone::London];
}

impl fmt::Display for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Meeting {
    pub id: u32,
    pub utc_hour: u32,
    pub title: String,
    pub essential: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectedSlot {
    pub utc_hour: u32,
    pub dallas_hour: f64,
    pub connecticut_hour: f64,
    pub london_hour: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RingAssignments {
    pub outer: Timezone,
    pub middle: Timezone,
    pub inner: Timezone,
}

impl Default for RingAssignments {
    fn default() -> Self {
        Self {
            outer: Timezone::Dallas,
            middle: Timezone::Connecticut,
            inner: Timezone::London,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ring {
    Outer,
    Middle,
    Inner,
}

// SVG constants matching original exactly (viewBox 0 0 400 400)
pub const CX: f64 = 200.0;
pub const CY: f64 = 200.0;
pub const OUTER_OUTER_R: f64 = 192.0;
pub const OUTER_INNER_R: f64 = 156.0;
pub const MIDDLE_OUTER_R: f64 = 152.0;
pub const MIDDLE_INNER_R: f64 = 116.0;
pub const INNER_OUTER_R: f64 = 112.0;
pub const INNER_INNER_R: f64 = 70.0;
pub const CENTER_R: f64 = 54.0;
pub const BG_R: f64 = 196.0;
pub const WORK_START: f64 = 9.0;
pub const WORK_END: f64 = 18.0;
