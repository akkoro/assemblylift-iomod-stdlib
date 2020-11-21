#[macro_export]
macro_rules! s3 {
    ($call:ident, $output:ty) => {
        fn $call(_input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
            Box::pin(async move {
                use rusoto_s3::*;
                $crate::__s3!($call, $output)
            })
        }
    };
    ($call:ident, $input:ty => $output:ty) => {
        fn $call(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
            Box::pin(async move {
                use rusoto_s3::*;
                let deserialized: $input = serde_json::from_slice(input.as_slice()).unwrap();
                $crate::__s3!($call, deserialized, $output)
            })
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __s3 {
    ($call:ident, $output:ty) => {
        match S3.$call().await {
            Ok(result) => {
                serde_json::to_vec(&Result::<$output, guest::Error>::Ok(result)).unwrap()
            }
            Err(err) => {
                serde_json::to_vec(&Result::<$output, guest::Error>::Err(guest::Error {
                    why: err.to_string(),
                }))
                .unwrap()
            }
        }
    };
    ($call:ident, $deserialized:ident, $output:ty) => {
        match S3.$call($deserialized).await {
            Ok(result) => {
                serde_json::to_vec(&Result::<$output, guest::Error>::Ok(result)).unwrap()
            }
            Err(err) => {
                serde_json::to_vec(&Result::<$output, guest::Error>::Err(guest::Error {
                    why: err.to_string(),
                }))
                .unwrap()
            }
        }
    };
}
