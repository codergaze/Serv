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
