use std::fmt;

use nom;
use nom::bytes::complete::take;
use nom::number::streaming::le_u32;
use nom::IResult;

pub enum RiffData {
    Data(Vec<u8>),
    Chunks(Vec<RiffChunk>),
}

pub struct RiffChunk {
    pub id: String,
    pub chunk_type: Option<String>,
    pub size: usize,
    pub data: RiffData,
}

impl RiffChunk {
    pub fn parse(i: &[u8]) -> Result<Self, String> {
        let chunk = match Self::parse_riff(i) {
            Ok((_, chunk)) => chunk,
            Err(e) => return Err(e.to_string()),
        };
        Ok(chunk)
    }

    fn fmt_(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let indent_str = " ".repeat(indent);
        write!(
            f,
            "{}id: {}, chunk_type: {:?}, size: {}",
            indent_str, self.id, self.chunk_type, self.size
        )?;

        if let RiffData::Chunks(chunks) = &self.data {
            for chunk in chunks {
                write!(f, "\n")?;
                chunk.fmt_(f, indent + 2)?;
            }
        }
        Ok(())
    }
    fn parse_riff(i: &[u8]) -> IResult<&[u8], RiffChunk> {
        let (i, id) = take(4u8)(i)?;
        let id = match String::from_utf8(id.to_vec()) {
            Ok(id) => id,
            Err(_) => return Err(nom::Err::Error((i, nom::error::ErrorKind::NoneOf))),
        };
        let (i, size) = le_u32(i)?;

        let (i, data) = take(size)(i)?;

        // padding
        let i = if size % 2 != 0 {
            let (i, _) = take(1 as usize)(i)?;
            i
        } else {
            i
        };

        match id.as_str() {
            "RIFF" | "LIST" => {
                let (mut data, chunk_type) = take(4u8)(data)?;
                let chunk_type = match String::from_utf8(chunk_type.to_vec()) {
                    Ok(chunk_type) => chunk_type,
                    Err(_) => return Err(nom::Err::Error((i, nom::error::ErrorKind::NoneOf))),
                };

                let mut chunks = Vec::new();
                while data.to_vec().len() > 0 {
                    let (new_data, chunk) = Self::parse_riff(data)?;
                    data = new_data;
                    chunks.push(chunk);
                }
                Ok((
                    i,
                    RiffChunk {
                        id,
                        chunk_type: Some(chunk_type),
                        size: size as usize,
                        data: RiffData::Chunks(chunks),
                    },
                ))
            }
            _ => Ok((
                i,
                RiffChunk {
                    id,
                    chunk_type: None,
                    size: size as usize,
                    data: RiffData::Data(data.to_vec()),
                },
            )),
        }
    }
}

impl fmt::Display for RiffChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_(f, 0)
    }
}
