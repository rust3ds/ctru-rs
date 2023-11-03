//! System Configuration service.
//!
//! This module contains basic methods to retrieve the console's system configuration.
#![doc(alias = "configuration")]

use crate::error::ResultCode;

/// Console region.
#[doc(alias = "CFG_Region")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Region {
    /// Japan.
    Japan = ctru_sys::CFG_REGION_JPN,
    /// USA.
    USA = ctru_sys::CFG_REGION_USA,
    /// Europe.
    Europe = ctru_sys::CFG_REGION_EUR,
    /// Australia.
    Australia = ctru_sys::CFG_REGION_AUS,
    /// China.
    China = ctru_sys::CFG_REGION_CHN,
    /// Korea.
    Korea = ctru_sys::CFG_REGION_KOR,
    /// Taiwan.
    Taiwan = ctru_sys::CFG_REGION_TWN,
}

/// Language set for the console's OS.
#[doc(alias = "CFG_Language")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Language {
    /// Japanese.
    Japanese = ctru_sys::CFG_LANGUAGE_JP,
    /// English.
    English = ctru_sys::CFG_LANGUAGE_EN,
    /// French.
    French = ctru_sys::CFG_LANGUAGE_FR,
    /// German.
    German = ctru_sys::CFG_LANGUAGE_DE,
    /// Italian.
    Italian = ctru_sys::CFG_LANGUAGE_IT,
    /// Spanish.
    Spanish = ctru_sys::CFG_LANGUAGE_ES,
    /// Korean.
    Korean = ctru_sys::CFG_LANGUAGE_KO,
    /// Dutch.
    Dutch = ctru_sys::CFG_LANGUAGE_NL,
    /// Portuguese.
    Portuguese = ctru_sys::CFG_LANGUAGE_PT,
    /// Russian.
    Russian = ctru_sys::CFG_LANGUAGE_RU,
    /// Simplified Chinese.
    SimplifiedChinese = ctru_sys::CFG_LANGUAGE_ZH,
    /// Traditional Chinese.
    TraditionalChinese = ctru_sys::CFG_LANGUAGE_TW,
}

/// Specific model of the console.
#[doc(alias = "CFG_SystemModel")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SystemModel {
    /// Old Nintendo 3DS.
    Old3DS = ctru_sys::CFG_MODEL_3DS,
    /// Old Nintendo 3DS XL.
    Old3DSXL = ctru_sys::CFG_MODEL_3DSXL,
    /// New Nintendo 3DS.
    New3DS = ctru_sys::CFG_MODEL_N3DS,
    /// Old Nintendo 2DS.
    Old2DS = ctru_sys::CFG_MODEL_2DS,
    /// New Nintendo 3DS XL.
    New3DSXL = ctru_sys::CFG_MODEL_N3DSXL,
    /// New Nintendo 2DS XL.
    New2DSXL = ctru_sys::CFG_MODEL_N2DSXL,
}

/// Handle to the System Configuration service.
pub struct Cfgu(());

impl Cfgu {
    /// Initialize a new service handle.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cfgu::Cfgu;
    ///
    /// let cfgu = Cfgu::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "cfguInit")]
    pub fn new() -> crate::Result<Cfgu> {
        ResultCode(unsafe { ctru_sys::cfguInit() })?;
        Ok(Cfgu(()))
    }

    /// Returns the console's region from the system's secure info.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cfgu::Cfgu;
    /// let cfgu = Cfgu::new()?;
    ///
    /// let region = cfgu.region()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "CFGU_SecureInfoGetRegion")]
    pub fn region(&self) -> crate::Result<Region> {
        let mut region: u8 = 0;

        ResultCode(unsafe { ctru_sys::CFGU_SecureInfoGetRegion(&mut region) })?;
        Ok(Region::try_from(region).unwrap())
    }

    /// Returns the console's model.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cfgu::Cfgu;
    /// let cfgu = Cfgu::new()?;
    ///
    /// let model = cfgu.model()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "CFGU_GetSystemModel")]
    pub fn model(&self) -> crate::Result<SystemModel> {
        let mut model: u8 = 0;

        ResultCode(unsafe { ctru_sys::CFGU_GetSystemModel(&mut model) })?;
        Ok(SystemModel::try_from(model).unwrap())
    }

    /// Returns the system language set for the console.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cfgu::Cfgu;
    /// let cfgu = Cfgu::new()?;
    ///
    /// let language = cfgu.language()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "CFGU_GetSystemLanguage")]
    pub fn language(&self) -> crate::Result<Language> {
        let mut language: u8 = 0;

        ResultCode(unsafe { ctru_sys::CFGU_GetSystemLanguage(&mut language) })?;
        Ok(Language::try_from(language).unwrap())
    }

    /// Check if NFC is supported by the console.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cfgu::Cfgu;
    /// let cfgu = Cfgu::new()?;
    ///
    /// if cfgu.is_nfc_supported()? {
    ///     println!("NFC is available!");
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "CFGU_IsNFCSupported")]
    pub fn is_nfc_supported(&self) -> crate::Result<bool> {
        let mut supported: bool = false;

        ResultCode(unsafe { ctru_sys::CFGU_IsNFCSupported(&mut supported) })?;
        Ok(supported)
    }

    /// Check if the console is from the 2DS family ([`Old2DS`](SystemModel::Old2DS), [`New2DSXL`](SystemModel::New2DSXL)).
    ///
    /// Useful to avoid stereoscopic 3D rendering when working with 2DS consoles.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::cfgu::Cfgu;
    /// let cfgu = Cfgu::new()?;
    ///
    /// if cfgu.is_2ds_family()? {
    ///     println!("Stereoscopic 3D is not supported.");
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "CFGU_GetModelNintendo2DS")]
    pub fn is_2ds_family(&self) -> crate::Result<bool> {
        let mut is_2ds_family: u8 = 0;

        ResultCode(unsafe { ctru_sys::CFGU_GetModelNintendo2DS(&mut is_2ds_family) })?;
        Ok(is_2ds_family == 0)
    }
}

impl Drop for Cfgu {
    #[doc(alias = "cfguExit")]
    fn drop(&mut self) {
        unsafe {
            ctru_sys::cfguExit();
        }
    }
}

from_impl!(Region, u8);
from_impl!(Language, u8);
from_impl!(SystemModel, u8);

impl TryFrom<u8> for Region {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as u32 {
            ctru_sys::CFG_REGION_JPN => Ok(Region::Japan),
            ctru_sys::CFG_REGION_USA => Ok(Region::USA),
            ctru_sys::CFG_REGION_EUR => Ok(Region::Europe),
            ctru_sys::CFG_REGION_AUS => Ok(Region::Australia),
            ctru_sys::CFG_REGION_CHN => Ok(Region::China),
            ctru_sys::CFG_REGION_KOR => Ok(Region::Korea),
            ctru_sys::CFG_REGION_TWN => Ok(Region::Taiwan),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Language {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as u32 {
            ctru_sys::CFG_LANGUAGE_JP => Ok(Language::Japanese),
            ctru_sys::CFG_LANGUAGE_EN => Ok(Language::English),
            ctru_sys::CFG_LANGUAGE_FR => Ok(Language::French),
            ctru_sys::CFG_LANGUAGE_DE => Ok(Language::German),
            ctru_sys::CFG_LANGUAGE_IT => Ok(Language::Italian),
            ctru_sys::CFG_LANGUAGE_ES => Ok(Language::Spanish),
            ctru_sys::CFG_LANGUAGE_ZH => Ok(Language::SimplifiedChinese),
            ctru_sys::CFG_LANGUAGE_KO => Ok(Language::Korean),
            ctru_sys::CFG_LANGUAGE_NL => Ok(Language::Dutch),
            ctru_sys::CFG_LANGUAGE_PT => Ok(Language::Portuguese),
            ctru_sys::CFG_LANGUAGE_RU => Ok(Language::Russian),
            ctru_sys::CFG_LANGUAGE_TW => Ok(Language::TraditionalChinese),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for SystemModel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as u32 {
            ctru_sys::CFG_MODEL_3DS => Ok(SystemModel::Old3DS),
            ctru_sys::CFG_MODEL_3DSXL => Ok(SystemModel::Old3DSXL),
            ctru_sys::CFG_MODEL_N3DS => Ok(SystemModel::New3DS),
            ctru_sys::CFG_MODEL_2DS => Ok(SystemModel::Old2DS),
            ctru_sys::CFG_MODEL_N3DSXL => Ok(SystemModel::New3DSXL),
            ctru_sys::CFG_MODEL_N2DSXL => Ok(SystemModel::New2DSXL),
            _ => Err(()),
        }
    }
}
