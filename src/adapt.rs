use values;
use devices;

pub enum Error;

pub struct AdapterId;

/// API that adapters must implement
pub trait Adapter {
    /// An id unique to this adapter. This id must persist between
    /// reboots/reconnections.
    fn id(&self) -> AdapterId;

    /// The name of the adapter.
    fn name(&self) -> &String;
    fn vendor(&self) -> &String;
    fn version(&self) -> &[u32;4];
    // ... more metadata
    
    /// Request a value from a service. The FoxBox (not the adapter)
    /// is in charge of keeping track of the age of values.
    fn get_value(&self, id: ServiceId<Input>) -> Result<Option<values::Value>, Error>;

    /// Request that a value be sent to a service.
    fn put_value(&self, id: ServiceId<Output>, value: values::Value) -> Result<(), Error>;
}

/// API provided by the FoxBox to adapters
pub trait Adapt {
    fn add_adapter(&self, Arc<Adapter>) -> Result<(), Error>;
    fn remove_adapter(&self, Arc<Adapter>) -> Result<(), Error>;

    /// Add a node to the system. Called by the adapter when a new
    /// node has been detected/configured. The adapter is in charge of
    /// making sure that identifiers persist across reboots.
    fn add_node(&self, Arc<Adapter>, devices::Node) -> Result<(), Error>;
    fn remove_node(&self, Arc<Adapter>, devices::Node) -> Result<(), Error>;

    /// Add an input to the system. Called by the adapter when a new
    /// input has been detected/configured. The adapter is in charge
    /// of making sure that identifiers persist across reboots.
    fn add_input(&self, Arc<Adapter>, devices::Service<Input>) -> Result<(), Error>;
    fn remove_input(&self, Arc<Adapter>, devices::Service<Input>) -> Result<(), Error>;
    fn add_output(&self, Arc<Adapter>, devices::Service<Output>) -> Result<(), Error>;
    fn remove_output(&self, Arc<Adapter>, devices::Service<Output>) -> Result<(), Error>;

    /// Put a value that has been received by an input service.
    fn put_value(&self, Arc<Adapter>, ServiceId<Input>, value::Value) -> Result<(), Error>;
}
