use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::thread;

use crate::web::models::transaction::request::Request;
use crate::web::models::transaction::response::Response;

pub mod request;
pub mod response;

//  █     █░▓█████   ▄████  ▄▄▄▄    ██▓     ▄▄▄      ▓█████▄
// ▓█░ █ ░█░▓█   ▀  ██▒ ▀█▒▓█████▄ ▓██▒    ▒████▄    ▒██▀ ██▌
// ▒█░ █ ░█ ▒███   ▒██░▄▄▄░▒██▒ ▄██▒██░    ▒██  ▀█▄  ░██   █▌
// ░█░ █ ░█ ▒▓█  ▄ ░▓█  ██▓▒██░█▀  ▒██░    ░██▄▄▄▄██ ░▓█▄   ▌
// ░░██▒██▓ ░▒████▒░▒▓███▀▒░▓█  ▀█▓░██████▒ ▓█   ▓██▒░▒████▓
// ░ ▓░▒ ▒  ░░ ▒░ ░ ░▒   ▒ ░▒▓███▀▒░ ▒░▓  ░ ▒▒   ▓▒█░ ▒▒▓  ▒
//   ▒ ░ ░   ░ ░  ░  ░   ░ ▒░▒   ░ ░ ░ ▒  ░  ▒   ▒▒ ░ ░ ▒  ▒
//   ░   ░     ░   ░ ░   ░  ░    ░   ░ ░     ░   ▒    ░ ░  ░
//     ░       ░  ░      ░  ░          ░  ░      ░  ░   ░
//                               ░                    ░

/// `Transaction` is a struct that contains a `Request`, a `Response`, and a `bool` that indicates
/// whether the transaction has been resolved.
///
/// Properties:
///
/// * `req`: The request object.
/// * `res`: The response object that will be sent back to the client.
/// * `resolved`: This is a boolean that indicates whether the transaction has been resolved.
pub struct Transaction
// <'a>
{
    req: Request,
    // res: Response<'a>,
    resolved: bool
}

impl Debug for Transaction
// <'_>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction")
            .field("req", &self.req)
            // .field("res", &self.res)
            .field("thread", &thread::current())
            .finish()
    }
}

#[allow(dead_code)]
impl <'a> Transaction
// <'a>
{
    /// `new` is a function that takes two arguments, `req` and `res`, and returns a `Response` object
    ///
    /// Arguments:
    ///
    /// * `req`: The request object.
    /// * `res`: The response object that will be sent back to the client.
    ///
    /// Returns:
    ///
    /// A new instance of the `Resolver` struct.
    pub fn new(req: Request,
               // res: Response<'a>
    ) -> Self {
        Self { req,
            // res,
            resolved: false
        }
    }

    /// If the transaction is not resolved, and the response status is not 0, then write the response to
    /// the stream and set the transaction as resolved
    ///
    /// Returns:
    ///
    /// A Result<(), &str>
    pub fn resolve(&self, mut res: Response) -> Result<(), &str> {
        return if self.resolved {
            Err("Transaction already resolved...")
        } else if res.status() == 0 {
            Err("You have to set the http status before resolving the transaction.")
        } else {
            self.req.stream()
                .write(res.get_as_u8_vec()
                    .as_slice())
                .expect("Failed to resolve transaction.");
            // self.set_resolved(true);
            Ok(())
        }
    }

    pub fn req(&self) -> &Request {
        &self.req
    }
    // pub fn res(&self) -> &Response<'a> {
    //     &self.res
    // }

    pub fn req_mut(&mut self) -> &mut Request {
        &mut self.req
    }

    // pub fn res_mut(&mut self) -> &mut Response<'a> {
    //     &mut self.res
    // }

    pub fn resolved(&self) -> bool {
        self.resolved
    }

    pub fn set_resolved(&mut self, resolved: bool) {
        self.resolved = resolved;
    }
}