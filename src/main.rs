use tide::{
    Request,
    prelude::*,
    Endpoint,

};

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    // let core = Core {};
    // app.at("/orders/shoes").post(p.order_shoes);
    app.at("/orders/shoes").post(Core::order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

struct Core {}

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
}

// 1. HTTP Group控制 组件  2. CORE组件
