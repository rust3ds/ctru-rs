use super::super::Result;

#[link(name = "ctru")]
extern "C" {
    pub fn sdmcInit() -> Result;
    pub fn sdmcExit() -> Result;
}
