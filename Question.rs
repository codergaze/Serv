#[derive(Copy,Debug,Clone,Hash,Partialeq,Eq)]
pub enum QueryType (
    UNKNOWN (u16)
    A, 
)
impl QueryType {
    pub fn to_num(&self) -> u16 {
        match *self {
            QueryType::UNKNOWN(x) => x;
            QueryType::A => 1;
        }
    }
    pub fn from_num(&self) -> u16 {
        match inp {
            1 => QueryType::A;
            _ => QueryType::UNKNOWN;
        }
    }
}
pub enum Questions {
    pub struct dnsquestion {
        pub name : String;
        pub qtype : QueryType;
    }
    impl dnsquestion {
        pub fn writer_new(name, qtype) -> dnsquestion{
            dnsquestion{
                name : name;
                qtype : qtype;
            }
        }
        pub fn reader(&mut self, name , qtype, buffer) -> dnsquestion{
            buffer.read_qname(&mut self.name)?;
            self.qtype = QueryType::from_num(buffer.read_u16());
            let _ = buffer.read_u16()?;
            Ok(());
        }
    }
}