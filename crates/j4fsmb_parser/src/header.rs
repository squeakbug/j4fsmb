use deku::prelude::*;

pub const SMB2_COMMAND_NEGOTIATE_PROTOCOL:      u16 = 0x0000;
pub const SMB2_COMMAND_SESSION_SETUP:           u16 = 0x0001;
pub const SMB2_COMMAND_SESSION_LOGOFF:          u16 = 0x0002;
pub const SMB2_COMMAND_TREE_CONNECT:            u16 = 0x0003;
pub const SMB2_COMMAND_TREE_DISCONNECT:         u16 = 0x0004;
pub const SMB2_COMMAND_CREATE:                  u16 = 0x0005;
pub const SMB2_COMMAND_CLOSE:                   u16 = 0x0006;
pub const SMB2_COMMAND_FLUSH:                   u16 = 0x0007;
pub const SMB2_COMMAND_READ:                    u16 = 0x0008;
pub const SMB2_COMMAND_WRITE:                   u16 = 0x0009;
pub const SMB2_COMMAND_LOCK:                    u16 = 0x000a;
pub const SMB2_COMMAND_IOCTL:                   u16 = 0x000b;
pub const SMB2_COMMAND_CANCEL:                  u16 = 0x000c;
pub const SMB2_COMMAND_ECHO:                    u16 = 0x000d;
pub const SMB2_COMMAND_QUERY_DIRECTORY:         u16 = 0x000e;
pub const SMB2_COMMAND_CHANGE_NOTIFY:           u16 = 0x000f;
pub const SMB2_COMMAND_QUERY_INFO:              u16 = 0x0010;
pub const SMB2_COMMAND_SET_INFO:                u16 = 0x0011;
pub const SMB2_COMMAND_OPLOCK_BREAK:            u16 = 0x0012;
pub const SMB2_COMMAND_SERVER_TO_CLIENT_NOTIFY: u16 = 0x0013;

pub enum Command {
    Negotiate,
    SessionSetup,
    SessionLogoff,
    TreeConnect,
    TreeDisconnect,
    Create,
    Close,
    Flush,
    Read,
    Write,
    Lock,
    Ioctl,
    Cancel,
    Echo,
    QueryDirectory,
    ChangeNotify,
    QueryInfo,
    SetInfo,
    OplockBreak,
    ServerToClientNotify,
}

pub enum Flags {
    ServerToRedir,
    AsyncCommand,
    RelatedOperations,
    Signed,
    PriorityMask,
    DfsOperations,
    ReplayOperation,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct Smb2SyncHeader {
    pub protocol_id: u32,
    pub structure_size: u16,
    pub credit_charge: u16,
    pub status: u32,
    pub command: u16,
    pub credit: u16,
    pub flags: u32,
    pub next_command: u16,
    pub message_id: u64,
    pub reserved: u16,
    pub tree_id: u32,
    pub session_id: u64,
    pub signature: u128,
}

impl Default for Smb2SyncHeader {
    fn default() -> Self {
        Smb2SyncHeader::new()
    }
}

impl Smb2SyncHeader {
    pub fn new() -> Self {
        Smb2SyncHeader {
            protocol_id: 0,
            structure_size: 36,
            credit_charge: 0,
            status: 0,
            command: 0,
            credit: 0,
            flags: 0,
            next_command: 0,
            message_id: 0,
            reserved: 0,
            tree_id: 0,
            session_id: 0,
            signature: 0,
        }
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct Smb2AsyncHeader {
    pub protocol_id: u32,
    pub structure_size: u16,
    pub credit_charge: u16,
    pub status: u32,
    pub command: u16,
    pub credit: u16,
    pub flags: u32,
    pub next_command: u16,
    pub message_id: u64,
    pub async_id: u64,
    pub session_id: u64,
    pub signature: u128,
}
