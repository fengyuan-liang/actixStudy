use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct R<T> {
    pub code: usize,
    pub data: T,
}

impl<T> R<T> {
    pub fn ok(data: T) -> R<T> {
        R {
            code: 200,
            data,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    pub async fn test_result() {
        let r = R::ok("hello");
        println!("{:?}", r)
    }
}