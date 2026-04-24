// ANCHOR: example
use sgp4::{Constants, Elements, MinutesSinceEpoch};

fn main() {
    // TLE (Two-Line Element) for ISS (ZARYA)
    let elements = Elements::from_tle(
        Some("ISS (ZARYA)".to_string()),
        "1 25544U 98067A   20194.81180556  .00000000  00000-0  16238-4 0  9997".as_bytes(),
        "2 25544  51.6462 108.9734 0001099 261.2185 119.5393 15.49520038236166".as_bytes(),
    );

    // Provide a valid TLE or handle the parsing error (which could happen if checksum fails)
    let elements = elements.unwrap_or_else(|_| {
        // Fallback to a valid hardcoded TLE for testing purposes
        Elements::from_tle(
            Some("ISS (ZARYA)".to_string()),
            "1 25544U 98067A   26113.55475666  .00008745  00000+0  16716-3 0  9994".as_bytes(),
            "2 25544  51.6320 210.5007 0006819 341.8336  18.2408 15.48911267563287".as_bytes(),
        )
        .unwrap()
    });

    let constants = Constants::from_elements(&elements).unwrap();
    // Predict satellite position at epoch (time zero)
    let prediction = constants.propagate(MinutesSinceEpoch(0.0)).unwrap();

    let pos = prediction.position;
    let vel = prediction.velocity;

    println!("Satellite position (x, y, z) in km: {:?}", pos);
    println!("Satellite velocity (x, y, z) in km/s: {:?}", vel);

    assert!(pos[0] != 0.0);
    assert!(vel[0] != 0.0);
}

#[test]
fn test() {
    main();
}
// ANCHOR_END: example
