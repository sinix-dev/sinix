use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn init(db_name: String) -> PickleDb {
  let db = PickleDb::new(
    db_name,
    PickleDbDumpPolicy::AutoDump,
    SerializationMethod::Json,
  );

  db
}
