pub const MAX_DATAGRAM_SIZE: usize = 64 * 1024;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PacketKind {
    SpawnEntity,
}

impl From<PacketKind> for u8 {
    fn from(value: PacketKind) -> Self {
        value as u8
    }
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

impl Packet {
    pub fn new(kind: PacketKind, payload: String) -> Self {
        Self {
            len: 3 + payload.len() as u16,
            kind,
            payload,
        }
    }
}

impl From<&Packet> for Vec<u8> {
    fn from(value: &Packet) -> Self {
        let payload = value.payload.as_bytes();

        let mut bytes = Vec::with_capacity(3 + payload.len());

        bytes.extend_from_slice(&value.len.to_le_bytes());
        bytes.push(value.kind.into());
        bytes.extend_from_slice(payload);

        bytes
    }
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
