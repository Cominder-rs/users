use tonic::body::BoxBody;
use hyper::Body;
use std::task::{Context, Poll};
use std::sync::{Mutex, Arc};

pub type IpDB = Arc<Mutex<DB>>;
use tower::{Service, Layer};

use ip2location::DB;

const IPV4BIN: &str = "services/users/assets/IP2LOCATION-LITE-DB1.BIN";

#[derive(Debug, Clone, Default)]
pub struct AppDataMiddlewareLayer;

impl <S> Layer<S> for AppDataMiddlewareLayer {
    type Service = AppDataMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        let ip_db = DB::from_file(IPV4BIN).expect("Openin ip adresses DB file");
        let ip_db = Arc::new(Mutex::new(ip_db));

        let app_data = AppData {
            ip_db
        };
        AppDataMiddleware { inner: service, app_data }
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
