use ruuviscanner::ruuvitag::SensorDataV5;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, sqlx::FromRow, PartialEq)]
pub struct Ruuvitag {
    mac: String,
    temperature_millicelcius: i32,
    humidity: f64,
    pressure: rust_decimal::Decimal,
    acceleration_x: i32,
    acceleration_y: i32,
    acceleration_z: i32,
    battery_voltage: i32,
    tx_power: i16,
    movement_counter: i16,
    measurement_number: i32,
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

pub async fn write_to_db(ruuvi: &SensorDataV5, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let ruuvi_db = Ruuvitag::from_sensor_data(&ruuvi);
    let query = format!(
        "INSERT INTO ruuvitag_data Values ('{0}', {1}, {2}, {3}, {4}, {5}, {6}, {7}, {8}, {9}, {10})",
        ruuvi_db.mac,
        ruuvi_db.temperature_millicelcius,
        ruuvi_db.humidity,
        ruuvi_db.pressure,
        ruuvi_db.acceleration_x,
        ruuvi_db.acceleration_y,
        ruuvi_db.acceleration_z,
        ruuvi_db.battery_voltage,
        ruuvi_db.tx_power,
        ruuvi_db.movement_counter,
        ruuvi_db.measurement_number,
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn read_ruuvitag_data<'a>(pool: &sqlx::PgPool) -> Result<Vec<Ruuvitag>, sqlx::Error> {
    // let rows = sqlx::query("SELECT * FROM ruuvitag_data").fetch(pool);
    let ruuvitags = sqlx::query_as::<_, Ruuvitag>("SELECT * from ruuvitag_data")
        .fetch_all(pool)
        .await?;
    Ok(ruuvitags)
}
pub async fn create_connection() -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://@localhost/foo")
        .await
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = create_connection().await;
    let ruuvitags = read_ruuvitag_data(&pool).await.unwrap();
    println!("{:?}", ruuvitags);
    Ok(())
}

// TODO: Currently tests interfere with each other. Clean ups fail.
#[cfg(test)]
mod tests {
    use ruuviscanner::ruuvitag::{Acceleration, SensorDataV5};

    use crate::{create_connection, read_ruuvitag_data, write_to_db, Ruuvitag};

    async fn db_clean_up(pool: &sqlx::PgPool) {
        sqlx::query("TRUNCATE TABLE ruuvitag_data")
            .execute(pool)
            .await
            .unwrap();
    }

    fn create_sensor_data() -> SensorDataV5 {
        SensorDataV5::new(
            100,
            50,
            60,
            Acceleration::new(0, 1, 2),
            70,
            101,
            202,
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        )
    }
    fn create_sensor_data_zeroes() -> SensorDataV5 {
        SensorDataV5::new(
            0,
            0,
            0,
            Acceleration::new(0, 0, 0),
            0,
            0,
            0,
            [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        )
    }

    fn create_sensor_data_min() -> SensorDataV5 {
        SensorDataV5::new(
            i16::MIN,
            u16::MIN,
            u16::MIN,
            Acceleration::new(i16::MIN, i16::MIN, i16::MIN),
            u16::MIN,
            u8::MIN,
            u16::MIN,
            [u8::MIN, u8::MIN, u8::MIN, u8::MIN, u8::MIN, u8::MIN],
        )
    }

    fn create_sensor_data_max() -> SensorDataV5 {
        SensorDataV5::new(
            i16::MAX,
            u16::MAX,
            u16::MAX,
            Acceleration::new(i16::MAX, i16::MAX, i16::MAX),
            u16::MAX,
            u8::MAX,
            u16::MAX,
            [u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX, u8::MAX],
        )
    }

    #[tokio::test]
    async fn test_write_to_db() {
        let ruuvidata = create_sensor_data();
        let correct_result = Ruuvitag::from_sensor_data(&ruuvidata);

        let pool = create_connection().await;
        db_clean_up(&pool).await;
        let db_write_success = write_to_db(&ruuvidata, &pool).await;
        assert!(db_write_success.is_ok());

        let result = read_ruuvitag_data(&pool).await.unwrap();
        assert_eq!(result[0], correct_result);
        // db_clean_up(&pool).await;
    }

    #[tokio::test]
    async fn test_write_to_db_zeroes() {
        let ruuvidata = create_sensor_data_zeroes();
        let correct_result = Ruuvitag::from_sensor_data(&ruuvidata);

        let pool = create_connection().await;
        db_clean_up(&pool).await;
        let db_write_success = write_to_db(&ruuvidata, &pool).await;
        assert!(db_write_success.is_ok());

        let result = read_ruuvitag_data(&pool).await.unwrap();
        assert_eq!(result[0], correct_result);
        // db_clean_up(&pool).await;
    }

    #[tokio::test]
    async fn test_write_to_db_min() {
        let ruuvidata = create_sensor_data_min();
        let correct_result = Ruuvitag::from_sensor_data(&ruuvidata);

        let pool = create_connection().await;
        db_clean_up(&pool).await;
        let db_write_success = write_to_db(&ruuvidata, &pool).await;
        assert!(db_write_success.is_ok());

        let result = read_ruuvitag_data(&pool).await.unwrap();
        assert_eq!(result[0], correct_result);
        // db_clean_up(&pool).await;
    }

    #[tokio::test]
    async fn test_write_to_db_max() {
        let ruuvidata = create_sensor_data_max();
        let correct_result = Ruuvitag::from_sensor_data(&ruuvidata);

        let pool = create_connection().await;
        db_clean_up(&pool).await;
        let db_write_success = write_to_db(&ruuvidata, &pool).await;
        assert!(db_write_success.is_ok());

        let result = read_ruuvitag_data(&pool).await.unwrap();
        assert_eq!(result[0], correct_result);
    }

    #[tokio::test]
    async fn test_ruuvitag_from_sensor_data() {
        let ruuvi_sensor_data = SensorDataV5::new(
            100,
            50,
            60,
            Acceleration::new(0, 1, 2),
            70,
            101,
            202,
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        );

        let ruuvi_db_data = Ruuvitag::from_sensor_data(&ruuvi_sensor_data);
        let ruuvi_db_data_correct = Ruuvitag {
            mac: "11:22:33:44:55:66".to_owned(),
            temperature_millicelcius: 500,
            humidity: 0.125,
            pressure: 50060.into(),
            acceleration_x: 0,
            acceleration_y: 1,
            acceleration_z: 2,
            battery_voltage: 1602,
            tx_power: -34,
            movement_counter: 101,
            measurement_number: 202,
        };

        assert_eq!(ruuvi_db_data, ruuvi_db_data_correct);
    }
}
