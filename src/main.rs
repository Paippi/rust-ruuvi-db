use rust_ruuvi_db::db::{create_connection, read_ruuvitag_data};

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
