use nom::bytes::complete::take;
use nom::number::streaming::le_u32;
use nom::IResult;

pub enum RiffData {
    Data(Vec<u8>),
    Chanks(Vec<RiffChank>),
}

pub struct RiffChank {
    id: String,
    chank_type: Option<String>,
    size: u32,
    data: RiffData,
}

fn parse_riff(i: &[u8]) -> IResult<&[u8], RiffChank> {
    let (i, id) = take(4u8)(i)?;
    let id = String::from_utf8(id.to_vec()).unwrap();
    let (i, size) = le_u32(i)?;

    let (i, data) = take(size)(i)?;

    match id.as_str() {
        "RIFF" | "LIST" => {
            let (mut data, chank_type) = take(4u8)(data)?;
            let chank_type = String::from_utf8(chank_type.to_vec()).unwrap();

            let mut chanks = Vec::new();
            while data.to_vec().len() > 0 {
                let (new_data, chank) = parse_riff(data)?;
                data = new_data;
                chanks.push(chank);
            }
            Ok((
                i,
                RiffChank {
                    id,
                    chank_type: Some(chank_type),
                    size,
                    data: RiffData::Chanks(chanks),
                },
            ))
        }
        _ => Ok((
            i,
            RiffChank {
                id,
                chank_type: None,
                size,
                data: RiffData::Data(data.to_vec()),
            },
        )),
    }
}

impl RiffChank {
    pub fn parse(i: &[u8]) -> Self {
        let (_, chank) = parse_riff(i).expect("Failed to parse RIFF");
        chank
    }

    pub fn print(&self) {
        self.print_(0);
    }

    fn print_(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!(
            "{}id: {}, chank_type: {:?}, size: {}",
            indent_str, self.id, self.chank_type, self.size
        );

        if let RiffData::Chanks(chanks) = &self.data {
            for chank in chanks {
                chank.print_(indent + 2);
            }
        }
    }
}
