use super::*;
use std::collections::HashMap;
use std::fmt::Debug;


/// Context 贯穿整个请求的生命周期
#[derive(Debug)]
pub struct Context<State: Clone + Send + Sync + 'static> {
    /// tide 的Request
    req: Request<State>,
    /// 一次请求的上下文所需的存放
    /// 考虑 go的interface 可以任意传入 rust 怎么做???
    params: HashMap<String, String>,
    /// 当前 context 所在层级
    idx: u32,
}