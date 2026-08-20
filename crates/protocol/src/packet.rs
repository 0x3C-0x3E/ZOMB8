use serde::Serialize;

pub const MAX_DATAGRAM_SIZE: usize = 64 * 256;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PacketKind {
    SpawnEntity,
    Ping,
    SetPlayerId,
}

impl TryFrom<u8> for PacketKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketKind::SpawnEntity),
            1 => Ok(PacketKind::Ping),
            2 => Ok(PacketKind::SetPlayerId),
            _ => Err(()),
        }
    }
}

impl From<PacketKind> for u8 {
    fn from(value: PacketKind) -> Self {
        value as u8
    }
}

pub trait PacketPayload: Serialize {
    const KIND: PacketKind;
}

#[derive(Debug)]
pub struct Packet {
    pub len: u16,
    pub kind: PacketKind,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn from_payload<T: PacketPayload>(payload: T) -> Result<Self, bincode::Error> {
        let payload = bincode::serialize(&payload)?;
        Ok(Self {
            len: 3 + payload.len() as u16,
            kind: T::KIND,
            payload,
        })
    }
}

impl From<&Packet> for Vec<u8> {
    fn from(value: &Packet) -> Self {
        let payload = value.payload.iter().as_slice();

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
            let len = u16::from_le_bytes([value[0], value[1]]);

            assert_eq!(
                len as usize,
                value.len(),
                "slice len did not match reported len"
            );

            Ok(Self {
                len,
                kind,
                payload: Vec::from(&value[3..value.len()]),
            })
        } else {
            Err(())
        }
    }
}
