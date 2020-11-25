use ipgeolocate::Locator;

// Prints all the possible data points for 1.1.1.1.
fn main() {
    match Locator::freegeoip("1.1.1.1") {
      Ok(ip) => println!("freegeoip: {}\nLatitude: {}\nLongitude: {}\nCity: {}\nRegion: {}\nCountry: {}\nTimezone: {}", ip.ip, ip.latitude, ip.longitude, ip.city, ip.region, ip.country, ip.timezone),
      Err(error) => println!("Error getting data: {}", error),
    };
}
