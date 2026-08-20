pub const MAX_DATAGRAM_SIZE: usize = 64 * 1024;

#[repr(u8)]
#[derive(Debug)]
pub enum PacketKind {
    SpawnEntity,
}

impl TryFrom<u8> for PacketKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketKind::SpawnEntity),
            _ => Err(()),
        }
    }
}

pub struct Packet {
    len: u16, // just to doublecheck
    kind: PacketKind,
    payload: String,
}

impl TryFrom<&[u8]> for Packet {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let kind = PacketKind::try_from(u8::from_le_bytes([value[2]]));
        if let Ok(kind) = kind {
            Ok(Self {
                len: u16::from_le_bytes([value[0], value[1]]),
                kind,
                payload: String::from_utf8_lossy(&value[3..]).to_string(),
            })
        } else {
            Err(())
        }
    }
}
