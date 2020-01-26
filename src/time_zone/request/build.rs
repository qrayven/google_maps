use crate::time_zone::request::Request;
use std::convert::TryFrom;

impl Request {

    /// Builds the query string for the Google Maps Time Zone API based on the
    /// input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> &mut Request {

        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = format!(
            "key={}&location={}&timestamp={}",
            String::try_from(&self.location).unwrap(),
            self.time.timestamp(),
            self.key
        );

        // This section builds the "optional parameters" portion of the query
        // string:

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language))
        }

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        self

    } // fn

} // impl