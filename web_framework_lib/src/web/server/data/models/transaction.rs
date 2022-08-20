use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::thread;
use crate::web::server::data::models::transaction::request::Request;
use crate::web::server::data::models::transaction::response::Response;

pub mod request;
pub mod response;

pub struct Transaction<'a> {
    req: Request,
    res: Response<'a>,
    resolved: bool
}

impl Debug for Transaction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction")
            .field("req", &self.req)
            .field("res", &self.res)
            .field("thread", &thread::current())
            .finish()
    }
}

#[allow(dead_code)]
impl <'a> Transaction<'a> {
    pub fn new(req: Request, res: Response<'a>) -> Self {
        Self { req, res, resolved: false }
    }

    pub fn resolve(&mut self) -> Result<(), &str> {
        return if self.resolved {
            Err("Transaction already resolved...")
        } else if self.res.status() == 0 {
            Err("An attempt was made to resolve \
            transaction without setting the status of the response.")
        } else {
            self.req.stream()
                .write(self.res.get_as_u8_vec()
                    .as_slice())
                .expect("Failed to resolve transaction.");
            self.set_resolved(true);
            Ok(())
        }
    }

    pub fn req(&self) -> &Request {
        &self.req
    }
    pub fn res(&self) -> &Response<'a> {
        &self.res
    }

    pub fn req_mut(&mut self) -> &mut Request {
        &mut self.req
    }

    pub fn res_mut(&mut self) -> &mut Response<'a> {
        &mut self.res
    }

    pub fn resolved(&self) -> bool {
        self.resolved
    }

    pub fn set_resolved(&mut self, resolved: bool) {
        self.resolved = resolved;
    }
}