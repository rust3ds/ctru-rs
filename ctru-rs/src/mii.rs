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
/// This struct can be retrieved by [`MiiSelector::launch()`](crate::applets::mii_selector::MiiSelector::launch).
#[derive(Clone, Debug)]
pub struct Mii {
    /// Mii options.
    pub options: Options,
    /// Position taken by the Mii on the Mii Selector screen.
    pub selector_position: SelectorPosition,
    /// Console the Mii was created on.
    pub console_identity: ConsoleIdentity,

    /// Unique system ID, not dependant on the MAC address
    pub system_id: u64,
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
    pub glasses_details: GlassesDetails,
    /// Mole details.
    pub mole_details: MoleDetails,

    /// Name of the Mii's original author.
    pub author_name: String,
}

impl From<ctru_sys::MiiData> for Mii {
    fn from(mii_data: ctru_sys::MiiData) -> Self {
        // Source for the representation and what each thing means: https://www.3dbrew.org/wiki/Mii
        let raw_options = mii_data.mii_options._bitfield_1;
        let raw_position = mii_data.mii_pos._bitfield_1;
        let raw_device = mii_data.console_identity._bitfield_1;
        let system_id = mii_data.system_id;
        let mac_address = mii_data.mac;
        let raw_details = mii_data.mii_details._bitfield_1;
        let raw_utf16_name = mii_data.mii_name;
        let height = mii_data.height;
        let width = mii_data.width;
        let raw_face_style = mii_data.face_style._bitfield_1;
        let raw_face_details = mii_data.face_details._bitfield_1;
        let raw_hair_style = mii_data.hair_style;
        let raw_hair_details = mii_data.hair_details._bitfield_1;
        let raw_eye_details = mii_data.eye_details._bitfield_1;
        let raw_eyebrow_details = mii_data.eyebrow_details._bitfield_1;
        let raw_nose_details = mii_data.nose_details._bitfield_1;
        let raw_mouth_details = mii_data.mouth_details._bitfield_1;
        let raw_mustache_details = mii_data.mustache_details._bitfield_1;
        let raw_beard_details = mii_data.beard_details._bitfield_1;
        let raw_glasses_details = mii_data.glasses_details._bitfield_1;
        let raw_mole_details = mii_data.mole_details._bitfield_1;
        let raw_utf16_author = mii_data.author_name;

        let name = String::from_utf16_lossy(&raw_utf16_name).replace('\0', "");
        let author_name = String::from_utf16_lossy(&raw_utf16_author).replace('\0', "");

        let options = Options {
            is_copying_allowed: raw_options.get_bit(0),
            is_profanity_flag_enabled: raw_options.get_bit(1),
            region_lock: {
                match (raw_options.get_bit(3), raw_options.get_bit(2)) {
                    (false, false) => RegionLock::None,
                    (false, true) => RegionLock::Japan,
                    (true, false) => RegionLock::USA,
                    (true, true) => RegionLock::Europe,
                }
            },
            charset: {
                match (raw_options.get_bit(5), raw_options.get_bit(4)) {
                    (false, false) => Charset::JapanUSAEurope,
                    (false, true) => Charset::China,
                    (true, false) => Charset::Korea,
                    (true, true) => Charset::Taiwan,
                }
            },
        };

        let selector_position = SelectorPosition {
            page_index: raw_position.get(0, 4) as u8, // index 0 to 3
            slot_index: raw_position.get(4, 4) as u8, // index 4 to 7
        };

        let console_identity = ConsoleIdentity {
            origin_console: {
                match (
                    raw_device.get_bit(6),
                    raw_device.get_bit(5),
                    raw_device.get_bit(4),
                ) {
                    (false, false, true) => OriginConsole::Wii,
                    (false, true, false) => OriginConsole::DSi,
                    (false, true, true) => OriginConsole::N3DS,
                    _ => OriginConsole::WiiUSwitch,
                }
            },
        };

        let details = Details {
            sex: {
                match raw_details.get_bit(0) {
                    true => Sex::Female,
                    false => Sex::Male,
                }
            },
            birthday_month: raw_details.get(1, 4) as u8, // index 1 to 4
            birthday_day: raw_details.get(5, 5) as u8,   // index 5 to 9
            shirt_color: raw_details.get(10, 4) as u8,   // index 10 to 13
            is_favorite: raw_details.get_bit(14),
            is_sharing_enabled: !raw_face_style.get_bit(0),
        };

        let face_details = FaceDetails {
            style: FaceStyle {
                shape: raw_face_style.get(1, 4) as u8,      // index 1 to 4
                skin_color: raw_face_style.get(5, 3) as u8, // index 5 to 7
            },
            wrinkles: raw_face_details.get(0, 4) as u8, // index 0 to 3
            makeup: raw_face_details.get(4, 4) as u8,   // index 4 to 7
        };

        let hair_details = HairDetails {
            style: raw_hair_style,
            color: raw_hair_details.get(0, 3) as u8, // index 0 to 2
            is_flipped: raw_hair_details.get_bit(3),
        };

        let eye_details = EyeDetails {
            style: raw_eye_details.get(0, 6) as u8,       // index 0 to 5
            color: raw_eye_details.get(6, 3) as u8,       // index 6 to 8
            scale: raw_eye_details.get(9, 4) as u8,       // index 9 to 12
            y_scale: raw_eye_details.get(13, 3) as u8,    // index 13 to 15
            rotation: raw_eye_details.get(16, 5) as u8,   // index 16 to 20
            x_spacing: raw_eye_details.get(21, 4) as u8,  // index 21 to 24
            y_position: raw_eye_details.get(25, 5) as u8, // index 25 to 29
        };

        let eyebrow_details = EyebrowDetails {
            style: raw_eyebrow_details.get(0, 5) as u8, // index 0 to 4
            color: raw_eyebrow_details.get(5, 3) as u8, // index 5 to 7
            scale: raw_eyebrow_details.get(8, 4) as u8, // index 8 to 11
            y_scale: raw_eyebrow_details.get(12, 3) as u8, // index 12 to 14
            // Bits are skipped here, following the 3dbrew wiki:
            // https://www.3dbrew.org/wiki/Mii#Mii_format offset 0x38
            rotation: raw_eyebrow_details.get(16, 4) as u8, // index 16 to 19
            x_spacing: raw_eyebrow_details.get(21, 4) as u8, // index 21 to 24
            y_position: raw_eyebrow_details.get(25, 5) as u8, // index 25 to 29
        };

        let nose_details = NoseDetails {
            style: raw_nose_details.get(0, 5) as u8,      // index 0 to 4
            scale: raw_nose_details.get(5, 4) as u8,      // index 5 to 8
            y_position: raw_nose_details.get(9, 5) as u8, // index 9 to 13
        };

        let mouth_details = MouthDetails {
            style: raw_mouth_details.get(0, 6) as u8,    // index 0 to 5
            color: raw_mouth_details.get(6, 3) as u8,    // index 6 to 8
            scale: raw_mouth_details.get(9, 4) as u8,    // index 9 to 12
            y_scale: raw_mouth_details.get(13, 3) as u8, // index 13 to 15
            y_position: raw_mustache_details.get(0, 5) as u8, // index 0 to 4
        };

        let mustache_details = MustacheDetails {
            mustache_style: raw_mustache_details.get(5, 3) as u8, // index 5 to 7
        };

        let beard_details = BeardDetails {
            style: raw_beard_details.get(0, 3) as u8, // index 0 to 2
            color: raw_beard_details.get(3, 6) as u8, // index 3 to 5
            scale: raw_beard_details.get(6, 4) as u8, // index 6 to 9
            y_position: raw_beard_details.get(10, 5) as u8, // index 10 to 14
        };

        let glasses_details = GlassesDetails {
            style: raw_glasses_details.get(0, 4) as u8, // index 0 to 3
            color: raw_glasses_details.get(4, 3) as u8, // index 4 to 6
            scale: raw_glasses_details.get(7, 4) as u8, // index 7 to 10
            y_position: raw_glasses_details.get(11, 5) as u8, // index 11 to 15
        };

        let mole_details = MoleDetails {
            is_enabled: raw_mole_details.get_bit(0),
            scale: raw_mole_details.get(1, 4) as u8, // index 1 to 4
            x_position: raw_mole_details.get(5, 5) as u8, // index 5 to 9
            y_position: raw_mole_details.get(10, 5) as u8, // index 10 to 14
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
            glasses_details,
            mole_details,
            author_name,
        }
    }
}
