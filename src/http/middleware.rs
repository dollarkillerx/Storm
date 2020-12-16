use super::*;
use std::fmt::Debug;
use async_std::future::Future;


/// Middleware handle function
#[async_trait]
pub trait Middleware<State: Clone + Send + Sync + 'static> {
    /// 具体实现方法
    async fn handle(&self, ctx: Context<State>);
}

#[async_trait]
impl<State, F> Middleware<State> for F
    where
        State: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(Context<State>),
{
    async fn handle(&self, ctx: Context<State>) {
        let fut = (self)(ctx);
        fut.await;
    }
}