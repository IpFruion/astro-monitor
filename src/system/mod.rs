pub struct System {
    pub battery_percentage: f32,
}

impl System {
    pub fn monitor() -> anyhow::Result<Self> {
        Ok(System {
            battery_percentage: 100.0,
        })
    }
}
