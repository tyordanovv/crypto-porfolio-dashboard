#[derive(Debug, Clone)]
pub enum M2Country {
    US,
    EU,
    Japan,
    Canada,
    China,
    India,
    UK,
}

impl M2Country {
    pub fn as_fxempire_symbol(&self) -> &'static str {
        match self {
            M2Country::US => "united-states",
            M2Country::EU => "euro-area",
            M2Country::Japan => "japan",
            M2Country::Canada => "canada",
            M2Country::China => "china",
            M2Country::India => "india",
            M2Country::UK => "united-kingdom",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            M2Country::US => "United States",
            M2Country::EU => "Euro Area",
            M2Country::Japan => "Japan",
            M2Country::Canada => "Canada",
            M2Country::China => "China",
            M2Country::India => "India",
            M2Country::UK => "United Kingdom",
        }
    }

    pub fn as_currency_code(&self) -> &'static str {
        match self {
            M2Country::US => "USD",
            M2Country::EU => "EUR",
            M2Country::Japan => "JPY",
            M2Country::Canada => "CAD",
            M2Country::China => "CNY",
            M2Country::India => "INR",
            M2Country::UK => "GBP",
        }
    }
}