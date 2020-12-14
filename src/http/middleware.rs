use super::*;
use std::fmt::Debug;
use async_std::future::Future;


/// Middleware handle function
#[async_trait]
pub trait Middleware<State: Clone + Send + Sync + 'static, V: Debug> {
    /// 具体实现方法
    async fn handle(&self, ctx: Context<State, V>);
}

#[async_trait]
impl<State, F, Fut> Middleware<State,V> for F
    where
        State: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(Context<State, V>) -> Fut,
{
    async fn handle(&self, ctx: Context<State, V>) {
        let fut = (self)(ctx);
        fut.await?;
    }
}