#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RegionLock {
    None,
    Japan,
    USA,
    Europe,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Charset {
    JapanUSAEurope,
    China,
    Korea,
    Taiwan,
}

#[derive(Copy, Clone, Debug)]
pub struct MiiDataOptions {
    pub is_copying_allowed: bool,
    pub is_profanity_flag_enabled: bool,
    pub region_lock: RegionLock,
    pub charset: Charset,
}

#[derive(Copy, Clone, Debug)]
pub struct SelectorPosition {
    pub page_index: u8,
    pub slot_index: u8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OriginConsole {
    ConsoleWii,
    ConsoleDSi,
    Console3DS,
    ConsoleWiiUSwitch,
}

#[derive(Copy, Clone, Debug)]
pub struct ConsoleIdentity {
    pub origin_console: OriginConsole,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MiiSex {
    Male,
    Female,
}

#[derive(Copy, Clone, Debug)]
pub struct Details {
    pub sex: MiiSex,
    pub birthday_month: u8,
    pub birthday_day: u8,
    pub shirt_color: u8,
    pub is_favorite: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct FaceStyle {
    pub is_sharing_enabled: bool,
    pub shape: u8,
    pub skin_color: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct FaceDetails {
    pub wrinkles: u8,
    pub makeup: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct HairDetails {
    pub color: u8,
    pub is_flipped: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct EyeDetails {
    pub style: u8,
    pub color: u8,
    pub scale: u8,
    pub y_scale: u8,
    pub rotation: u8,
    pub x_spacing: u8,
    pub y_position: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct EyebrowDetails {
    pub style: u8,
    pub color: u8,
    pub scale: u8,
    pub y_scale: u8,
    pub rotation: u8,
    pub x_spacing: u8,
    pub y_position: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct NoseDetails {
    pub style: u8,
    pub scale: u8,
    pub y_position: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct MouthDetails {
    pub style: u8,
    pub color: u8,
    pub scale: u8,
    pub y_scale: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct MustacheDetails {
    pub mouth_y_position: u8,
    pub mustache_style: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct BeardDetails {
    pub style: u8,
    pub color: u8,
    pub scale: u8,
    pub y_position: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct GlassDetails {
    pub style: u8,
    pub color: u8,
    pub scale: u8,
    pub y_position: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct MoleDetails {
    pub is_enabled: bool,
    pub scale: u8,
    pub x_position: u8,
    pub y_position: u8,
}

#[derive(Clone, Debug)]
pub struct MiiData {
    pub options: MiiDataOptions,
    pub selector_position: SelectorPosition,
    pub console_identity: ConsoleIdentity,

    pub system_id: [u8; 8],
    pub mac_address: [u8; 6],

    pub details: Details,
    pub name: String,

    pub height: u8,
    pub width: u8,

    pub face_style: FaceStyle,
    pub face_details: FaceDetails,

    pub hair_style: u8,

    pub hair_details: HairDetails,
    pub eye_details: EyeDetails,
    pub eyebrow_details: EyebrowDetails,
    pub nose_details: NoseDetails,
    pub mouth_details: MouthDetails,
    pub mustache_details: MustacheDetails,
    pub beard_details: BeardDetails,
    pub glass_details: GlassDetails,
    pub mole_details: MoleDetails,

    pub author_name: String,
}

impl From<ctru_sys::MiiData> for MiiData {
    fn from(mii_data: ctru_sys::MiiData) -> Self {
        let raw_mii_data = mii_data._bindgen_opaque_blob;
        // Source for the representation and what each thing means: https://www.3dbrew.org/wiki/Mii
        let raw_options = vec_bit(raw_mii_data[0x1]);
        let raw_position = vec_bit(raw_mii_data[0x2]);
        let raw_device = vec_bit(raw_mii_data[0x3]);
        let system_id = [
            raw_mii_data[0x4],
            raw_mii_data[0x5],
            raw_mii_data[0x6],
            raw_mii_data[0x7],
            raw_mii_data[0x8],
            raw_mii_data[0x9],
            raw_mii_data[0xA],
            raw_mii_data[0xB],
        ];
        let creator_mac = [
            raw_mii_data[0x10],
            raw_mii_data[0x11],
            raw_mii_data[0x12],
            raw_mii_data[0x13],
            raw_mii_data[0x14],
            raw_mii_data[0x15],
        ];
        let raw_details: [bool; 16] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x18, 0x19])
                .try_into()
                .unwrap();
        let raw_utf16_name = &raw_mii_data[0x1A..0x2D];
        let height = raw_mii_data[0x2E];
        let width = raw_mii_data[0x2F];
        let raw_face_style = vec_bit(raw_mii_data[0x30]);
        let raw_face_details = vec_bit(raw_mii_data[0x31]);
        let hair_style = raw_mii_data[0x32];
        let raw_hair_details = vec_bit(raw_mii_data[0x33]);
        let raw_eye_details: [bool; 32] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x34, 0x35, 0x36, 0x37])
                .try_into()
                .unwrap();
        let raw_eyebrow_details: [bool; 32] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x38, 0x39, 0x3A, 0x3B])
                .try_into()
                .unwrap();
        let raw_nose_details: [bool; 16] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x3C, 0x3D])
                .try_into()
                .unwrap();
        let raw_mouth_details: [bool; 16] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x3E, 0x3F])
                .try_into()
                .unwrap();
        let raw_mustache_details: [bool; 16] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x40, 0x41])
                .try_into()
                .unwrap();
        let raw_beard_details: [bool; 16] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x42, 0x42])
                .try_into()
                .unwrap();
        let raw_glass_details: [bool; 16] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x44, 0x45])
                .try_into()
                .unwrap();
        let raw_mole_details: [bool; 16] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x46, 0x47])
                .try_into()
                .unwrap();
        let raw_utf16_author = &raw_mii_data[0x48..0x5C];

        let mii_name = utf16_byte_pairs_to_string(raw_utf16_name);
        let author_name = utf16_byte_pairs_to_string(raw_utf16_author);

        let options = MiiDataOptions {
            is_copying_allowed: raw_options[0],
            is_profanity_flag_enabled: raw_options[1],
            region_lock: {
                match (raw_options[3], raw_options[2]) {
                    (false, false) => RegionLock::None,
                    (false, true) => RegionLock::Japan,
                    (true, false) => RegionLock::USA,
                    (true, true) => RegionLock::Europe,
                }
            },
            charset: {
                match (raw_options[5], raw_options[4]) {
                    (false, false) => Charset::JapanUSAEurope,
                    (false, true) => Charset::China,
                    (true, false) => Charset::Korea,
                    (true, true) => Charset::Taiwan,
                }
            },
        };

        let position = SelectorPosition {
            page_index: partial_vec_to_u8_with_reverse(&raw_position[0..3]),
            slot_index: partial_vec_to_u8_with_reverse(&raw_position[4..7]),
        };

        let device = ConsoleIdentity {
            origin_console: {
                match (raw_device[6], raw_device[5], raw_device[4]) {
                    (false, false, true) => OriginConsole::ConsoleWii,
                    (false, true, false) => OriginConsole::ConsoleDSi,
                    (false, true, true) => OriginConsole::Console3DS,
                    _ => OriginConsole::ConsoleWiiUSwitch,
                }
            },
        };

        let details = Details {
            sex: {
                match raw_details[0] {
                    true => MiiSex::Female,
                    false => MiiSex::Male,
                }
            },
            birthday_month: partial_vec_to_u8_with_reverse(&raw_details[1..4]),
            birthday_day: partial_vec_to_u8_with_reverse(&raw_details[5..9]),
            shirt_color: partial_vec_to_u8_with_reverse(&raw_details[10..13]),
            is_favorite: raw_details[14],
        };

        let face_style = FaceStyle {
            is_sharing_enabled: !raw_face_style[1],
            shape: partial_vec_to_u8_with_reverse(&raw_face_style[1..4]),
            skin_color: partial_vec_to_u8_with_reverse(&raw_face_style[5..7]),
        };

        let face_details = FaceDetails {
            wrinkles: partial_vec_to_u8_with_reverse(&raw_face_details[0..3]),
            makeup: partial_vec_to_u8_with_reverse(&raw_face_details[4..7]),
        };

        let hair_details = HairDetails {
            color: partial_vec_to_u8_with_reverse(&raw_hair_details[0..2]),
            is_flipped: raw_hair_details[3],
        };

        let eye_details = EyeDetails {
            style: partial_vec_to_u8_with_reverse(&raw_eye_details[0..5]),
            color: partial_vec_to_u8_with_reverse(&raw_eye_details[6..8]),
            scale: partial_vec_to_u8_with_reverse(&raw_eye_details[9..12]),
            y_scale: partial_vec_to_u8_with_reverse(&raw_eye_details[13..15]),
            rotation: partial_vec_to_u8_with_reverse(&raw_eye_details[16..20]),
            x_spacing: partial_vec_to_u8_with_reverse(&raw_eye_details[21..24]),
            y_position: partial_vec_to_u8_with_reverse(&raw_eye_details[25..29]),
        };

        let eyebrow_details = EyebrowDetails {
            style: partial_vec_to_u8_with_reverse(&raw_eyebrow_details[0..4]),
            color: partial_vec_to_u8_with_reverse(&raw_eyebrow_details[5..7]),
            scale: partial_vec_to_u8_with_reverse(&raw_eyebrow_details[8..11]),
            y_scale: partial_vec_to_u8_with_reverse(&raw_eyebrow_details[12..14]),
            rotation: partial_vec_to_u8_with_reverse(&raw_eyebrow_details[16..19]),
            x_spacing: partial_vec_to_u8_with_reverse(&raw_eyebrow_details[21..24]),
            y_position: partial_vec_to_u8_with_reverse(&raw_eyebrow_details[25..29]),
        };

        let nose_details = NoseDetails {
            style: partial_vec_to_u8_with_reverse(&raw_nose_details[0..4]),
            scale: partial_vec_to_u8_with_reverse(&raw_nose_details[5..8]),
            y_position: partial_vec_to_u8_with_reverse(&raw_nose_details[9..13]),
        };

        let mouth_details = MouthDetails {
            style: partial_vec_to_u8_with_reverse(&raw_mouth_details[0..5]),
            color: partial_vec_to_u8_with_reverse(&raw_mouth_details[6..8]),
            scale: partial_vec_to_u8_with_reverse(&raw_mouth_details[9..12]),
            y_scale: partial_vec_to_u8_with_reverse(&raw_mouth_details[13..15]),
        };

        let mustache_details = MustacheDetails {
            mouth_y_position: partial_vec_to_u8_with_reverse(&raw_mustache_details[0..4]),
            mustache_style: partial_vec_to_u8_with_reverse(&raw_mustache_details[5..7]),
        };

        let beard_details = BeardDetails {
            style: partial_vec_to_u8_with_reverse(&raw_beard_details[0..2]),
            color: partial_vec_to_u8_with_reverse(&raw_beard_details[3..5]),
            scale: partial_vec_to_u8_with_reverse(&raw_beard_details[6..9]),
            y_position: partial_vec_to_u8_with_reverse(&raw_beard_details[10..14]),
        };

        let glass_details = GlassDetails {
            style: partial_vec_to_u8_with_reverse(&raw_glass_details[0..3]),
            color: partial_vec_to_u8_with_reverse(&raw_glass_details[4..6]),
            scale: partial_vec_to_u8_with_reverse(&raw_glass_details[7..10]),
            y_position: partial_vec_to_u8_with_reverse(&raw_glass_details[11..15]),
        };

        let mole_details = MoleDetails {
            is_enabled: raw_mole_details[0],
            scale: partial_vec_to_u8_with_reverse(&raw_mole_details[1..4]),
            x_position: partial_vec_to_u8_with_reverse(&raw_mole_details[5..9]),
            y_position: partial_vec_to_u8_with_reverse(&raw_mole_details[10..14]),
        };

        MiiData {
            options: options,
            selector_position: position,
            console_identity: device,
            system_id,
            mac_address: creator_mac,
            details: details,
            name: mii_name,
            height,
            width,
            face_style,
            face_details,
            hair_style,
            hair_details,
            eye_details,
            eyebrow_details,
            nose_details,
            mouth_details,
            mustache_details,
            beard_details,
            glass_details,
            mole_details,
            author_name,
        }
    }
}

// Methods to handle "_bits_", ``bitvec`` cannot compile to 32-bit targets, so I had to create a few
// helper methods

/// Transforms a u8 into a [bool; 8]
fn vec_bit(data: u8) -> [bool; 8] {
    (0..8)
        .map(|i| (data & (1 << i)) != 0)
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap()
}

/// Transforms a [bool; 8] into an u8
fn vec_bit_to_u8(data: [bool; 8]) -> u8 {
    data.into_iter().fold(0, |result, bit| {
        (result << 1) ^ u8::from(bit)
    })
}

/// The reverse allows to write things on a more _humane_ way, but return a LE u8
fn partial_vec_to_u8_with_reverse(data: &[bool]) -> u8 {
    let leading_zeroes_to_add = 8 - data.len();
    let leading_zeroes = vec![false; leading_zeroes_to_add];
    let mut val = [data, leading_zeroes.as_slice()].concat();
    val.reverse();
    vec_bit_to_u8(val.try_into().unwrap())
}

/// UTF-16 Strings are give in pairs of bytes (u8), this converts them into an _actual_ string
fn utf16_byte_pairs_to_string(data: &[u8]) -> String {
    let raw_utf16_composed = data
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<u16>>();

    String::from_utf16_lossy(raw_utf16_composed.as_slice()).replace("\0", "")
}

/// Gets the values from the slice and concatenates them
fn get_and_concat_vec_bit(data: &[u8], get_values: &[usize]) -> Vec<bool> {
    get_values
        .iter()
        .flat_map(|v| vec_bit(data[*v]))
        .collect()
}
