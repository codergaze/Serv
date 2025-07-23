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
    pub struct {
        
    }
}