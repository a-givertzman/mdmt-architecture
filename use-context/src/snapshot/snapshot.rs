use std::sync::Arc;

use sal_core::error::Error;
use sal_sync::sync::channel::Sender;

///
/// Transactional [Snapshot]
/// - Accumulating calculation results.
/// - Atomic database inserts
/// - UI synchronization.
pub struct Snapshot {
    link: Sender<Event>,
    api_client: Arc<ApiClient>,
    items: Vec<(&'static str, String)>,
}
impl Snapshot {
    ///
    /// Creates a new [Snapshot] instance.
    pub fn new(link: Sender<Event>, api_client: Arc<ApiClient>) -> Self {
        Self {
            link,
            api_client,
            items: Vec::new(),
        }
    }
    ///
    /// Adds a [Context] member to the transaction
    pub fn add(&mut self, items: impl Properties) {
        for item in items.properties() {
            self.items.push(item);
        }
    }
    ///
    /// Sendins the current Snapshot items to the UI
    /// - Useful for user confirmation in case of consistency conflicts
    pub fn send(&self) {
        // Логика отправки событий на UI
        let result = self.link.send(
            Event::from(&self.items),
        );
        if let Err(err) = result {
            log::error!("Snapshot.semd | Error: {:?}", err);
        }
    }
    ///
    /// Completes the transaction
    /// - Applies all members to the database
    /// - Prevents double commits.
    pub fn commit(self) -> Result<(), Error> {
        let upsert = Upsert::new();
        for item in self.items {
            upsert.insert(item)
        }
        self.api_client.request(upsert.build())
            .map_err(|err| Error::new("Snapshot", "commit").pass(err))
    }
    ///
    /// Cancels the transaction
    /// - Discards all accumulated items
    pub fn rollback(self) {
        drop(self);
    }
}
///
/// Trait for converting [Context] members into key-value properties
/// - Context -> `DB.properties` adapter
pub trait Properties {
    fn properties(&self) -> Vec<(&'static str, String)>;
}

//
// Temprorary structures

/// To be removed
struct Event {}
impl From<&Vec<(&'static str, String)>> for Event {
    fn from(value: &Vec<(&'static str, String)>) -> Self {
        Self {  }
    }
}
/// To be removed
struct Sql();
/// To be removed
struct ApiClient {}
impl ApiClient {
    pub fn request(&self, sql: Sql) -> Result<(), Error> {
        Ok(())
    }
}
/// To be removed
struct Upsert {}
impl Upsert {
    pub fn new() -> Self {
        Self {}
    }
    pub fn insert(&self, item: (&str, String)) {
        
    }
    pub fn build(&self) -> Sql {
        Sql()
    }
}