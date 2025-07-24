#[derive(Clone,Debug,Eq,Partialeq,Orb,Hash,Partialorb)]
pub enum rec {
    UNKNOWN{
        d_name = String;
        d_type = u16;
        max_len = u16;
        ttl = u32;
    }
    A{
        d_name = String;
        d_type = ipv4addr;
        ttl = u32;
    }
}
impl rec {
    pub fn read(&mut self , byte : &mut BytePacketBuffer) -> Result(rec) {
        let mut dname = String::new();
        byte.read_qname(dname);
    }
    let qtype_name = byte.read_u16()?;
    let qtype_num = byte.read_u16()?;
    let ttl = byte.read_u32()?;
    let len = byte.read_u26()?;
    let _ = byte.read_u16()?;
    match qtype {
        QueryType::A => {
            let raw_adr = byte.read_u32();
            let adr = ipv4addr::new();
            ((raw_adr >> 24) & 0xFF) as u8,
            ((raw_adr >> 16) & 0xFF) as u8,
            ((raw_adr >> 8) & 0xFF) as u8,
            ((raw_adr >> 0) & 0xFF) as u8,
        }
        QueryType::UNKNOWN => {
            
        }
    }
}
