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
    fn 
}