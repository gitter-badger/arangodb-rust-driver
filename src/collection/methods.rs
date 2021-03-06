
use api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use consts::{FIELD_CODE, FIELD_ID, FIELD_RESULT, PARAM_EXCLUDE_SYSTEM,
    PATH_API_COLLECTION, PATH_PROPERTIES, PATH_RENAME, VALUE_TRUE};
#[cfg(feature = "cluster")]
use consts::{PARAM_WAIT_FOR_SYNC_REPLICATION, VALUE_ZERO};
use super::types::*;

/// Retrieves a list of existing collections.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct ListCollections {
    /// Whether or not to exclude system collections from the response.
    exclude_system: bool
}

impl ListCollections {
    /// Constructs a new instance of the `ListCollections` method with
    /// the `exclude_system` parameter set to `true`.
    pub fn new() -> Self {
        ListCollections {
            exclude_system: true,
        }
    }

    /// Constructs a new instance of the `ListCollections` method with
    /// the `exclude_system` parameter set to `false`.
    pub fn including_system() -> Self {
        ListCollections {
            exclude_system: false,
        }
    }

    /// Sets whether system collections shall be excluded from the response.
    pub fn set_exclude_system(&mut self, exclude: bool) {
        self.exclude_system = exclude;
    }

    /// Returns whether system collections are going to be excluded from the
    /// response.
    pub fn is_exclude_system(&self) -> bool {
        self.exclude_system
    }
}

impl Method for ListCollections {
    type Result = Vec<Collection>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_RESULT),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ListCollections {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_COLLECTION)
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        if self.exclude_system {
            params.push(PARAM_EXCLUDE_SYSTEM, VALUE_TRUE);
        }
        params
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Creates a new collection with the given name.
#[derive(Clone, Debug, PartialEq)]
pub struct CreateCollection {
    collection: NewCollection,
    #[cfg(feature = "cluster")]
    wait_for_sync_replication: bool,
}

impl CreateCollection {
    /// Constructs a new instance of the `CreateCollection` method with the
    /// given `NewCollection` parameters.
    pub fn new(collection: NewCollection) -> Self {
        CreateCollection {
            collection,
            #[cfg(feature = "cluster")]
            wait_for_sync_replication: true,
        }
    }

    /// Constructs a new instance of the `CreateCollection` method that will
    /// create a new collection with the given name and the default collection
    /// type. The default collection type is defined by the ArangoDB server.
    ///
    /// All other parameters will be set to their default values.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        CreateCollection {
            collection: NewCollection::with_name(name),
            #[cfg(feature = "cluster")]
            wait_for_sync_replication: true,
        }
    }

    /// Constructs a new instance of the `CreateCollection` method that will
    /// create a new documents collection with the given name.
    ///
    /// All other parameters will be set to their default values.
    pub fn documents_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        CreateCollection {
            collection: NewCollection::documents_with_name(name),
            #[cfg(feature = "cluster")]
            wait_for_sync_replication: true,
        }
    }

    /// Constructs a new instance of the `CreateCollection` method that will
    /// create a new edge collection with the given name.
    ///
    /// All other parameters will be set to their default values.
    pub fn edges_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        CreateCollection {
            collection: NewCollection::edges_with_name(name),
            #[cfg(feature = "cluster")]
            wait_for_sync_replication: true,
        }
    }

    #[cfg(feature = "cluster")]
    /// Set whether the server shall wait until the new collection has been
    /// created at all replications before it returns the response.
    pub fn set_wait_for_sync_replication(&mut self, wait_for_sync_replication: bool) {
        self.wait_for_sync_replication = wait_for_sync_replication;
    }

    /// Returns the parameters that are going to be used to create the new
    /// collection.
    pub fn collection(&self) -> &NewCollection {
        &self.collection
    }

    #[cfg(feature = "cluster")]
    /// Returns whether the request will wait until the new collection has
    /// been created at all replications.
    pub fn is_wait_for_sync_replication(&self) -> bool {
        self.wait_for_sync_replication
    }
}

impl Method for CreateCollection {
    type Result = BasicCollectionProperties;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for CreateCollection {
    type Content = NewCollection;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_COLLECTION)
    }

    #[cfg(not(feature = "cluster"))]
    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    #[cfg(feature = "cluster")]
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        if !self.wait_for_sync_replication {
            params.push(PARAM_WAIT_FOR_SYNC_REPLICATION, VALUE_ZERO);
        }
        params
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.collection)
    }
}

/// Drops the collection identified by the given name.
///
/// If the collection was successfully dropped, the identifier of the dropped
/// collection is returned.
#[derive(Clone, Debug, PartialEq)]
pub struct DropCollection {
    name: String,
    system: bool,
}

impl DropCollection {
    /// Constructs a new instance of the `DropCollection` method that is
    /// going to be drop the user collection identified by the given name.
    ///
    /// **Note**: This method returns a `DropCollection` instance that drops
    /// user collections only. To drop a system collection either use the
    /// constructor method `DropCollection::system_with_name` or set the
    /// is_system property to `true` explicitly by calling the function
    /// `DropCollection::set_system`.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        DropCollection {
            name: name.into(),
            system: false,
        }
    }

    /// Constructs a new instance of the `DropCollection` method that is
    /// going to be drop the system collection identified by the given name.
    ///
    /// **Note**: This method returns a `DropCollection` instance that drops
    /// system collections only. To drop a user collection either use the
    /// constructor method `DropCollection::with_name` or set the
    /// is_system property to `false` explicitly by calling the function
    /// `DropCollection::set_system`.
    pub fn system_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        DropCollection {
            name: name.into(),
            system: true,
        }
    }

    /// Set whether the collection to be dropped is a system collection.
    ///
    /// The collection is dropped only when this property is reflects the
    /// type of the collection to be dropped.
    pub fn set_system(&mut self, system: bool) {
        self.system = system;
    }

    /// Returns the name of the collection to be dropped.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the collection to be dropped is a system collection.
    pub fn is_system(&self) -> bool {
        self.system
    }
}

impl Method for DropCollection {
    type Result = String;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_ID),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for DropCollection {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_COLLECTION)
            + "/" + &self.name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Fetch information about the collection identified by the given name.
#[derive(Clone, Debug, PartialEq)]
pub struct GetCollection {
    name: String,
}

impl GetCollection {
    /// Constructs a new instance of the `GetCollection` method.
    pub fn new(name: String) -> Self {
        GetCollection {
            name,
        }
    }

    /// Constructs a new instance of the `GetCollection` method to get
    /// information about the collection with the given name.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        GetCollection {
            name: name.into(),
        }
    }

    /// Returns the name of the collection for which the information shall
    /// be fetched.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Method for GetCollection {
    type Result = Collection;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetCollection {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_COLLECTION)
            + "/" + &self.name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Fetch the properties of the collection identified by the given name.
#[derive(Clone, Debug, PartialEq)]
pub struct GetCollectionProperties {
    name: String,
}

impl GetCollectionProperties {
    /// Constructs a new instance of the `GetCollectionProperties` method.
    pub fn new(name: String) -> Self {
        GetCollectionProperties {
            name,
        }
    }

    /// Constructs a new instance of the `GetCollectionProperties` method to
    /// get properties about the collection with the given name.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        GetCollectionProperties {
            name: name.into(),
        }
    }

    /// Returns the name of the collection for which the properties shall
    /// be fetched.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Method for GetCollectionProperties {
    type Result = CollectionProperties;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetCollectionProperties {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_COLLECTION)
            + "/" + &self.name
            + "/" + PATH_PROPERTIES
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Change the properties of a collection identified by name.
///
/// **Note:** except for `wait_for_sync`, `journal_size` (for MMFiles DB) and
/// name, collection properties cannot be changed once a collection is created.
/// To rename a collection, the `RenameCollection` method must be used.
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeCollectionProperties {
    name: String,
    updates: CollectionPropertiesUpdate,
}

impl ChangeCollectionProperties {
    /// Constructs a new instance of the `ChangeCollectionProperties` method.
    ///
    /// The `name` parameter must contain the name of the collection for which
    /// the properties shall be changed. The `updates` parameter contains the
    /// actual changes that shall be applied.
    pub fn new(name: String, updates: CollectionPropertiesUpdate) -> Self {
        ChangeCollectionProperties {
            name,
            updates,
        }
    }

    /// Returns the name of the collection for which the properties shall be
    /// changed.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the updates that shall be applied to the collection's
    /// properties.
    pub fn updates(&self) -> &CollectionPropertiesUpdate {
        &self.updates
    }
}

impl Method for ChangeCollectionProperties {
    type Result = CollectionProperties;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ChangeCollectionProperties {
    type Content = CollectionPropertiesUpdate;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_COLLECTION)
            + "/" + &self.name + "/" + PATH_PROPERTIES
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.updates)
    }
}

/// Renames a collection.
///
/// ***Note:** this method is not available in a cluster.
#[derive(Clone, Debug, PartialEq)]
pub struct RenameCollection {
    name: String,
    rename_to: RenameTo,
}

impl RenameCollection {
    /// Constructs a new instance of the `RenameCollection` method with all
    /// parameters specified.
    pub fn new(name: String, rename_to: RenameTo) -> Self {
        RenameCollection {
            name,
            rename_to,
        }
    }

    /// Returns a builder to construct a new instance of the `RenameCollection`
    /// method that will rename the collection identified by the given name.
    pub fn with_name<N>(name: N) -> RenameCollectionBuilder
        where N: Into<String>
    {
        RenameCollectionBuilder {
            collection_name: name.into(),
        }
    }
}

impl Method for RenameCollection {
    type Result = Collection;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for RenameCollection {
    type Content = RenameTo;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_COLLECTION)
            + "/" + &self.name
            + "/" + PATH_RENAME
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.rename_to)
    }
}

/// A struct that helps to provide an efficient fluent API to build a new
/// instance of the `RenameCollection` method.
#[derive(Debug)]
pub struct RenameCollectionBuilder {
    collection_name: String,
}

impl RenameCollectionBuilder {
    //noinspection RsSelfConvention
    /// Constructs a new instance of the `RenameCollection` method for the
    /// collection name of this builder and the given new name.
    pub fn to_name<N>(self, name: N) -> RenameCollection
        where N: Into<String>
    {
        RenameCollection {
            name: self.collection_name,
            rename_to: RenameTo::new(name),
        }
    }
}
