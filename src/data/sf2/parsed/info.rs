use std::fmt;

use nom::number::streaming::le_u16;
use nom::IResult;

use super::super::super::riff::{RiffChunk, RiffData};

pub struct SF2Info {
    pub ifil: SFVersion,
    pub isng: String,
    pub inam: String,
    pub irom: Option<String>,
    pub iver: Option<SFVersion>,
    pub icrd: Option<String>,
    pub ieng: Option<String>,
    pub iprd: Option<String>,
    pub icop: Option<String>,
    pub icmt: Option<String>,
    pub isft: Option<String>,
}

#[derive(Clone)]
pub struct SFVersion {
    pub major: u16,
    pub minor: u16,
}

impl fmt::Display for SF2Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "***SF2Info***\n")?;
        write!(f, "ifil: {}\n", self.ifil)?;
        write!(f, "isng: {}\n", self.isng)?;
        write!(f, "INAM: {}\n", self.inam)?;
        if let Some(irom) = &self.irom {
            write!(f, "irom: {}\n", irom)?;
        }
        if let Some(iver) = &self.iver {
            write!(f, "iver: {}\n", iver)?;
        }
        if let Some(icrd) = &self.icrd {
            write!(f, "ICRD: {}\n", icrd)?;
        }
        if let Some(ieng) = &self.ieng {
            write!(f, "IENG: {}\n", ieng)?;
        }
        if let Some(iprd) = &self.iprd {
            write!(f, "IPRD: {}\n", iprd)?;
        }
        if let Some(icop) = &self.icop {
            write!(f, "ICOP: {}\n", icop)?;
        }
        if let Some(icmt) = &self.icmt {
            write!(f, "ICMT: {}\n", icmt)?;
        }
        if let Some(isft) = &self.isft {
            write!(f, "ISFT: {}\n", isft)?;
        }

        Ok(())
    }
}

impl fmt::Display for SFVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

fn parse_sfversion(i: &[u8]) -> IResult<&[u8], SFVersion> {
    let (i, major) = le_u16(i)?;
    let (i, minor) = le_u16(i)?;
    Ok((i, SFVersion { major, minor }))
}

pub fn convert_chunk_to_sf2info(chunk: &RiffChunk) -> Result<SF2Info, String> {
    let mut ifil: Option<SFVersion> = None;
    let mut isng: Option<String> = None;
    let mut inam: Option<String> = None;
    let mut irom: Option<String> = None;
    let mut iver: Option<SFVersion> = None;
    let mut icrd: Option<String> = None;
    let mut ieng: Option<String> = None;
    let mut iprd: Option<String> = None;
    let mut icop: Option<String> = None;
    let mut icmt: Option<String> = None;
    let mut isft: Option<String> = None;

    if let Some(chunk_type) = &chunk.chunk_type {
        if chunk_type == "INFO" && chunk.id == "LIST" {
            if let RiffData::Chunks(subchunks) = &chunk.data {
                for subchunk in subchunks {
                    if let RiffData::Data(data_in_subchunk) = &subchunk.data {
                        match subchunk.id.as_str() {
                            "ifil" => {
                                let i = data_in_subchunk;
                                let (_i, ifil_) = parse_sfversion(i).expect("Invalid ifil");
                                ifil = Some(ifil_);
                            }
                            "isng" => {
                                isng = Some(
                                    String::from_utf8(data_in_subchunk.to_vec())
                                        .expect("Invalid isng"),
                                );
                            }
                            "INAM" => {
                                inam = Some(
                                    String::from_utf8(data_in_subchunk.to_vec())
                                        .expect("Invalid INAM"),
                                );
                            }
                            "irom" => {
                                irom = Some(
                                    String::from_utf8(data_in_subchunk.to_vec())
                                        .expect("Invalid irom"),
                                );
                            }
                            "iver" => {
                                let i = data_in_subchunk;
                                let (_i, iver_) = parse_sfversion(i).expect("Invalid iver");
                                iver = Some(iver_);
                            }
                            "ICRD" => {
                                icrd = Some(
                                    String::from_utf8(data_in_subchunk.to_vec())
                                        .expect("Invalid ICRD"),
                                );
                            }
                            "IENG" => {
                                ieng = Some(
                                    String::from_utf8(data_in_subchunk.to_vec())
                                        .expect("Invalid IENG"),
                                );
                            }
                            "IPRD" => {
                                iprd = Some(
                                    String::from_utf8(data_in_subchunk.to_vec())
                                        .expect("Invalid IPRD"),
                                );
                            }
                            "ICOP" => {
                                icop = Some(
                                    String::from_utf8(data_in_subchunk.to_vec())
                                        .expect("Invalid ICOP"),
                                );
                            }
                            "ICMT" => {
                                icmt = Some(
                                    String::from_utf8(data_in_subchunk.to_vec())
                                        .expect("Invalid ICMT"),
                                );
                            }
                            "ISFT" => {
                                isft = Some(
                                    String::from_utf8(data_in_subchunk.to_vec())
                                        .expect("Invalid ISFT"),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(SF2Info {
        ifil: ifil.expect("Failed to parse ifil_major"),
        isng: isng.expect("Failed to parse isng"),
        inam: inam.expect("Failed to parse inam"),
        irom,
        iver,
        icrd,
        ieng,
        iprd,
        icop,
        icmt,
        isft,
    })
}
