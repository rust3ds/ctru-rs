use ::Result;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum PS_AESAlgorithm {
    PS_ALGORITHM_CBC_ENC = 0,
    PS_ALGORITHM_CBC_DEC = 1,
    PS_ALGORITHM_CTR_ENC = 2,
    PS_ALGORITHM_CTR_DEC = 3,
    PS_ALGORITHM_CCM_ENC = 4,
    PS_ALGORITHM_CCM_DEC = 5,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum PS_AESKeyType {
    PS_KEYSLOT_0D = 0,
    PS_KEYSLOT_2D = 1,
    PS_KEYSLOT_31 = 2,
    PS_KEYSLOT_38 = 3,
    PS_KEYSLOT_32 = 4,
    PS_KEYSLOT_39_DLP = 5,
    PS_KEYSLOT_2E = 6,
    PS_KEYSLOT_INVALID = 7,
    PS_KEYSLOT_36 = 8,
    PS_KEYSLOT_39_NFC = 9,
}

extern "C" {
    pub fn psInit() -> Result;
    pub fn psExit();
    pub fn PS_EncryptDecryptAes(size: u32, _in: *mut u8, out: *mut u8,
                                aes_algo: PS_AESAlgorithm,
                                key_type: PS_AESKeyType, iv: *mut u8)
     -> Result;
    pub fn PS_EncryptSignDecryptVerifyAesCcm(_in: *mut u8, in_size: u32,
                                             out: *mut u8, out_size: u32,
                                             data_len: u32,
                                             mac_data_len: u32,
                                             mac_len: u32,
                                             aes_algo: PS_AESAlgorithm,
                                             key_type: PS_AESKeyType,
                                             nonce: *mut u8) -> Result;
    pub fn PS_GetLocalFriendCodeSeed(seed: *mut u64) -> Result;
    pub fn PS_GetDeviceId(device_id: *mut u32) -> Result;
}
