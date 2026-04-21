use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Country {
    pub code: &'static str,
    pub name: &'static str,
    pub flag: &'static str,
}

impl Country {
    pub fn display(&self) -> String {
        format!("{}  {}", self.flag, self.name)
    }
}

pub const COUNTRIES: &[Country] = &[
    Country { code: "AR", name: "Argentina",       flag: "🇦🇷" },
    Country { code: "AT", name: "Austria",         flag: "🇦🇹" },
    Country { code: "AU", name: "Australia",       flag: "🇦🇺" },
    Country { code: "BE", name: "Belgium",         flag: "🇧🇪" },
    Country { code: "BG", name: "Bulgaria",        flag: "🇧🇬" },
    Country { code: "BR", name: "Brazil",          flag: "🇧🇷" },
    Country { code: "CA", name: "Canada",          flag: "🇨🇦" },
    Country { code: "CH", name: "Switzerland",     flag: "🇨🇭" },
    Country { code: "CL", name: "Chile",           flag: "🇨🇱" },
    Country { code: "CO", name: "Colombia",        flag: "🇨🇴" },
    Country { code: "CZ", name: "Czech Republic",  flag: "🇨🇿" },
    Country { code: "DE", name: "Germany",         flag: "🇩🇪" },
    Country { code: "DK", name: "Denmark",         flag: "🇩🇰" },
    Country { code: "EE", name: "Estonia",         flag: "🇪🇪" },
    Country { code: "ES", name: "Spain",           flag: "🇪🇸" },
    Country { code: "FI", name: "Finland",         flag: "🇫🇮" },
    Country { code: "FR", name: "France",          flag: "🇫🇷" },
    Country { code: "GB", name: "United Kingdom",  flag: "🇬🇧" },
    Country { code: "GR", name: "Greece",          flag: "🇬🇷" },
    Country { code: "HK", name: "Hong Kong",       flag: "🇭🇰" },
    Country { code: "HR", name: "Croatia",         flag: "🇭🇷" },
    Country { code: "HU", name: "Hungary",         flag: "🇭🇺" },
    Country { code: "ID", name: "Indonesia",       flag: "🇮🇩" },
    Country { code: "IE", name: "Ireland",         flag: "🇮🇪" },
    Country { code: "IL", name: "Israel",          flag: "🇮🇱" },
    Country { code: "IN", name: "India",           flag: "🇮🇳" },
    Country { code: "IS", name: "Iceland",         flag: "🇮🇸" },
    Country { code: "IT", name: "Italy",           flag: "🇮🇹" },
    Country { code: "JP", name: "Japan",           flag: "🇯🇵" },
    Country { code: "KR", name: "South Korea",     flag: "🇰🇷" },
    Country { code: "LT", name: "Lithuania",       flag: "🇱🇹" },
    Country { code: "LU", name: "Luxembourg",      flag: "🇱🇺" },
    Country { code: "LV", name: "Latvia",          flag: "🇱🇻" },
    Country { code: "MD", name: "Moldova",         flag: "🇲🇩" },
    Country { code: "MX", name: "Mexico",          flag: "🇲🇽" },
    Country { code: "MY", name: "Malaysia",        flag: "🇲🇾" },
    Country { code: "NL", name: "Netherlands",     flag: "🇳🇱" },
    Country { code: "NO", name: "Norway",          flag: "🇳🇴" },
    Country { code: "NZ", name: "New Zealand",     flag: "🇳🇿" },
    Country { code: "PE", name: "Peru",            flag: "🇵🇪" },
    Country { code: "PH", name: "Philippines",     flag: "🇵🇭" },
    Country { code: "PL", name: "Poland",          flag: "🇵🇱" },
    Country { code: "PT", name: "Portugal",        flag: "🇵🇹" },
    Country { code: "RO", name: "Romania",         flag: "🇷🇴" },
    Country { code: "RS", name: "Serbia",          flag: "🇷🇸" },
    Country { code: "SE", name: "Sweden",          flag: "🇸🇪" },
    Country { code: "SG", name: "Singapore",       flag: "🇸🇬" },
    Country { code: "SI", name: "Slovenia",        flag: "🇸🇮" },
    Country { code: "SK", name: "Slovakia",        flag: "🇸🇰" },
    Country { code: "TH", name: "Thailand",        flag: "🇹🇭" },
    Country { code: "TR", name: "Turkey",          flag: "🇹🇷" },
    Country { code: "TW", name: "Taiwan",          flag: "🇹🇼" },
    Country { code: "UA", name: "Ukraine",         flag: "🇺🇦" },
    Country { code: "US", name: "United States",   flag: "🇺🇸" },
    Country { code: "UY", name: "Uruguay",         flag: "🇺🇾" },
    Country { code: "VN", name: "Vietnam",         flag: "🇻🇳" },
    Country { code: "ZA", name: "South Africa",    flag: "🇿🇦" },
];

pub fn find_by_code(code: &str) -> Option<&'static Country> {
    let upper = code.to_uppercase();
    COUNTRIES.iter().find(|c| c.code == upper)
}
