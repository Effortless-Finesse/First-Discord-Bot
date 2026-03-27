use twilight_http::Client as HttpClient;
use twilight_model::id::Id;
use twilight_model::id::marker::ApplicationMarker;

pub struct Context {
    pub http: HttpClient,
    pub application_id: Id<ApplicationMarker>,
}

impl Context {
    pub fn new(token: String, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            http: HttpClient::new(token),
            application_id,
        }
    }
}
