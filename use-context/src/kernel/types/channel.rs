pub type Sender<T> = kanal::Sender<T>;
pub type Receiver<T> = kanal::Receiver<T>;
pub type RecvTimeoutError = kanal::ReceiveErrorTimeout;
pub use sal_sync::sync::channel as channel;