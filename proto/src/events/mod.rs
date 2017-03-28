use config::XMPPConfig;

pub trait EventTrait {
    type Item;
    fn namespace() -> &'static str;
    fn new(config: &XMPPConfig) -> Self;
    fn compute(&self) -> String;
}

pub struct Event<T> {
    inner: Box<T>
}

impl<T: EventTrait> Event<T> {
    pub fn new(config: &XMPPConfig) -> Event<T> {
        Event {
            inner: Box::new(T::new(config))
        }
    }

    pub fn compute(&self) -> String {
        self.inner.compute()
    }
}

mod stream;

pub type OpenStreamEvent = Event<stream::OpenStream>;
pub type StartTlsEvent = Event<stream::StartTls>;
