use iron::prelude::*;
use iron::status;

pub fn index(req: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "This is default response!")))
}
