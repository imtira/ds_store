#[derive(Debug)]
pub struct Record {
    pub file_name: String,
    pub structure_type: String,
    pub structure_id: usize,
}

impl Record {
    pub fn new(file_name: String, structure_type: String, structure_id: usize) -> Self {
        Record {
            file_name,
            structure_type,
            structure_id,
        }
    }
}
