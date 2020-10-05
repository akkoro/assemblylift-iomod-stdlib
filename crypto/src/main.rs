use assemblylift_core_iomod::iomod;
use futures::future::BoxFuture;
use tokio;
use uuid::Uuid;

use serde_json::to_vec;

#[tokio::main]
async fn main() {
    iomod!(akkoro.std.crypto => {
        uuid4 => uuid4
    });
}

fn uuid4(_input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    Box::pin(async move {
        let buffer = &mut Uuid::encode_buffer();
        let id = Uuid::new_v4().to_hyphenated().encode_lower(buffer);
        to_vec(&String::from(id)).unwrap()
    })
}
