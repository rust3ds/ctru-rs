//! Configuration service
//!
//! This module contains basic methods to retrieve and change configuration from the console.

#[derive(Debug)]
#[repr(u32)]
pub enum Region {
    Japan       = ctru_sys::CFG_REGION_JPN,
    Usa         = ctru_sys::CFG_REGION_USA,
    Europe      = ctru_sys::CFG_REGION_EUR,
    Australia   = ctru_sys::CFG_REGION_AUS,
    China       = ctru_sys::CFG_REGION_CHN,
    Korea       = ctru_sys::CFG_REGION_KOR,
    Taiwan      = ctru_sys::CFG_REGION_TWN,
}

#[derive(Debug)]
#[repr(u32)]
pub enum Language {
    Japan       = ctru_sys::CFG_LANGUAGE_JP,
    English     = ctru_sys::CFG_LANGUAGE_EN,
    French      = ctru_sys::CFG_LANGUAGE_FR,
    German      = ctru_sys::CFG_LANGUAGE_DE,
    Italian     = ctru_sys::CFG_LANGUAGE_IT,
    Spanish     = ctru_sys::CFG_LANGUAGE_ES,
    SimpChinese = ctru_sys::CFG_LANGUAGE_ZH,
    Korean      = ctru_sys::CFG_LANGUAGE_KO,
    Dutch       = ctru_sys::CFG_LANGUAGE_NL,
    Portuguese  = ctru_sys::CFG_LANGUAGE_PT,
    Russian     = ctru_sys::CFG_LANGUAGE_RU,
    TradChinese = ctru_sys::CFG_LANGUAGE_TW,
}

#[derive(Debug)]
#[repr(u32)]
pub enum SystemModel {
    Model3DS        = ctru_sys::CFG_MODEL_3DS,
    Model3DSXL      = ctru_sys::CFG_MODEL_3DSXL,
    ModelNew3DS     = ctru_sys::CFG_MODEL_N3DS,
    Model2DS        = ctru_sys::CFG_MODEL_2DS,
    ModelNew3DSXL   = ctru_sys::CFG_MODEL_N3DSXL,
    ModelNew2DSXL   = ctru_sys::CFG_MODEL_N2DSXL,
}

/// Represents the configuration service. No actions can be performed
/// until an instance of this struct is created.
///
/// The service exits when all instances of this struct go out of scope.
pub struct Cfgu(());

impl Cfgu {
    /// Initializes the CFGU service.
    ///
    /// # Errors
    ///
    /// This function will return Err if there was an error initializing the
    /// CFGU service.
    ///
    /// ctrulib services are reference counted, so this function may be called
    /// as many times as desired and the service will not exit until all
    /// instances of Fs drop out of scope.
    pub fn init() -> crate::Result<Cfgu> {
        unsafe {
            let r = ctru_sys::cfguInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Cfgu(()))
            }
        }
    }

    /// Gets system region from secure info
    pub fn get_region(&self) -> crate::Result<Region> {
        let mut region: u8 = 0;
        let region_pointer: *mut u8 = &mut region;

        unsafe {
            let r = ctru_sys::CFGU_SecureInfoGetRegion(region_pointer);
            if r < 0 {
                Err(r.into())
            } else {
                // The system shouldn't give an invalid value
                Ok(Region::try_from(region).unwrap())
            }
        }
    }

    /// Gets system's model
    pub fn get_model(&self) -> crate::Result<SystemModel> {
        let mut model: u8 = 0;
        let model_pointer: *mut u8 = &mut model;

        unsafe {
            let r = ctru_sys::CFGU_GetSystemModel(model_pointer);
            if r < 0 {
                Err(r.into())
            } else {
                // The system shouldn't give an invalid value
                Ok(SystemModel::try_from(model).unwrap())
            }
        }
    }

    /// Gets system's language
    pub fn get_language(&self) -> crate::Result<Language> {
        let mut language: u8 = 0;
        let language_pointer: *mut u8 = &mut language;

        unsafe {
            let r = ctru_sys::CFGU_GetSystemLanguage(language_pointer);
            if r < 0 {
                Err(r.into())
            } else {
                // The system shouldn't give an invalid value
                Ok(Language::try_from(language).unwrap())
            }
        }
    }

    /// Checks if NFC is supported by the console
    pub fn is_nfc_supported(&self) -> crate::Result<bool> {
        let mut supported: bool = false;
        let supported_pointer: *mut bool = &mut supported;

        unsafe {
            let r = ctru_sys::CFGU_IsNFCSupported(supported_pointer);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(supported)
            }
        }
    }

    /// Check if the console is from the 2DS family (2DS, New2DS, New2DSXL)
    pub fn is_2ds_family(&self) -> crate::Result<bool> {
        let mut is_2ds_family: u8 = 0;
        let is_2ds_family_pointer: *mut u8 = &mut is_2ds_family;

        unsafe {
            let r = ctru_sys::CFGU_GetModelNintendo2DS(is_2ds_family_pointer);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(is_2ds_family < 1)
            }
        }
    }
}

impl Drop for Cfgu {
    fn drop(&mut self) {
        unsafe {
            ctru_sys::cfguExit();
        }
    }
}

macro_rules! from_type_to_u8 {
    ($from_type:ty) => {
        impl From<$from_type> for u8 {
            fn from(v: $from_type) -> Self {
                v as u8
            }
        }
    }
}

from_type_to_u8!(Region);
from_type_to_u8!(Language);
from_type_to_u8!(SystemModel);

impl TryFrom<u8> for Region {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as u32 {
            ctru_sys::CFG_REGION_JPN => Ok(Region::Japan),
            ctru_sys::CFG_REGION_USA => Ok(Region::Usa),
            ctru_sys::CFG_REGION_EUR => Ok(Region::Europe),
            ctru_sys::CFG_REGION_AUS => Ok(Region::Australia),
            ctru_sys::CFG_REGION_CHN => Ok(Region::China),
            ctru_sys::CFG_REGION_KOR => Ok(Region::Korea),
            ctru_sys::CFG_REGION_TWN => Ok(Region::Taiwan),
            _ => Err(())
        }
    }
}

impl TryFrom<u8> for Language {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as u32 {
            ctru_sys::CFG_LANGUAGE_JP  => Ok(Language::Japan),
            ctru_sys::CFG_LANGUAGE_EN  => Ok(Language::English),
            ctru_sys::CFG_LANGUAGE_FR  => Ok(Language::French),
            ctru_sys::CFG_LANGUAGE_DE  => Ok(Language::German),
            ctru_sys::CFG_LANGUAGE_IT  => Ok(Language::Italian),
            ctru_sys::CFG_LANGUAGE_ES  => Ok(Language::Spanish),
            ctru_sys::CFG_LANGUAGE_ZH  => Ok(Language::SimpChinese),
            ctru_sys::CFG_LANGUAGE_KO  => Ok(Language::Korean),
            ctru_sys::CFG_LANGUAGE_NL  => Ok(Language::Dutch),
            ctru_sys::CFG_LANGUAGE_PT  => Ok(Language::Portuguese),
            ctru_sys::CFG_LANGUAGE_RU => Ok(Language::Russian),
            ctru_sys::CFG_LANGUAGE_TW => Ok(Language::TradChinese),
            _  => Err(())
        }
    }
}

impl TryFrom<u8> for SystemModel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as u32 {
            ctru_sys::CFG_MODEL_3DS => Ok(SystemModel::Model3DS),
            ctru_sys::CFG_MODEL_3DSXL => Ok(SystemModel::Model3DSXL),
            ctru_sys::CFG_MODEL_N3DS => Ok(SystemModel::ModelNew3DS),
            ctru_sys::CFG_MODEL_2DS => Ok(SystemModel::Model2DS),
            ctru_sys::CFG_MODEL_N3DSXL => Ok(SystemModel::ModelNew3DSXL),
            ctru_sys::CFG_MODEL_N2DSXL => Ok(SystemModel::ModelNew2DSXL),
            _ => Err(())
        }
    }
}