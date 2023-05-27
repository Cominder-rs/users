use civilization::common_structs::Env;
use sea_orm::DatabaseConnection;
use tonic::body::BoxBody;
use hyper::Body;
use std::task::{Context, Poll};
use std::sync::Arc;
use parking_lot::Mutex;

use tower::{Service, Layer};
use ip2location::DB;


pub type IpDB = Arc<Mutex<DB>>;

#[derive(Debug, Clone)]
pub struct AppDataMiddlewareLayer {
    pub env: Env,
    pub ip_db: IpDB,
    pub db: DatabaseConnection
}

impl <S> Layer<S> for AppDataMiddlewareLayer {
    type Service = AppDataMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        AppDataMiddleware { inner: service, app_data: AppData { ip_db: self.ip_db.clone(), db: self.db.clone() } }
    }
}


#[derive(Clone)]
pub struct AppDataMiddleware<S> {
    inner: S,
    app_data: AppData,
}

#[derive(Clone)]
pub struct AppData {
    pub ip_db: IpDB,
    pub db: DatabaseConnection
}

impl <S> Service<hyper::Request<Body>> for AppDataMiddleware<S>
where 
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<Body>) -> Self::Future {
        let new_uri = req.uri().to_string();
        let new_uri = new_uri.strip_prefix("/api/users").unwrap();
        *req.uri_mut() = new_uri.parse().unwrap();
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let app_data = self.app_data.clone();

        req.extensions_mut().insert(app_data);

        Box::pin(async move {
            // Do extra async work here...
            let response = inner.call(req).await?;

            Ok(response)
        })
    }    
}
