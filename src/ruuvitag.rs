use ruuviscanner::ruuvitag::SensorDataV5;

#[derive(Debug, sqlx::FromRow, PartialEq)]
pub struct Ruuvitag {
    pub mac: String,
    pub temperature_millicelcius: i32,
    pub humidity: f64,
    pub pressure: rust_decimal::Decimal,
    pub acceleration_x: i32,
    pub acceleration_y: i32,
    pub acceleration_z: i32,
    pub battery_voltage: i32,
    pub tx_power: i16,
    pub movement_counter: i16,
    pub measurement_number: i32,
}

impl Ruuvitag {
    pub fn from_sensor_data(ruuvi: &SensorDataV5) -> Self {
        Ruuvitag {
            mac: ruuvi.mac_as_str(),
            temperature_millicelcius: ruuvi.temperature_in_millicelcius().into(),
            humidity: ruuvi.get_humidity().into(),
            pressure: ruuvi.get_pressure().into(),
            acceleration_x: ruuvi.acceleration.x.into(),
            acceleration_y: ruuvi.acceleration.y.into(),
            acceleration_z: ruuvi.acceleration.z.into(),
            battery_voltage: ruuvi.get_battery_voltage().into(),
            tx_power: ruuvi.get_tx_power().into(),
            movement_counter: ruuvi.movement_counter.into(),
            measurement_number: ruuvi.measurement_number.into(),
        }
    }
}

// TODO: debug later these should be the real values... Something wrong in the database types

// #[derive(sqlx::FromRow)]
// struct Ruuvitag {
//     mac: String,
//     temperature_millicelcius: i16,
//     //     humidity: f64,
//     //     pressure: u32,
//     //     acceleration_x: i16,
//     //     acceleration_y: i16,
//     //     acceleration_z: i16,
//     //     battery_voltage: u16,
//     //     tx_power: i8,
//     //     movement_counter: u8,
//     //     measurement_number: u16,
// }
