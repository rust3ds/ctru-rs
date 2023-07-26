//! Mii data.
//!
//! This module contains the structs that represent all the data of a Mii.
//!
//! Have a look at the [`MiiSelector`](crate::applets::mii_selector::MiiSelector) applet to learn how to ask the user for a specific Mii.

/// Region lock of the Mii.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RegionLock {
    /// No region-lock.
    None,
    /// Japan region-lock.
    Japan,
    /// USA region-lock.
    USA,
    /// Europe region-lock.
    Europe,
}

/// Charset of the Mii.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Charset {
    /// Japan-USA-Europe unified charset.
    JapanUSAEurope,
    /// China charset.
    China,
    /// Korea charset.
    Korea,
    /// Taiwan charset.
    Taiwan,
}

/// Generic options of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct Options {
    /// Whether it is allowed to copy the Mii.
    pub is_copying_allowed: bool,
    /// Whether the profanity flag is active.
    pub is_profanity_flag_enabled: bool,
    /// The Mii's active region-lock.
    pub region_lock: RegionLock,
    /// The Mii's used charset.
    pub charset: Charset,
}

/// Positional Index that the Mii has on the [`MiiSelector`](crate::applets::mii_selector::MiiSelector) window.
#[derive(Copy, Clone, Debug)]
pub struct SelectorPosition {
    /// Index of the page where the Mii is found.
    pub page_index: u8,
    /// Index of the slot (relative to the page) where the Mii is found.
    pub slot_index: u8,
}

/// Console model from which the Mii originated.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OriginConsole {
    /// Nintendo Wii.
    Wii,
    /// Nintendo DSi.
    DSi,
    /// Nintendo 3DS.
    ///
    /// This includes all consoles of the 3DS family (3DS, 2DS, and their respective "New" or "XL" variants).
    N3DS,
    /// Nintendo Wii U/Switch.
    WiiUSwitch,
}

/// Identity of the origin console.
#[derive(Copy, Clone, Debug)]
pub struct ConsoleIdentity {
    /// From which console the Mii originated from.
    pub origin_console: OriginConsole,
}

/// Sex of the Mii.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Sex {
    /// Male sex.
    Male,
    /// Female sex.
    Female,
}

/// Generic details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct Details {
    /// Sex of the Mii.
    pub sex: Sex,
    /// Birthday month.
    pub birthday_month: u8,
    /// Birthday day.
    pub birthday_day: u8,
    /// Color of the Mii's shirt.
    pub shirt_color: u8,
    /// Whether the Mii is a favorite.
    pub is_favorite: bool,
    /// Whether the Mii can be shared.
    pub is_sharing_enabled: bool,
}

/// Face style of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct FaceStyle {
    /// Face shape.
    pub shape: u8,
    /// Skin color.
    pub skin_color: u8,
}

/// Face details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct FaceDetails {
    /// Face style.
    pub style: FaceStyle,
    /// Wrinkles.
    pub wrinkles: u8,
    /// Makeup.
    pub makeup: u8,
}

/// Hair details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct HairDetails {
    /// Hair style.
    pub style: u8,
    /// Hair color.
    pub color: u8,
    /// Whether the Mii's hair is flipped.
    pub is_flipped: bool,
}

/// Eye details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct EyeDetails {
    /// Eye style.
    pub style: u8,
    /// Eye color.
    pub color: u8,
    /// Eye scale.
    pub scale: u8,
    /// Eye scale (y-axis).
    pub y_scale: u8,
    /// Eye rotation.
    pub rotation: u8,
    /// Spacing between the eyes.
    pub x_spacing: u8,
    /// Eye height.
    pub y_position: u8,
}

/// Eyebrow details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct EyebrowDetails {
    /// Eyebrow style.
    pub style: u8,
    /// Eyebrow color.
    pub color: u8,
    /// Eyebrow scale.
    pub scale: u8,
    /// Eyebrow scale (y-axis).
    pub y_scale: u8,
    /// Eyebrow rotation.
    pub rotation: u8,
    /// Spacing between the eyebrows
    pub x_spacing: u8,
    /// Eyebrow height.
    pub y_position: u8,
}

/// Nose details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct NoseDetails {
    /// Nose style.
    pub style: u8,
    /// Nose scale.
    pub scale: u8,
    /// Nose height.
    pub y_position: u8,
}

/// Mouth details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct MouthDetails {
    /// Mouth style.
    pub style: u8,
    /// Mouth color.
    pub color: u8,
    /// Mouth scale.
    pub scale: u8,
    /// Mouth scale (y-axis).
    pub y_scale: u8,
    /// Mouth height.
    pub y_position: u8,
}

/// Mustache details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct MustacheDetails {
    /// Mustache style.
    pub mustache_style: u8,
}

/// Beard details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct BeardDetails {
    /// Beard style
    pub style: u8,
    /// Beard color.
    pub color: u8,
    /// Beard scale.
    pub scale: u8,
    /// Beard height.
    pub y_position: u8,
}

/// Glasses details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct GlassesDetails {
    /// Glasses style.
    pub style: u8,
    /// Glasses color.
    pub color: u8,
    /// Glasses scale.
    pub scale: u8,
    /// Glasses height.
    pub y_position: u8,
}

/// Mole details of the Mii.
#[derive(Copy, Clone, Debug)]
pub struct MoleDetails {
    /// Whether the Mii has a mole.
    pub is_enabled: bool,
    /// Mole scale.
    pub scale: u8,
    /// Mole position (x-axis).
    pub x_position: u8,
    /// Mole position (y-axis).
    pub y_position: u8,
}

/// Full Mii data representation.
///
/// Some values are not ordered *like* the Mii Editor UI. The mapped values can be seen [here](https://www.3dbrew.org/wiki/Mii#Mapped_Editor_.3C-.3E_Hex_values).
///
/// This struct can be retrieved by [`MiiSelector::lauch()`](crate::applets::mii_selector::MiiSelector::launch).
#[derive(Clone, Debug)]
pub struct Mii {
    /// Mii options.
    pub options: Options,
    /// Position taken by the Mii on the Mii Selector screen.
    pub selector_position: SelectorPosition,
    /// Console the Mii was created on.
    pub console_identity: ConsoleIdentity,

    /// Unique system ID, not dependant on the MAC address
    pub system_id: [u8; 8],
    /// Console's MAC address.
    pub mac_address: [u8; 6],

    /// General information about the Mii.
    pub details: Details,
    /// Mii name.
    pub name: String,

    /// Mii height.
    pub height: u8,
    /// Mii width.
    pub width: u8,

    /// Face details.
    pub face_details: FaceDetails,
    /// Hair details.
    pub hair_details: HairDetails,
    /// Eyes details.
    pub eye_details: EyeDetails,
    /// Eyebrow details.
    pub eyebrow_details: EyebrowDetails,
    /// Nose details.
    pub nose_details: NoseDetails,
    /// Mouth details.
    pub mouth_details: MouthDetails,
    /// Mustache details.
    pub mustache_details: MustacheDetails,
    /// Beard details.
    pub beard_details: BeardDetails,
    /// Glasses details.
    pub glass_details: GlassesDetails,
    /// Mole details.
    pub mole_details: MoleDetails,

    /// Name of the Mii's original author.
    pub author_name: String,
}

impl From<ctru_sys::MiiData> for Mii {
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
        let mac_address = [
            raw_mii_data[0x10],
            raw_mii_data[0x11],
            raw_mii_data[0x12],
            raw_mii_data[0x13],
            raw_mii_data[0x14],
            raw_mii_data[0x15],
        ];
        let raw_details: [bool; 16] = get_and_concat_vec_bit(&raw_mii_data, &[0x18, 0x19])
            .try_into()
            .unwrap();
        let raw_utf16_name = &raw_mii_data[0x1A..0x2D];
        let height = raw_mii_data[0x2E];
        let width = raw_mii_data[0x2F];
        let raw_face_style = vec_bit(raw_mii_data[0x30]);
        let raw_face_details = vec_bit(raw_mii_data[0x31]);
        let raw_hair_details = vec_bit(raw_mii_data[0x33]);
        let raw_eye_details: [bool; 32] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x34, 0x35, 0x36, 0x37])
                .try_into()
                .unwrap();
        let raw_eyebrow_details: [bool; 32] =
            get_and_concat_vec_bit(&raw_mii_data, &[0x38, 0x39, 0x3A, 0x3B])
                .try_into()
                .unwrap();
        let raw_nose_details: [bool; 16] = get_and_concat_vec_bit(&raw_mii_data, &[0x3C, 0x3D])
            .try_into()
            .unwrap();
        let raw_mouth_details: [bool; 16] = get_and_concat_vec_bit(&raw_mii_data, &[0x3E, 0x3F])
            .try_into()
            .unwrap();
        let raw_mustache_details: [bool; 16] = get_and_concat_vec_bit(&raw_mii_data, &[0x40, 0x41])
            .try_into()
            .unwrap();
        let raw_beard_details: [bool; 16] = get_and_concat_vec_bit(&raw_mii_data, &[0x42, 0x42])
            .try_into()
            .unwrap();
        let raw_glass_details: [bool; 16] = get_and_concat_vec_bit(&raw_mii_data, &[0x44, 0x45])
            .try_into()
            .unwrap();
        let raw_mole_details: [bool; 16] = get_and_concat_vec_bit(&raw_mii_data, &[0x46, 0x47])
            .try_into()
            .unwrap();
        let raw_utf16_author = &raw_mii_data[0x48..0x5C];

        let name = utf16_byte_pairs_to_string(raw_utf16_name);
        let author_name = utf16_byte_pairs_to_string(raw_utf16_author);

        let options = Options {
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

        let selector_position = SelectorPosition {
            page_index: partial_u8_bits_to_u8(&raw_position[0..=3]),
            slot_index: partial_u8_bits_to_u8(&raw_position[4..=7]),
        };

        let console_identity = ConsoleIdentity {
            origin_console: {
                match (raw_device[6], raw_device[5], raw_device[4]) {
                    (false, false, true) => OriginConsole::Wii,
                    (false, true, false) => OriginConsole::DSi,
                    (false, true, true) => OriginConsole::N3DS,
                    _ => OriginConsole::WiiUSwitch,
                }
            },
        };

        let details = Details {
            sex: {
                match raw_details[0] {
                    true => Sex::Female,
                    false => Sex::Male,
                }
            },
            birthday_month: partial_u8_bits_to_u8(&raw_details[1..=4]),
            birthday_day: partial_u8_bits_to_u8(&raw_details[5..=9]),
            shirt_color: partial_u8_bits_to_u8(&raw_details[10..=13]),
            is_favorite: raw_details[14],
            is_sharing_enabled: !raw_face_style[0],
        };

        let face_details = FaceDetails {
            style: FaceStyle {
                shape: partial_u8_bits_to_u8(&raw_face_style[1..=4]),
                skin_color: partial_u8_bits_to_u8(&raw_face_style[5..=7]),
            },
            wrinkles: partial_u8_bits_to_u8(&raw_face_details[0..=3]),
            makeup: partial_u8_bits_to_u8(&raw_face_details[4..=7]),
        };

        let hair_details = HairDetails {
            style: raw_mii_data[0x32],
            color: partial_u8_bits_to_u8(&raw_hair_details[0..=2]),
            is_flipped: raw_hair_details[3],
        };

        let eye_details = EyeDetails {
            style: partial_u8_bits_to_u8(&raw_eye_details[0..=5]),
            color: partial_u8_bits_to_u8(&raw_eye_details[6..=8]),
            scale: partial_u8_bits_to_u8(&raw_eye_details[9..=12]),
            y_scale: partial_u8_bits_to_u8(&raw_eye_details[13..=15]),
            rotation: partial_u8_bits_to_u8(&raw_eye_details[16..=20]),
            x_spacing: partial_u8_bits_to_u8(&raw_eye_details[21..=24]),
            y_position: partial_u8_bits_to_u8(&raw_eye_details[25..=29]),
        };

        let eyebrow_details = EyebrowDetails {
            style: partial_u8_bits_to_u8(&raw_eyebrow_details[0..=4]),
            color: partial_u8_bits_to_u8(&raw_eyebrow_details[5..=7]),
            scale: partial_u8_bits_to_u8(&raw_eyebrow_details[8..=11]),
            // Bits are skipped here, following the 3dbrew wiki:
            // https://www.3dbrew.org/wiki/Mii#Mii_format offset 0x38
            y_scale: partial_u8_bits_to_u8(&raw_eyebrow_details[12..=14]),
            rotation: partial_u8_bits_to_u8(&raw_eyebrow_details[16..=19]),
            x_spacing: partial_u8_bits_to_u8(&raw_eyebrow_details[21..=24]),
            y_position: partial_u8_bits_to_u8(&raw_eyebrow_details[25..=29]),
        };

        let nose_details = NoseDetails {
            style: partial_u8_bits_to_u8(&raw_nose_details[0..=4]),
            scale: partial_u8_bits_to_u8(&raw_nose_details[5..=8]),
            y_position: partial_u8_bits_to_u8(&raw_nose_details[9..=13]),
        };

        let mouth_details = MouthDetails {
            style: partial_u8_bits_to_u8(&raw_mouth_details[0..=5]),
            color: partial_u8_bits_to_u8(&raw_mouth_details[6..=8]),
            scale: partial_u8_bits_to_u8(&raw_mouth_details[9..=12]),
            y_scale: partial_u8_bits_to_u8(&raw_mouth_details[13..=15]),
            y_position: partial_u8_bits_to_u8(&raw_mustache_details[0..=4]),
        };

        let mustache_details = MustacheDetails {
            mustache_style: partial_u8_bits_to_u8(&raw_mustache_details[5..=7]),
        };

        let beard_details = BeardDetails {
            style: partial_u8_bits_to_u8(&raw_beard_details[0..=2]),
            color: partial_u8_bits_to_u8(&raw_beard_details[3..=5]),
            scale: partial_u8_bits_to_u8(&raw_beard_details[6..=9]),
            y_position: partial_u8_bits_to_u8(&raw_beard_details[10..=14]),
        };

        let glass_details = GlassesDetails {
            style: partial_u8_bits_to_u8(&raw_glass_details[0..=3]),
            color: partial_u8_bits_to_u8(&raw_glass_details[4..=6]),
            scale: partial_u8_bits_to_u8(&raw_glass_details[7..=10]),
            y_position: partial_u8_bits_to_u8(&raw_glass_details[11..=15]),
        };

        let mole_details = MoleDetails {
            is_enabled: raw_mole_details[0],
            scale: partial_u8_bits_to_u8(&raw_mole_details[1..=4]),
            x_position: partial_u8_bits_to_u8(&raw_mole_details[5..=9]),
            y_position: partial_u8_bits_to_u8(&raw_mole_details[10..=14]),
        };

        Mii {
            options,
            selector_position,
            console_identity,
            system_id,
            mac_address,
            details,
            name,
            height,
            width,
            face_details,
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
    data.into_iter()
        .fold(0, |result, bit| (result << 1) ^ u8::from(bit))
}

/// Given a series of LE bits, they are filled until a full LE u8 is reached
fn partial_u8_bits_to_u8(data: &[bool]) -> u8 {
    let leading_zeroes_to_add = 8 - data.len();
    let leading_zeroes = vec![false; leading_zeroes_to_add];

    vec_bit_to_u8([data, &leading_zeroes].concat().try_into().unwrap())
}

/// UTF-16 Strings are give in pairs of bytes (u8), this converts them into an _actual_ string
fn utf16_byte_pairs_to_string(data: &[u8]) -> String {
    let raw_utf16_composed = data
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<u16>>();

    String::from_utf16_lossy(raw_utf16_composed.as_slice()).replace('\0', "")
}

/// Gets the values from the slice and concatenates them
fn get_and_concat_vec_bit(data: &[u8], get_values: &[usize]) -> Vec<bool> {
    get_values.iter().flat_map(|v| vec_bit(data[*v])).collect()
}
