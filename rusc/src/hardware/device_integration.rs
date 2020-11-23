pub trait DeviceIntegration<T> {
  fn attach_to<'a>(host: &'a T) -> Self;
}
