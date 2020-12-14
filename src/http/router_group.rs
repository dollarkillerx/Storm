use super::*;
use tide::Server;
use async_std::sync::Arc;


/// tide 交互等和核心
struct Engine<'a> {
    /// tide core
    tide: &'a Arc<Server<()>>,
}


/// 路由组
pub struct RouterGroup<'a> {
    /// 当前路由path
    router_path: String,
    /// 多路由具体绑定
    engine: &'a Arc<Engine<'a>>,
}