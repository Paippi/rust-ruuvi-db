use crate::ruuvitag::Ruuvitag;
use ruuviscanner::ruuvitag::SensorDataV5;
use sqlx::postgres::PgPoolOptions;

pub async fn create_connection() -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://@localhost/foo")
        .await
        .unwrap()
}

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
