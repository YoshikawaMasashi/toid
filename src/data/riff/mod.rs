use std::fmt;

use nom;
use nom::bytes::complete::take;
use nom::number::streaming::le_u32;
use nom::IResult;

pub enum RiffData {
    Data(Vec<u8>),
    Chanks(Vec<RiffChank>),
}

pub struct RiffChank {
    pub id: String,
    pub chank_type: Option<String>,
    pub size: usize,
    pub data: RiffData,
}

impl RiffChank {
    pub fn parse(i: &[u8]) -> Result<Self, String> {
        let chank = match Self::parse_riff(i) {
            Ok((_, chank)) => chank,
            Err(e) => return Err(e.to_string()),
        };
        Ok(chank)
    }

    fn fmt_(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let indent_str = " ".repeat(indent);
        write!(
            f,
            "{}id: {}, chank_type: {:?}, size: {}",
            indent_str, self.id, self.chank_type, self.size
        )?;

        if let RiffData::Chanks(chanks) = &self.data {
            for chank in chanks {
                write!(f, "\n")?;
                chank.fmt_(f, indent + 2)?;
            }
        }
        Ok(())
    }
    fn parse_riff(i: &[u8]) -> IResult<&[u8], RiffChank> {
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
                let (mut data, chank_type) = take(4u8)(data)?;
                let chank_type = match String::from_utf8(chank_type.to_vec()) {
                    Ok(chank_type) => chank_type,
                    Err(_) => return Err(nom::Err::Error((i, nom::error::ErrorKind::NoneOf))),
                };

                let mut chanks = Vec::new();
                while data.to_vec().len() > 0 {
                    let (new_data, chank) = Self::parse_riff(data)?;
                    data = new_data;
                    chanks.push(chank);
                }
                Ok((
                    i,
                    RiffChank {
                        id,
                        chank_type: Some(chank_type),
                        size: size as usize,
                        data: RiffData::Chanks(chanks),
                    },
                ))
            }
            _ => Ok((
                i,
                RiffChank {
                    id,
                    chank_type: None,
                    size: size as usize,
                    data: RiffData::Data(data.to_vec()),
                },
            )),
        }
    }
}

impl fmt::Display for RiffChank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_(f, 0)
    }
}
