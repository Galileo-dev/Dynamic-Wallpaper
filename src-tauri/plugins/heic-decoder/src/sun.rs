use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_sun_pos(lat: f64, lon: f64) -> (f64, f64) {
    let unixtime = get_unix_time();
    println!("unix_time: {}", unixtime);
    let pos = sun::pos(unixtime, lat, lon);
    let az = pos.azimuth.to_degrees();
    let alt = pos.altitude.to_degrees();
    println!("The position of the sun is {}/{}", az, alt);
    return (az, alt);
}

fn get_unix_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    in_ms as i64
}
