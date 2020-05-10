use super::super::super::riff::{RiffChank, RiffData};

pub struct DataChunk {
    pub data: Vec<u8>,
}

impl std::fmt::Display for DataChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "***DataChunk***\n")?;

        Ok(())
    }
}

pub fn convert_chank_to_data_chunk(chank: &RiffChank) -> Result<DataChunk, String> {
    if chank.chank_type == None {
        if let RiffData::Data(data) = &chank.data {
            return Ok(DataChunk {
                data: data.to_vec(),
            });
        }
    }

    Err("error".to_string())
}
