use Result;


extern "C" {
    pub fn sdmcInit() -> Result;
    pub fn sdmcExit() -> Result;
}
