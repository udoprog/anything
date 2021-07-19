#[test]
fn test_temperatures() {
    assert_query!("0°C to °F", 32, fahrenheit);
    assert_query!("0°C to K", 27315 / 100, K);

    assert_query!("0°F to K", 45967 / 180, K);
    assert_query!("32°F to °C", 0, celsius);
    assert_query!("32°F to K", 27315 / 100, K);
}
