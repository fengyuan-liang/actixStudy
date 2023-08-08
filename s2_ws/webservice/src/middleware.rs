use std::pin::Pin;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::http::{HeaderName, HeaderValue};
use uuid::Uuid;

// 定义中间件 trait
trait TraceMiddleware: Clone {
    fn apply(&self, req: ServiceRequest, srv: &dyn Service<Request=ServiceRequest, Response=ServiceResponse, Error=actix_web::Error, Future=()>) -> Pin<Box<dyn futures::Future<Output=Result<ServiceResponse, actix_web::Error>>>>;
}

// 定义中间件类型
#[derive(Clone)]
pub struct TraceMiddlewareImpl;

impl TraceMiddleware for TraceMiddlewareImpl {
    fn apply(&self, req: ServiceRequest, srv: &dyn Service<Request=ServiceRequest, Response=ServiceResponse, Error=actix_web::Error, Future=()>) -> Pin<Box<dyn futures::Future<Output=Result<ServiceResponse, actix_web::Error>>>> {
        Box::pin(trace_id_middleware(req, srv))
    }
}

// trace
async fn trace_id_middleware<S>(
    req: ServiceRequest,
    srv: &mut dyn Service<Request=ServiceRequest, Response=ServiceResponse, Error=Error, Future=()>,
) -> Result<ServiceResponse, actix_web::Error>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse, Error=actix_web::Error>,
{
    // 生成 Trace ID
    let trace_id = Uuid::new_v4().to_string();

    let mut req = req;
    req.headers_mut().insert(
        HeaderName::from_static("traceId"),
        HeaderValue::from_str(&trace_id).expect("Failed to create HeaderValue"),
    );

    // 继续处理请求
    let res = srv.call(req).await?;
    Ok(res)
}