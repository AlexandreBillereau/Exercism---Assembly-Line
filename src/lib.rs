const CAR_BY_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0      => 0.0,
        1..=4  => calc_production(speed, 1.0),
        5..=8  => calc_production(speed, 0.9),
        9..=10 => calc_production(speed, 0.77),
        _      => panic!("error")
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}

fn calc_production(speed: u8, rate: f64) -> f64 {
    speed as f64 * CAR_BY_HOUR * rate
}
