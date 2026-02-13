use uuid::Uuid;

pub fn uuid_to_str(uuid: Uuid) -> String {
    uuid.to_string()
}

pub fn str_to_uuid(string: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(string)
}

pub fn uuid_generate() -> String {
    Uuid::new_v4().to_string()
}
