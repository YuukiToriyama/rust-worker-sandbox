use worker::{event, Context, Env, Request, Response, Result};

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::ok("Hello, World!")
}
