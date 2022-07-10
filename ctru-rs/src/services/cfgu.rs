//! Configuration service
//!
//! This module contains basic methods to retrieve and change configuration from the console.

#[derive(Debug)]
#[repr(u8)]
pub enum Region {
    Japan       = 0,
    Usa         = 1,
    Europe      = 2,
    Australia   = 3,
    China       = 4,
    Korea       = 5,
    Taiwan      = 6,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Language {
    Japan       = 0,
    English     = 1,
    French      = 2,
    German      = 3,
    Italian     = 4,
    Spanish     = 5,
    SimpChinese = 6,
    Korean      = 7,
    Dutch       = 8,
    Portuguese  = 9,
    Russian     = 10,
    TradChinese = 11,
}

#[derive(Debug)]
#[repr(u8)]
pub enum SystemModel {
    Model3DS        = 0,
    Model3DSXL      = 1,
    ModelNew3DS     = 2,
    Model2DS        = 3,
    ModelNew3DSXL   = 4,
    ModelNew2DSXL   = 5,
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

    /// Check if the console is 2DS-like (2DS, New2DS, New2DSXL)
    pub fn is_2ds_like(&self) -> crate::Result<bool> {
        let mut is_2ds_like: u8 = 0;
        let is_2ds_like_pointer: *mut u8 = &mut is_2ds_like;

        unsafe {
            let r = ctru_sys::CFGU_GetModelNintendo2DS(is_2ds_like_pointer);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(is_2ds_like < 1)
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
        match value {
            0 => Ok(Region::Japan),
            1 => Ok(Region::Usa),
            2 => Ok(Region::Europe),
            3 => Ok(Region::Australia),
            4 => Ok(Region::China),
            5 => Ok(Region::Korea),
            6 => Ok(Region::Taiwan),
            _ => Err(())
        }
    }
}

impl TryFrom<u8> for Language {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0  => Ok(Language::Japan),
            1  => Ok(Language::English),
            2  => Ok(Language::French),
            3  => Ok(Language::German),
            4  => Ok(Language::Italian),
            5  => Ok(Language::Spanish),
            6  => Ok(Language::SimpChinese),
            7  => Ok(Language::Korean),
            8  => Ok(Language::Dutch),
            9  => Ok(Language::Portuguese),
            10 => Ok(Language::Russian),
            11 => Ok(Language::TradChinese),
            _  => Err(())
        }
    }
}

impl TryFrom<u8> for SystemModel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SystemModel::Model3DS),
            1 => Ok(SystemModel::Model3DSXL),
            2 => Ok(SystemModel::ModelNew3DS),
            3 => Ok(SystemModel::Model2DS),
            4 => Ok(SystemModel::ModelNew3DSXL),
            5 => Ok(SystemModel::ModelNew2DSXL),
            _ => Err(())
        }
    }
}