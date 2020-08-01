use crate::error;
use actix_identity::Identity;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use futures::future::Future;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;

use crate::firestore::v1::Document;
use crate::firestore::FirestoreV1Client;

#[derive(Deserialize, Serialize, Clone)]
pub struct Participant {
  pub id: String,
}

impl From<Document> for Participant {
  fn from(document: Document) -> Self {
    Participant {
      id: document
        .name
        .rsplitn(2, '/')
        .next()
        .expect("document id")
        .into(),
    }
  }
}

impl FromRequest for Participant {
  type Error = error::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self, error::Error>>>>;
  type Config = ();

  fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
    let firestore = req
      .app_data::<Data<FirestoreV1Client>>()
      .expect("firestore client");
    let firestore = &(*Arc::clone(&firestore.clone().into_inner()));
    Box::pin(super::new(
      firestore.clone(),
      Identity::from_request(req, payload),
    ))
  }
}
