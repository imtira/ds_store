// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.
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
