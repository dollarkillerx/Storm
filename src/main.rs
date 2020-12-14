use tide::{
    Request,
    prelude::*,
    Endpoint,
    Response,
    Result,
};

use storm::*;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    let core = Core {};
    app.at("/orders/shoes").get(core);
    app.at("/orders/shoes2").get(Core::order_shoes);
    app.listen("127.0.0.1:8181").await?;
    Ok(())
}

struct Core {}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for Core {
    async fn call(&self, req: Request<State>) -> Result<Response> {
        Ok(format!("Hello World").into())
    }
}

impl Core {
    async fn order_shoes(mut req: Request<()>) -> tide::Result {
        let Animal { name, legs } = req.body_json().await?;
        Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
    }

    // async fn order_x(&self) -> fn(Request<()>) -> tide::Result {
    //     async move |mut req: Request<()>| -> tide::Result {
    //         let Animal { name, legs } = req.body_json().await?;
    //         Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
    //     }
    // }

    // async fn order_shoes_test2(&self) -> Box<dyn Future<Output=tide::Result>> {
    //     // 我想用self的一些变量
    //     Box::new(async move |mut req: Request<()>| -> tide::Result {
    //         let Animal { name, legs } = req.body_json().await?;
    //         Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
    //     })
    // }
}

// 1. HTTP Group控制 组件  2. CORE组件
