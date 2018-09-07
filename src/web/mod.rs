use rocket::{response, request};

pub mod guard;

pub struct CachedFile(pub response::NamedFile);

impl<'r> response::Responder<'r> for CachedFile {
    fn respond_to(self, req: &request::Request) -> response::Result<'r> {
        response::Response::build_from(self.0.respond_to(req)?)
            .raw_header("Cache-control", "max-age=86400") //  24h (24*60*60)
            .ok()
    }
}
