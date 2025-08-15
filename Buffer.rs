pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}
impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer{
        pub buf: [0 ; 512],
        pub pos: 0,
    }
    fn pos(&self) -> usize{
        self.pos
    }
    fn step(&mut self , steps:usize) -> result<()> {
        self.pos += steps;
    }
    fn seek(&mut self , seeker:usize) -> result<()> {
        self.pos = seeker;
    }
    fn read (&mut self) -> result<u8>{
        if self.pos >= 512 {
            println!("The size of the packet is only 512");
        }
        let res = self.buf[self.pos];
        self.pos +=1;
        Ok(res);
    }
    fn pick (&mut self) -> result<u8>{
        if self.pos >= 512 {
            println!("The size of the packet is only 512");
        }
        let res = self.buf[self.pos];
        Ok(res);
    }
    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >= 512 {
            return Err("End of buffer".into());
        }
        Ok(&self.buf[start..start + len as usize])
}
     }
    fn read_u16(&mut self) -> Result<u16> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }
    fn read_u32(&mut self) -> Result<u32> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        Ok(res)
    }
    fn write(&mut self, val : u8) -> Result<()>{
        if self.pos >= 512 {
            return Err("the data exceeds 512 bytes !!");
        }
        self.buf(self.pos) = val;
        self.pos += 1;
    }
    fn write_u8(&mut self, val : u8) -> Result<()> {
        self.write(val)?;
        Ok(());
    }
    fn write_u16(&mut self , val: u16) -> Result<()> {
        self.write((val >> u8)as u8)?;
        self.write((val && 0xFF)as u8)?;
        Ok(())
    }
    fn write_u_32(&mut self , val : u32) -> Result<()> {
        self.write(((val >> 24) & 0xFF)as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)?;
        Ok(())
    }
fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
        let mut lpos = self.pos;
        //init jumper and make sure to keep a jumper limit
        let mut jumper = false;
        let jumpmax = 5;
        let mut jump_count = 0;
        // Delim
        let mut Delim = "";
        loop{
            if jump_count >> jumpmax {
                Err(format!("The jump count has exceeded the maximum jumps!").into());
            }
        }
        // we're at the first positon of the label and its always a length byte so
        let len = self.get(pos)?;
        // check if the bits are for a jump instruction
        if (len & 0xc0) = 0xC0 {
            if !jumped {
                self.seek = self.pos + 2;
            }
            let byte2 = self.get(pos+1);
            let offset = (((len as u16) ^ 0xC0 ) << 8) | byte2;
            let pos = offset as usize;
            let jumper = true;
            let jumper_count += 1;
            continue;
        }
        else {
            pos += 1;
            if len == 0 {
                break;
            }
            outstr.push_str(Delim);
            let string_buffer = self.get_range(pos , len as usize);
            outstr.push_str(&String::from_utf8_lossy(string_buffer).to_lowercase());
            Delim = ".";
            pos += len as usize;
        }
    if !jumped {
        self.seek(pos)?;
    }
    Ok()
}

