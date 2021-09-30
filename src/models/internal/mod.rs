pub mod city_internal;
pub mod country_internal;

static NOT_AVAILABLE_MESSAGE: &str = "n/a";

/// Internal helper trait to get "en" name.
trait DefaultName {
    fn get_default_name(&self) -> &str;
}
