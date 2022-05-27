#[macro_export(define_callback)]
macro_rules! define_callback {
  ($(#[$attribute:meta])* pub struct $name:ident, $cbtype:expr => $offset:literal; $($rest:tt)*) => {
    $(#[$attribute])*
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct $name {
      $($rest)*
    }
    impl crate::steam_api::Callback for $name {
      fn get_type(&self) -> CallbackType { $cbtype as CallbackType + $offset }
    }
  };
}