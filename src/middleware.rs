use std::{future::Future, pin::Pin, rc::Rc};
use actix_web::HttpResponse;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_utils::future::{ready, Ready};
use actix_identity::IdentityExt;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
  S::Future: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = actix_web::Error;
  type Transform = InnerAuthMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(InnerAuthMiddleware {
      service: Rc::new(service),
    }))
  }
}
pub struct InnerAuthMiddleware<S>{
  service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for InnerAuthMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
  S::Future: 'static,
{
  type Response = S::Response;
  type Error = actix_web::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let service = Rc::clone(&self.service);

    Box::pin(async move {
      let mut unauthorized = true;
      let path = req.path().to_string();
      let id_wrapped = req.get_identity();

      if path == "/login" || path == "/" {
        unauthorized = false;
      }
      else if let Ok(id) = id_wrapped {
        if id.id().is_ok(){
          unauthorized = false;
        }
      }

      if !unauthorized {
        let res = service.call(req).await?;
        return Ok(res);
      }

      Err(
        actix_web::error::InternalError::from_response(
          "Unauthorized",
          HttpResponse::Unauthorized().body("Unauthorized"),
        )
        .into()
      )
    })
  }
}
