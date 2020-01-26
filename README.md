An unofficial Google Maps Platform client for the Rust programming language.
This client currently implements the Directions API, Distance Matrix API,
Elevation API, Geocoding API, and Time Zone API.

# Welcome

This crate is expected to work well and have the more important Google Maps
features implemented. It should work well because Reqwest and Serde do most of
the heavy lifting! While it's an early release, this crate should work fine as
is for most people.

I created this library because I needed several Google Maps Platform features
for a project that I'm working on. So, I've decided to spin my library off into
a public crate. This is a very small token of gratitude and an attempt to give
back to the Rust community. I hope it saves someone out there some work.

I would like for you to be successful with your project! If this crate is not
working for you, doesn't work how you think it should, you have requests, or
suggestions - please contact me. I'm not always fast at responding but I will
respond. Thanks!

## Example Directions API Request

```rust
use google_maps::*;

let directions = DirectionsRequest::new(
    YOUR_GOOGLE_API_KEY_HERE,
    // Canadian Museum of Nature
    Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    // Canada Science and Technology Museum
    Location::LatLng { lat: 45.403509, lng: -75.618904 },
)
.with_travel_mode(TravelMode::Transit)
.with_arrival_time(PrimitiveDateTime::new(
    Date::try_from_ymd(2020, 1, 20).unwrap(),
    Time::try_from_hms(13, 00, 0).unwrap()
))
.execute().unwrap();

println!("{:#?}", directions);
```

## Example Distance Matrix API Request

```rust
use google_maps::*;

// Example request:

let distance_matrix = DistanceMatrixRequest::new(
    YOUR_GOOGLE_API_KEY_HERE,
    // Origins
    vec![
        // Microsoft
        Waypoint::Address(String::from("One Microsoft Way, Redmond, WA 98052, United States")),
        // Apple
        Waypoint::Address(String::from("Infinite Loop, Cupertino, CA 95014, United States")),
    ],
    // Destinations
    vec![
        // Google
        Waypoint::PlaceId(String::from("ChIJj61dQgK6j4AR4GeTYWZsKWw")),
        // Mozilla
        Waypoint::LatLng { lat: 37.387316, lng: -122.060008 },
    ],
).execute().unwrap();

// Dump entire response:

println!("{:#?}", distance_matrix);
```

## Example Elevation API Positional Request

```rust
use google_maps::*;

// Example request:

let elevation = ElevationRequest::new(YOUR_GOOGLE_API_KEY_HERE)
.positional_request(ElevationLocations::LatLngs(vec![
    // Denver, Colorado, the "Mile High City"
    LatLng { lat: 39.7391536, lng: -104.9847034 },
]))
.execute().unwrap();

// Dump entire response:

println!("{:#?}", elevation);

// Parsing example:

println!("Elevation: {} meters", elevation.results.unwrap()[0].elevation);
```

## Example Geocoding API Request

```rust
use google_maps::*;

// Example request:

let location = GeocodingRequest::new(YOUR_GOOGLE_API_KEY_HERE)
.with_address("10 Downing Street London")
.execute().unwrap();

// Dump entire response:

println!("{:#?}", location);

// Parsing example:

for result in &location.results {
    println!(
        "Latitude: {:.7}, Longitude: {:.7}",
        result.geometry.location.lat, result.geometry.location.lng
    );
}
```

## Example Reverse Geocoding API Request

```rust
use google_maps::*;

// Example request:

let location = GeocodingReverseRequest::new(
    YOUR_GOOGLE_API_KEY_HERE,
    // 10 Downing St, Westminster, London
    LatLng { lat: 51.5033635, lng: -0.1276248 }
)
.with_result_type(PlaceType::StreetAddress)
.execute().unwrap();

// Dump entire response:

println!("{:#?}", location);

// Parsing example:

for result in &location.results {
    for address_component in &result.address_components {
        print!("{} ", address_component.short_name);
    }
    println!(""); // New line.
}
```

## Example Time Zone API Request

```rust
use google_maps::*;

// Example request:

let time_zone = TimeZoneRequest::new(
    YOUR_GOOGLE_API_KEY_HERE,
    // St. Vitus Cathedral in Prague, Czechia
    LatLng { lat: 50.090903, lng: 14.400512 },
    PrimitiveDateTime::new(
        // Tuesday February 15, 2022
        Date::try_from_ymd(2022, 2, 15).unwrap(),
        // 6:00:00 pm
        Time::try_from_hms(18, 00, 0).unwrap(),
    ),
).execute().unwrap();

// Dump entire response:

println!("{:#?}", time_zone);

// Parsing example:

use std::time::{SystemTime, UNIX_EPOCH};

let unix_timestamp =
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

println!("Time at your computer: {}", unix_timestamp);

println!("Time in {}: {}",
    time_zone.time_zone_id.unwrap(),
    unix_timestamp as i64 + time_zone.dst_offset.unwrap() as i64 +
        time_zone.raw_offset.unwrap() as i64
);
```

# To do

1. [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
2. Convert explicit query validation to session types wherever reasonable.
3. [Places API](https://developers.google.com/places/web-service/intro)
4. [Roads API](https://developers.google.com/maps/documentation/roads/intro)
5. Retry on Failure
6. Automatic Rate Limiting