use serde::{Deserialize, Serialize};

/// Representa una plataforma de videojuegos en el dominio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platform {
    /// El ID único de la plataforma (ej. de IGDB).
    pub id: i64,
    /// El nombre de la plataforma.
    pub name: String,
    /// La generación de la plataforma.
    pub generation: Option<i32>,
    /// El tipo de plataforma.
    #[serde(rename = "platform_type")]
    pub platform_type: PlatformType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i32)]
pub enum PlatformType {
    Unknown = 0,
    Console = 1,
    Arcade = 2,
    Platform = 3,
    OperatingSystem = 4,
    PortableConsole = 5,
    Computer = 6,
}

impl From<PlatformType> for i32 {
    fn from(platform_type: PlatformType) -> Self {
        platform_type as i32
    }
}

impl TryFrom<i32> for PlatformType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PlatformType::Console),
            2 => Ok(PlatformType::Arcade),
            3 => Ok(PlatformType::Platform),
            4 => Ok(PlatformType::OperatingSystem),
            5 => Ok(PlatformType::PortableConsole),
            6 => Ok(PlatformType::Computer),
            0 => Ok(PlatformType::Unknown),
            _ => Ok(PlatformType::Unknown), // Fallback to Unknown for unmapped values
        }
    }
}
