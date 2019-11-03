#[derive(Debug)]
pub enum Activity {
    Unknown(u8),
    InVehicle(u8),
    OnBicycle(u8),
    OnFoot(u8),
    Walking(u8),
    Running(u8),
    Still(u8),
    InRoadVehicle(u8),
    InRailVehicle(u8),
    InFourWheelerVehicle(u8),
    InCar(u8),
}

impl Activity {
    pub fn from_str(s: &str, v: u8) -> Option<Activity> {
        match s {
            "UNKNOWN" => Some(Activity::Unknown(v)),
            "IN_VEHICLE" => Some(Activity::InVehicle(v)),
            "ON_BICYCLE" => Some(Activity::OnBicycle(v)),
            "ON_FOOT" => Some(Activity::OnFoot(v)),
            "WALKING" => Some(Activity::Walking(v)),
            "RUNNING" => Some(Activity::Running(v)),
            "STILL" => Some(Activity::Still(v)),
            "IN_ROAD_VEHICLE" => Some(Activity::InRoadVehicle(v)),
            "IN_RAIL_VEHICLE" => Some(Activity::InRailVehicle(v)),
            "IN_FOUR_WHEELER_VEHICLE" => Some(Activity::InFourWheelerVehicle(v)),
            "IN_CAR" => Some(Activity::InCar(v)),
            _ => None,
        }
    }
}

pub struct ActivityBuilder {
    invalid: bool,
    r#type: Option<String>,
    confidence: Option<u8>,
}

impl ActivityBuilder {
    pub fn new() -> ActivityBuilder {
        ActivityBuilder {
            invalid: false,
            r#type: None,
            confidence: None,
        }
    }

    pub fn r#type(&mut self, value: String) -> &mut ActivityBuilder {
        self.r#type = Some(value);
        self
    }

    pub fn confidence(&mut self, value: u8) -> &mut ActivityBuilder {
        if value > 100 {
            self.invalid = true;
        } else {
            self.confidence = Some(value);
        }
        self
    }

    pub fn build(self) -> Result<Activity, ()> {
        if !self.invalid && self.r#type.is_some() && self.confidence.is_some() {
            let activity =
                Activity::from_str(self.r#type.unwrap().as_str(), self.confidence.unwrap());
            if activity.is_some() {
                Ok(activity.unwrap())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
