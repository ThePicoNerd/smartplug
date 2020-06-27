use super::Smartplug;
use std::io::prelude::*;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::TcpStream;

pub struct Kasa {
    ip: IpAddr,
    port: u16,
}

impl Kasa {
    pub fn new(ip: IpAddr) -> Kasa {
        Kasa { ip, port: 9999 }
    }

    /// Encode a JSON command because TP-Link doesn't know better.
    fn encode_command(command: String) -> Vec<u8> {
        let mut key: u8 = 171;
        let mut result: Vec<u8> = (command.len() as u32).to_be_bytes().to_vec();

        for c in command.chars() {
            let a = key ^ (c as u8);
            key = a;
            result.push(a);
        }

        result
    }
}

impl Smartplug for Kasa {
    fn set_power(&self, power: bool) -> Result<(), std::io::Error> {
        let command = Kasa::encode_command(format!(
            "{{\"system\":{{\"set_relay_state\":{{\"state\":{}}}}}}}",
            power as u8
        ));

        let mut stream = TcpStream::connect(SocketAddr::new(self.ip, self.port))?;

        stream.write_all(command.as_slice())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_command() {
        assert_eq!(
            Kasa::encode_command("bruh".to_string()),
            [0x0, 0x0, 0x0, 0x4, 0xc9, 0xbb, 0xce, 0xa6]
        );
    }
}
