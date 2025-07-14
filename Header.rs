#[derive(Clone, Debug)]
pub struck Dnsheader {
    pub id : u16;
    pub qr : bool;
    pub opcode : bool;
    pub AA : bool;
    pub TC :bool;
    pub RD : bool;
    pub RA : bool;
    pub rcode : u8;
    pub qcount : u16;
    pub acount : u16;
    pub au_count : u16;
    pub r_entry : u16;
}
impl Dnsheader {
    pub fn new() -> Dnsheader{
        Dnsheader{
            id : 0;
            qr : false;
            opcode : false;
            AA : false;
            TC : false;
            RD : false;
            RA : false;
            rcode : 0;
            qucount : 0;
            acount : 0;
            au_count : 0;
            r_entry : 0;
        }
    }
    pub fn reader (&mut self , buffer: &mut BytePacketBuffer) -> Result<()>{
        self.id = buffer.read_u16()?;
        let flag = buffer.read_u16()?;
        let a = (flag >> 8) as u8;
        let b = (flags & 0xFF) as u8;
        self.

}
