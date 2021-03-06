use super::locale::Locale;
use super::helpers::Helpers;

pub struct Address {
    building_number: Vec<&'static str>,
    city_prefix: Vec<&'static str>,
    city_suffix: Vec<&'static str>,
    street_suffix: Vec<&'static str>,
    secondary_address: Vec<&'static str>,
    first_names: Vec<&'static str>,
    last_names: Vec<&'static str>,
    state: Vec<&'static str>,
    zip: Vec<&'static str>,
    time_zone: Vec<&'static str>,
    state_abbr: Vec<&'static str>,
    country: Vec<&'static str>,
    helpers: Helpers
}


impl Address {

    pub fn new(locale: Locale) -> Address {
        Address {
            building_number: locale.building_number,
            city_prefix: locale.city_prefix,
            city_suffix: locale.city_suffix,
            street_suffix: locale.street_suffix,
            secondary_address: locale.secondary_address,
            first_names: locale.name_first,
            last_names: locale.name_last,
            state: locale.state,
            zip: locale.zip,
            time_zone: locale.time_zone,
            state_abbr: locale.state_abbr,
            country: locale.country,
            helpers: Helpers
        }
    }

    fn first_name(&self) -> String {
            self.helpers.array_element(&self.first_names).to_string()
    }

    fn last_name(&self)  -> String {
            self.helpers.array_element(&self.last_names).to_string()
    }

    pub fn city_prefix(&self)  -> String {
            self.helpers.array_element(&self.city_prefix).to_string()
    }

    pub fn city_suffix(&self)  -> String {
            self.helpers.sentence_case(self.helpers.array_element(&self.city_suffix).to_string())
    }

    fn city_suffix_lower(&self)  -> String {
            self.helpers.lowercase(self.city_suffix())
    }

    pub fn street_suffix(&self)  -> String {
            self.helpers.array_element(&self.street_suffix).to_string()
    }

    pub fn state(&self)  -> String {
            self.helpers.array_element(&self.state).to_string()
    }

    pub fn country(&self)  -> String {
            self.helpers.array_element(&self.country).to_string()
    }

    pub fn time_zone(&self)  -> String {
            self.helpers.array_element(&self.time_zone).to_string()
    }

    pub fn state_abbr(&self)  -> String {
            self.helpers.array_element(&self.state_abbr).to_string()
    }

    pub fn building_number(&self) -> String {
            let format = self.helpers.array_element(&self.building_number).to_string();
            self.helpers.replace_sym_with_number(format)
    }

    pub fn zip(&self) -> String {
            let format = self.helpers.array_element(&self.zip).to_string();
            self.helpers.replace_sym_with_number(format)
    }

    pub fn secondary_address(&self) -> String {
            let format = self.helpers.array_element(&self.secondary_address).to_string();
            self.helpers.replace_sym_with_number(format)
    }

    pub fn city(&self) -> String {
        match self.helpers.number_in_range::<i32>(0, 3) {
                0 => format!("{} {}{}", self.city_prefix(), self.first_name(), self.city_suffix_lower()),
                1 => format!("{} {}", self.city_prefix(), self.first_name()),
                2 => format!("{}{}", self.first_name(), self.city_suffix_lower()),
                _ => format!("{}{}", self.last_name(), self.city_suffix_lower())
        }
    }

    pub fn street_name(&self) -> String {
        match self.helpers.number_in_range::<i32>(0, 1) {
                0 => format!("{} {}", self.first_name(), self.street_suffix()),
                _ => format!("{} {}", self.last_name(), self.street_suffix())
        }
    }

    pub fn street_address(&self) -> String {
         format!("{} {}", self.building_number(), self.street_name())
    }

    pub fn latitude(&self) -> String {
        ((self.helpers.number::<f64>() * 180f64) - 90f64).to_string()
    }

    pub fn longitude(&self) -> String {
        ((self.helpers.number::<f64>() * 360f64) - 180f64).to_string()
    }

}
