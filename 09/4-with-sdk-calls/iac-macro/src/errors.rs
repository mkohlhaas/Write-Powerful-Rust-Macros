use aws_sdk_lambda::error::SdkError;
use aws_sdk_lambda::operation::add_permission::AddPermissionError;
use aws_sdk_lambda::operation::create_function::CreateFunctionError;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::operation::create_bucket::CreateBucketError;
use aws_sdk_s3::operation::put_bucket_notification_configuration::PutBucketNotificationConfigurationError;
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum IacError {
  Bucket(String),
  Lambda(String),
  Event(String),
}

impl IacError {
  pub fn into_compile_error(self) -> TokenStream {
    use IacError::*;

    match self {
      Bucket(message) => syn::Error::new(
        Span::call_site(),
        format!("bucket could not be created: {}", message),
      )
      .into_compile_error()
      .into(),

      Lambda(message) => syn::Error::new(
        Span::call_site(),
        format!("lambda could not be created: {}", message),
      )
      .into_compile_error()
      .into(),

      Event(message) => syn::Error::new(
        Span::call_site(),
        format!(
          "event to link bucket and lambda could not be created: {}",
          message
        ),
      )
      .into_compile_error()
      .into(),
    }
  }
}

impl Error for IacError {}

impl Display for IacError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("retrieval error")
  }
}

macro_rules! generate_from_error {
  // path would also work for $mine
  ($mine:expr, $aws:ty) => {
    impl From<SdkError<$aws>> for IacError {
      fn from(value: SdkError<$aws>) -> Self {
        let message = value
          .message()
          .map(|v| v.to_string())
          .unwrap_or_else(|| "no message".to_string());
        $mine(message)
      }
    }
  };
}

generate_from_error!(IacError::Bucket, CreateBucketError);
generate_from_error!(IacError::Lambda, CreateFunctionError);
generate_from_error!(IacError::Event, PutBucketNotificationConfigurationError);
generate_from_error!(IacError::Event, AddPermissionError);
