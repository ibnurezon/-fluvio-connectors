#![allow(clippy::unnecessary_mut_passed)]
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();
    let mut value = Vec::from(record.value.as_ref());

    value.make_ascii_uppercase();
    Ok((key, value.into()))
}
