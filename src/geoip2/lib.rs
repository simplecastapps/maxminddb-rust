#![crate_name = "geoip2"]

#![comment = "MaxMind GeoIP2"]
#![license = "Apache 2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

extern crate collections;
extern crate maxminddb;
extern crate serialize;

#[deriving(Decodable, Show)]
pub struct Names {
    en: Option<String>,
}

#[deriving(Decodable, Show)]
pub struct Continent {
    code: Option<String>,
    geoname_id: Option<uint>,
    names: Option<Names>,
}

#[deriving(Decodable, Show)]
pub struct Place {
    geoname_id: Option<uint>,
    iso_code: Option<String>,
    names: Option<Names>,
}

#[deriving(Decodable, Show)]
pub struct Traits {
    is_anonymous_proxy: Option<bool>,
    is_satellite_provider: Option<bool>,
}

#[deriving(Decodable, Show)]
pub struct Country {
    continent: Option<Continent>,
    country: Option<Place>,
    registered_country: Option<Place>,
    represented_country: Option<Place>,
    traits: Option<Traits>,
}
