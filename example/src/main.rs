use async_std::future::Future;

#[async_std::main]
async fn main() {
    println!("Hello, world!");
}

// async fn hello() -> fn(String) -> impl Future<Output=()> {
//     let str = String::from("hello world");
//     async move |a: String| {
//         println!("{}   {}",str,a);
//     }
// }

// async fn hello() -> impl Future<Output=()> {
//     let str = String::from("hello world");
//     async move |a: String| {
//         println!("{}   {}",str,a);
//     }
// }

