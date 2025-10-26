// Define type for tuple containing handler information.
// (HTTP Method, Path, Handler Function)

pub type HandlerInfo = (String, String, fn(&crate::http::request::Request) -> crate::http::response::Response);
