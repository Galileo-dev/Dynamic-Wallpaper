use ipgeolocate::{Locator, Service};

pub async fn get_location() -> (f64, f64) {
    let service = Service::IpApi; // Multiple Serivce options
                                  //todo(): add option to manually enter location

    if let Some(ip) = public_ip::addr().await {
        match Locator::get(&ip.to_string(), service).await {
            Ok(ip) => return (ip.latitude.parse().unwrap(), ip.longitude.parse().unwrap()),
            Err(_error) => return (37.3346, 122.0090), // Apple Cupertino ,
        };
        return (37.3346, 122.0090); // Apple Cupertino
    } else {
        println!("couldn't get an IP address");
        return (37.3346, 122.0090); // Apple Cupertino
    }
}
