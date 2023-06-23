use std::net::IpAddr;

use ip2location::{error, Record};


use users_proto::CountryCode;
use crate::middlewares::IpDB;

pub fn find_country(ip_addr: IpAddr, db: IpDB) -> Result<CountryCode, error::Error> {
    let mut db = db.lock();
    let record = db.ip_lookup(ip_addr)?;
    if let Record::LocationDb(rec) = record {
        if let Some(country) = rec.country {
            Ok(country.short_name.into())
        } else {
            Err(error::Error::RecordNotFound)
        }
    } else {
        Err(error::Error::RecordNotFound)
    }
}


