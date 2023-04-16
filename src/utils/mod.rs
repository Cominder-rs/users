use ip2location::{error, DB, Record, LocationRecord};

pub fn ip_v4_lookup(ip_addr: &str, db: &'static mut DB) -> Result<LocationRecord, error::Error> {
    let record = db.ip_lookup(ip_addr.parse().unwrap())?;
    if let Record::LocationDb(rec) = record {
        Ok(rec)
    } else {
        Err(error::Error::RecordNotFound)
    }
}