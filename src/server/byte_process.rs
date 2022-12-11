pub struct IncomingBytes {
    pub bytes: [u8; 1028],
    packet_size: PacketSize,
    packet: Vec<u8>,
}
impl IncomingBytes {
    pub fn new() -> Self {
        Self {
            bytes: [0; 1028],
            packet_size: PacketSize::new(),
            packet: Vec::new(),
        }
    }

    pub fn process(&mut self, size: usize) -> Vec<Vec<u8>> {
        let mut i: u32 = 0;

        let mut packets = Vec::new();

        println!("bytes recieved: {:?}\n", &self.bytes[0..size]);
        //print bytes as string
        println!(
            "string recieved:\n{:?}\n\n",
            //convert to string lossy
            String::from_utf8_lossy(&self.bytes[0..size])
        );

        while i < size as u32 {
            match self.packet_size.size {
                None => {
                    self.packet_size.find_size(&self.bytes, &mut i);
                }
                Some(packet_size) => {
                    if packet_size - self.packet.len() as i32 <= size as i32 - i as i32 {
                        let p = i + packet_size as u32 - self.packet.len() as u32;
                        self.packet.append(
                            &mut self.bytes[i as usize
                                ..(i + packet_size as u32 - self.packet.len() as u32) as usize]
                                .to_vec(),
                        );
                        i = p;
                        packets.push(self.packet.clone());
                        self.packet = Vec::new();
                        self.packet_size.reset();
                    } else {
                        self.packet
                            .append(&mut self.bytes[i as usize..size].to_vec());
                        i += size as u32;
                    }
                }
            }
        }
        self.bytes = [0; 1028];
        packets
    }
}

struct PacketSize {
    size: Option<i32>,
    buf: [u8; 4],
    iter: u32,
}

impl PacketSize {
    fn new() -> Self {
        Self {
            size: None,
            buf: [0, 0, 0, 0],
            iter: 0,
        }
    }

    fn find_size(&mut self, bytes: &[u8], i: &mut u32) {
        let bytes_len: u32 = bytes.len() as u32;

        while *i < bytes_len && self.iter < 4 {
            self.buf[self.iter as usize] = bytes[*i as usize];
            self.iter += 1;
            *i += 1;
        }
        if self.iter == 4 {
            self.size = Some(i32::from_be_bytes(self.buf));
        }
    }

    fn reset(&mut self) {
        self.size = None;
        self.iter = 0;
    }
}
