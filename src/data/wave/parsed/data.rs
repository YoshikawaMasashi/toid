use super::super::super::riff::{RiffChunk, RiffData};

pub struct DataChunk {
    pub data: Vec<u8>,
}

impl std::fmt::Display for DataChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "***DataChunk***\n")?;

        Ok(())
    }
}

pub fn convert_chunk_to_data_chunk(chunk: &RiffChunk) -> Result<DataChunk, String> {
    if chunk.chunk_type == None {
        if let RiffData::Data(data) = &chunk.data {
            return Ok(DataChunk {
                data: data.to_vec(),
            });
        }
    }

    Err("error".to_string())
}
