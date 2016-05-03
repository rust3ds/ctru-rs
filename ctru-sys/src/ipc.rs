//TODO: Implement static inline functions + solve the anonymous enum enigma

#[derive(Clone, Copy)]
#[repr(C)]
pub enum IPC_BufferRights {
    IPC_BUFFER_R = 2,
    IPC_BUFFER_W = 4,
    IPC_BUFFER_RW = 6,
}

