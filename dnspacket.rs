#[derive(Clone,Debug)]
pub struct Dnspacket{
    pub header: DnsHeader;
    pub questions: vec<Dnspacket>,
    pub answers: Vec<rec>,
    pub authorities: Vec<rec>,
    pub resources : Vec<rec>,
}
impl Dnspacket{
    pub fn new() -> Dnspacket {
        Dnspacket {
            header: DnsHeader::new;
            questions: vec::new;
            answers:vec::new;
            authorities:vec::new;
            resources:vec::new;
        }
    }
    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket> {
        let mut result = DnsPacket::new();
        result.header.read(buffer)?;

        for _ in 0..result.header.questions {
            let mut question = DnsQuestion::new("".to_string(), QueryType::UNKNOWN(0));
            question.read(buffer)?;
            result.questions.push(question);
        }

        for _ in 0..result.header.answers {
            let rec = DnsRecord::read(buffer)?;
            result.answers.push(rec);
        }
        for _ in 0..result.header.authoritative_entries {
            let rec = DnsRecord::read(buffer)?;
            result.authorities.push(rec);
        }
        for _ in 0..result.header.resource_entries {
            let rec = DnsRecord::read(buffer)?;
            result.resources.push(rec);
        }

        Ok(result)
    }
}
