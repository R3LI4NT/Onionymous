use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExitAvailability {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Country {
    pub code: &'static str,
    pub name: &'static str,
    pub flag: &'static str,
    pub availability: ExitAvailability,
}

impl Country {
    pub fn display(&self) -> String {
        let marker = match self.availability {
            ExitAvailability::High => "★",
            ExitAvailability::Medium => " ",
            ExitAvailability::Low => "⚠",
        };
        format!("{} {}  {}", marker, self.flag, self.name)
    }
}

pub const COUNTRIES: &[Country] = &[
    Country { code: "AR", name: "Argentina",       flag: "🇦🇷", availability: ExitAvailability::Low },
    Country { code: "AT", name: "Austria",         flag: "🇦🇹", availability: ExitAvailability::High },
    Country { code: "AU", name: "Australia",       flag: "🇦🇺", availability: ExitAvailability::Medium },
    Country { code: "BE", name: "Belgium",         flag: "🇧🇪", availability: ExitAvailability::Medium },
    Country { code: "BG", name: "Bulgaria",        flag: "🇧🇬", availability: ExitAvailability::Medium },
    Country { code: "BR", name: "Brazil",          flag: "🇧🇷", availability: ExitAvailability::Low },
    Country { code: "CA", name: "Canada",          flag: "🇨🇦", availability: ExitAvailability::High },
    Country { code: "CH", name: "Switzerland",     flag: "🇨🇭", availability: ExitAvailability::High },
    Country { code: "CL", name: "Chile",           flag: "🇨🇱", availability: ExitAvailability::Low },
    Country { code: "CO", name: "Colombia",        flag: "🇨🇴", availability: ExitAvailability::Low },
    Country { code: "CZ", name: "Czech Republic",  flag: "🇨🇿", availability: ExitAvailability::Medium },
    Country { code: "DE", name: "Germany",         flag: "🇩🇪", availability: ExitAvailability::High },
    Country { code: "DK", name: "Denmark",         flag: "🇩🇰", availability: ExitAvailability::Medium },
    Country { code: "EE", name: "Estonia",         flag: "🇪🇪", availability: ExitAvailability::Medium },
    Country { code: "ES", name: "Spain",           flag: "🇪🇸", availability: ExitAvailability::Medium },
    Country { code: "FI", name: "Finland",         flag: "🇫🇮", availability: ExitAvailability::High },
    Country { code: "FR", name: "France",          flag: "🇫🇷", availability: ExitAvailability::High },
    Country { code: "GB", name: "United Kingdom",  flag: "🇬🇧", availability: ExitAvailability::High },
    Country { code: "GR", name: "Greece",          flag: "🇬🇷", availability: ExitAvailability::Low },
    Country { code: "HK", name: "Hong Kong",       flag: "🇭🇰", availability: ExitAvailability::Low },
    Country { code: "HR", name: "Croatia",         flag: "🇭🇷", availability: ExitAvailability::Low },
    Country { code: "HU", name: "Hungary",         flag: "🇭🇺", availability: ExitAvailability::Medium },
    Country { code: "ID", name: "Indonesia",       flag: "🇮🇩", availability: ExitAvailability::Low },
    Country { code: "IE", name: "Ireland",         flag: "🇮🇪", availability: ExitAvailability::Medium },
    Country { code: "IL", name: "Israel",          flag: "🇮🇱", availability: ExitAvailability::Low },
    Country { code: "IN", name: "India",           flag: "🇮🇳", availability: ExitAvailability::Low },
    Country { code: "IS", name: "Iceland",         flag: "🇮🇸", availability: ExitAvailability::Low },
    Country { code: "IT", name: "Italy",           flag: "🇮🇹", availability: ExitAvailability::Medium },
    Country { code: "JP", name: "Japan",           flag: "🇯🇵", availability: ExitAvailability::Low },
    Country { code: "KR", name: "South Korea",     flag: "🇰🇷", availability: ExitAvailability::Low },
    Country { code: "LT", name: "Lithuania",       flag: "🇱🇹", availability: ExitAvailability::Medium },
    Country { code: "LU", name: "Luxembourg",      flag: "🇱🇺", availability: ExitAvailability::Medium },
    Country { code: "LV", name: "Latvia",          flag: "🇱🇻", availability: ExitAvailability::Medium },
    Country { code: "MD", name: "Moldova",         flag: "🇲🇩", availability: ExitAvailability::Medium },
    Country { code: "MX", name: "Mexico",          flag: "🇲🇽", availability: ExitAvailability::Low },
    Country { code: "MY", name: "Malaysia",        flag: "🇲🇾", availability: ExitAvailability::Low },
    Country { code: "NL", name: "Netherlands",     flag: "🇳🇱", availability: ExitAvailability::High },
    Country { code: "NO", name: "Norway",          flag: "🇳🇴", availability: ExitAvailability::Medium },
    Country { code: "NZ", name: "New Zealand",     flag: "🇳🇿", availability: ExitAvailability::Low },
    Country { code: "PE", name: "Peru",            flag: "🇵🇪", availability: ExitAvailability::Low },
    Country { code: "PH", name: "Philippines",     flag: "🇵🇭", availability: ExitAvailability::Low },
    Country { code: "PL", name: "Poland",          flag: "🇵🇱", availability: ExitAvailability::Medium },
    Country { code: "PT", name: "Portugal",        flag: "🇵🇹", availability: ExitAvailability::Low },
    Country { code: "RO", name: "Romania",         flag: "🇷🇴", availability: ExitAvailability::High },
    Country { code: "RS", name: "Serbia",          flag: "🇷🇸", availability: ExitAvailability::Medium },
    Country { code: "SE", name: "Sweden",          flag: "🇸🇪", availability: ExitAvailability::High },
    Country { code: "SG", name: "Singapore",       flag: "🇸🇬", availability: ExitAvailability::Low },
    Country { code: "SI", name: "Slovenia",        flag: "🇸🇮", availability: ExitAvailability::Medium },
    Country { code: "SK", name: "Slovakia",        flag: "🇸🇰", availability: ExitAvailability::Medium },
    Country { code: "TH", name: "Thailand",        flag: "🇹🇭", availability: ExitAvailability::Low },
    Country { code: "TR", name: "Turkey",          flag: "🇹🇷", availability: ExitAvailability::Low },
    Country { code: "TW", name: "Taiwan",          flag: "🇹🇼", availability: ExitAvailability::Low },
    Country { code: "UA", name: "Ukraine",         flag: "🇺🇦", availability: ExitAvailability::Medium },
    Country { code: "US", name: "United States",   flag: "🇺🇸", availability: ExitAvailability::High },
    Country { code: "UY", name: "Uruguay",         flag: "🇺🇾", availability: ExitAvailability::Low },
    Country { code: "VN", name: "Vietnam",         flag: "🇻🇳", availability: ExitAvailability::Low },
    Country { code: "ZA", name: "South Africa",    flag: "🇿🇦", availability: ExitAvailability::Low },
];

pub fn find_by_code(code: &str) -> Option<&'static Country> {
    let upper = code.to_uppercase();
    COUNTRIES.iter().find(|c| c.code == upper)
}
