use wasm_bindgen::prelude::*;

const EARTH_RADIUS_KM: f64 = 6_371.008_8;

/// Return the great-circle distance between two WGS84 coordinates in kilometers.
///
/// Invalid latitude or longitude values return `NaN`, which maps naturally to
/// JavaScript and keeps the exported contract small.
#[wasm_bindgen]
pub fn haversine_distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    if !valid_coordinate(lat1, lon1) || !valid_coordinate(lat2, lon2) {
        return f64::NAN;
    }

    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let delta_lat = lat2 - lat1;
    let delta_lon = (lon2 - lon1).to_radians();

    let a =
        (delta_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
    2.0 * EARTH_RADIUS_KM * a.sqrt().atan2((1.0 - a).sqrt())
}

fn valid_coordinate(latitude: f64, longitude: f64) -> bool {
    latitude.is_finite()
        && longitude.is_finite()
        && (-90.0..=90.0).contains(&latitude)
        && (-180.0..=180.0).contains(&longitude)
}

#[cfg(test)]
mod tests {
    use super::haversine_distance_km;

    #[test]
    fn distance_from_bern_to_lausanne_is_reasonable() {
        let distance = haversine_distance_km(46.948, 7.4474, 46.5197, 6.6323);
        assert!((distance - 78.4).abs() < 0.5);
    }

    #[test]
    fn identical_points_have_zero_distance() {
        assert_eq!(haversine_distance_km(46.948, 7.4474, 46.948, 7.4474), 0.0);
    }

    #[test]
    fn invalid_coordinates_return_nan() {
        assert!(haversine_distance_km(91.0, 0.0, 0.0, 0.0).is_nan());
    }
}
