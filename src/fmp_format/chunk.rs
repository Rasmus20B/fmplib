use crate::util::format_decode::{get_int, get_path_int};

#[derive(Debug, Clone, PartialEq)]
pub enum ChunkType {
    DataSimple = 0,
    RefSimple = 1,
    RefLong = 2,
    DataSegment = 3,
    PathPush = 4,
    PathPop = 5,
    Noop = 6,
}

#[derive(Clone)]
pub struct Chunk<'a> {
    pub ctype: ChunkType,
    pub code: u16,
    pub data: Option<&'a [u8]>,
    pub ref_data: Option<&'a [u8]>,
    pub path: Vec::<String>,
    pub segment_idx: Option<u8>,
    pub ref_simple: Option<u16>,
}

impl<'a> Chunk<'a> {
    pub fn new(ctype: ChunkType,
           code: u16,
           data: Option<&'a [u8]>,
           ref_data: Option<&'a [u8]>,
           path: Vec::<String>,
           segment_idx: Option<u8>,
           ref_simple: Option<u16>,
        ) -> Self {
        Self {
            ctype,
            code,
            data,
            ref_data,
            path,
            segment_idx,
            ref_simple,
        }
    }
}

pub fn get_chunk_from_code<'a>(code: &'a[u8], offset: &mut usize, path: &mut Vec<String>, local : usize) -> Result<Chunk<'a>, &'static str> {
    let mut chunk_code = code[*offset];
    let mut ctype = ChunkType::Noop;
    let mut data: Option<&[u8]> = None;
    let mut ref_data: Option<&[u8]> = None;
    let mut segidx: Option<u8> = None;
    let mut ref_simple: Option<u16> = None;
    let mut delayed = false;
    
    let table_idx = 0;

    if (chunk_code & 0xC0) == 0xC0 {
        chunk_code &= 0x3F;
        delayed = true;
    }

    // println!("offset: {}, Code: {:x}", offset, chunk_code);

    match chunk_code {
        0x00 => {
            *offset += 1;
            ctype = ChunkType::DataSimple;
            data = Some(&code[*offset..*offset]);
            if code[1] == 0x00 { }
            *offset += 1;
        },
        0x01 | 0x02 | 0x03 | 0x04 | 0x05 => {
            *offset += 1;
            ctype = ChunkType::RefSimple;
            ref_simple = Some(code[*offset] as u16);
            *offset += 1;
            let len = (chunk_code == 0x01) as usize + (2 * (chunk_code - 0x01) as usize);
            data = Some(&code[*offset..*offset+len]);
            *offset += len;
        }
        0x06 => {
            *offset += 1;
            ctype = ChunkType::RefSimple;
            ref_simple = Some(code[*offset] as u16);
            *offset += 1;
            let len = code[*offset] as usize;
            *offset += 1;
            data = Some(&code[*offset..*offset+len]);
            *offset += data.unwrap().len();
        },
        0x07 => {
            *offset += 1;
            ctype = ChunkType::DataSegment;
            segidx = Some(code[*offset]);
            *offset += 1;
            let len = get_int(&code[*offset..*offset+2]);
            *offset += 2;
            data = Some(&code[*offset..*offset+len]);
            *offset += len;
        },
        0x08 => {
            *offset += 1;
            ctype = ChunkType::DataSimple;
            data = Some(&code[*offset..*offset+2]);
            *offset += 2;
        },
        0x09 | 0x0A | 0x0B | 0x0C | 0x0D => {
            *offset += 1;
            ctype = ChunkType::RefSimple;
            ref_simple = Some(get_path_int(&code[*offset..*offset+2]) as u16);
            *offset += 2;
            let len = (chunk_code == 0x09) as usize + (2 *(chunk_code - 0x09) as usize);
            data = Some(&code[*offset..*offset+len]);
            *offset += len;
        },
        0x0E => {
            if code[*offset + 1] != 0xFF {
                *offset += 1;
                ctype = ChunkType::RefSimple;
                ref_simple = Some(get_path_int(&code[*offset..*offset+2]) as u16);
                *offset += 2;
                let len = code[*offset] as usize;
                *offset += 1;
                data = Some(&code[*offset..*offset+len]);
                *offset += len;
            } else {
                *offset += 1;
                ctype = ChunkType::DataSimple;
                data = Some(&code[*offset..*offset+6]);
                *offset += 6;
            }
        },
        0x0F => {
            if code[*offset+1] == 0x80 {
                ctype = ChunkType::DataSegment;
                *offset += 2;
                segidx = Some(code[*offset]);
                *offset += 1;
                let len = get_int(&code[*offset..*offset+2]);
                *offset += 2;
                data = Some(&code[*offset..*offset+len]);
                *offset += len;
            }
        },
        0x10 => {
            *offset += 1;
            ctype = ChunkType::DataSimple;
            data = Some(&code[*offset..*offset+3]);
            *offset += 3;
        },
        0x11 | 0x12 | 0x13 | 0x14 | 0x15 => {
            *offset += 1;
            ctype = ChunkType::DataSimple;
            let len = 3 + (chunk_code == 0x11) as usize + (2 * (chunk_code as usize - 0x11));
            data = Some(&code[*offset..*offset+len]);
            *offset += len;
        },
        0x16 => {
            *offset += 1;
            ctype = ChunkType::RefLong;
            ref_data = Some(&code[*offset..*offset+3]);
            *offset += 3;
            let len = code[*offset] as usize;
            *offset += 1;
            data = Some(&code[*offset..*offset+len]);
            *offset += len;
        }
        0x17 => {
            *offset += 1;
            ctype = ChunkType::RefLong;
            ref_data = Some(&code[*offset..*offset+3]);
            *offset += 3;
            let len = get_path_int(&code[*offset..*offset+2]);
            *offset += 2;
            data = Some(&code[*offset..*offset+len]);
            *offset += len;
        },
        0x1B => {
            if code[*offset + 1] == 0x00 {
                *offset += 2;
                ctype = ChunkType::RefSimple;
                ref_simple = Some(code[*offset] as u16);
                *offset += 1;
                data = Some(&code[*offset..*offset+4]);
                *offset += 4;
            } else {
                *offset += 1;
                ctype = ChunkType::DataSimple;
                let len = code[*offset] as usize;
                *offset += 1;
                data = Some(&code[*offset..*offset+len]);
                *offset += len + (chunk_code == 0x19) as usize + (2 * (chunk_code as usize - 0x19));
            }
        },
        0x19 | 0x1A | 0x1C | 0x1D => {
            *offset += 1;
            ctype = ChunkType::DataSimple;
            let len = code[*offset] as usize;
            *offset += 1;
            data = Some(&code[*offset..*offset+len]);
            *offset += len + (chunk_code == 0x19) as usize + (2 * (chunk_code as usize - 0x19));
        },
        0x1E => {
            *offset += 1;
            ctype = ChunkType::RefLong;
            let ref_len = code[*offset] as usize;
            *offset += 1;
            ref_data = Some(&code[*offset..*offset+ref_len]);
            *offset += ref_len;
            let len = code[*offset] as usize;
            *offset += 1;
            data = Some(&code[*offset..*offset+len]);
            *offset += len;
        },
        0x1F => {
            *offset += 1;
            ctype = ChunkType::RefLong;
            let ref_len = code[*offset] as usize;
            *offset += 1;
            ref_data = Some(&code[*offset..*offset+ref_len]);
            let len = get_int(&code[*offset..*offset+2]);
            *offset += 2;
            data = Some(&code[*offset..*offset+len]);
            *offset += len;
        },
        0x20 => {
            *offset += 1;
            ctype = ChunkType::PathPush;
            if code[*offset] == 0xFE {
                *offset += 1;
                data = Some(&code[*offset..*offset+8]);
            } else {
                data = Some(&code[*offset..*offset+1]);
            }
            let idx = get_path_int(&code[*offset..*offset+1]);
            *offset += data.unwrap().len();
            path.push(idx.to_string());
        },
        0x23 => {
            *offset += 1;
            ctype = ChunkType::DataSimple;
            let len = code[*offset] as usize;
            *offset += 1;
            data = Some(&code[*offset..*offset+len]);
            *offset += len;
        },
        0x28 => {
            *offset += 1;
            ctype = ChunkType::PathPush;
            data = Some(&code[*offset..*offset+2]);
            let idx = get_path_int(&code[*offset..*offset+2]);
            *offset += 2;
            path.push(idx.to_string());
        },
        0x30 => {
            *offset += 1;
            ctype = ChunkType::PathPush;
            data = Some(&code[*offset..*offset+3]);
            // let dir = 0x80 + ((code[*offset + 1] as usize) << 8) + code[*offset + 2] as usize;
            let dir = get_path_int(&code[*offset..*offset+3]).to_string();
            path.push(dir.to_string());
            *offset += 3;
        },
        0x38 => {
            *offset += 1;
            ctype = ChunkType::PathPush;
            let len = code[*offset] as usize;
            *offset += 1;
            data = Some(&code[*offset..*offset+2]);
            path.push(get_path_int(&code[*offset..*offset+2]).to_string());
            *offset += len;
        },
        0x3D | 0x40 => {
            ctype = ChunkType::PathPop;
            *offset += 1;
            path.pop();
        },
        0x80 => {
            ctype = ChunkType::Noop;
            *offset += 1;
        }
        _ => {
            eprintln!("Unknown code @ B:{}, O:{}: {:x}", (4096 / (*offset - local)), local, chunk_code);
            return Err("Invalid Opcode for Chunk.");
        }
    };

    if delayed == true {
        path.pop();
    }
    return Ok(Chunk::new(ctype,
                      chunk_code.into(),
                      data,
                      ref_data,
                      path.clone(),
                      segidx,
                      ref_simple));
}



