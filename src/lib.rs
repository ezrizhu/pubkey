use worker::{event, Context, Env, Response, Request, Result};

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let pubkeys = include_str!("keys");
    Response::ok(pubkeys)
}
