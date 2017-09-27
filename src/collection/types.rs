
use std::fmt;
use std::u8;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

/// The `Collection` struct holds attributes of a collection.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    /// The id of the collection.
    id: String,
    /// The name of the collection.
    name: String,
    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,
    /// The status of the collection.
    status: CollectionStatus,
    /// Whether the collection is a system collection or regular collection.
    is_system: bool,
}

impl Collection {
    /// Returns the id of this collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of this collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of this collection.
    pub fn kind(&self) -> &CollectionType {
        &self.kind
    }

    /// Returns the status of this collection.
    pub fn status(&self) -> &CollectionStatus {
        &self.status
    }

    /// Returns whether this collection is a system collection or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }
}

/// The `NewCollection` struct defines all attributes that may be specified for
/// a new collection that is going to be created.
///
/// The `name` attribute is mandatory. All other attributes are optional and
/// if not specified are assigned to their default values as defined by the
/// ArangoDB server.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCollection {
    /// The name of the collection.
    name: String,

    /// The type of the collection to create.
    /// (The default is 'Documents')
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<CollectionType>,

    /// If true, create a system collection. In this case collection-name should start with an
    /// underscore.
    /// (The default is false)
    ///
    /// End users should normally create non-system collections only. API implementors
    /// may be required to create system collections in very special occasions, but normally a
    /// regular collection will do.
    #[serde(skip_serializing_if = "Option::is_none")]
    is_system: Option<bool>,

    /// Key options.
    #[serde(skip_serializing_if = "Option::is_none")]
    key_options: Option<NewKeyOptions>,

    /// If true then the data is synchronized to disk before returning from a document create,
    /// update, replace or removal operation.
    /// (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_sync: Option<bool>,

    /// In a cluster, this value determines the number of shards to create for the collection.
    /// (The default is 1)
    ///
    /// In a single server setup, this option is meaningless.
    #[cfg(feature = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_shards: Option<u16>,

    /// (The default is [ "_key" ])
    ///
    /// In a cluster, this attribute determines which document attributes are used to determine
    /// the target shard for documents. Documents are sent to shards based on the values of their
    /// shard key attributes. The values of all shard key attributes in a document are hashed, and
    /// the hash value is used to determine the target shard.
    ///
    /// Note: Values of shard key attributes cannot be changed once set.
    ///
    /// In a single server setup, this option is meaningless.
    #[cfg(feature = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    shard_keys: Option<String>,

    /// The replication factor.
    /// (The default is 1)
    ///
    /// In a cluster, this attribute determines how many copies of each shard are kept on different
    /// DBServers. The value 1 means that only one copy (no synchronous replication) is kept. A
    /// value of k means that k-1 replicas are kept. Any two copies reside on different DBServers.
    /// Replication between them is synchronous, that is, every write operation to the "leader"
    /// copy will be replicated to all "follower" replicas, before the write operation is reported
    /// successful. If a server fails, this is detected automatically and one of the servers
    /// holding copies take over, usually without an error being reported.
    ///
    /// In a single server setup, this option is meaningless.
    #[cfg(feature = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_factor: Option<u16>,

    /// If true then the collection data is kept in-memory only and not made persistent.
    /// (The default is false)
    ///
    /// Unloading the collection will cause the collection data to be discarded. Stopping or
    /// re-starting the server will also cause full loss of data in the collection. Setting this
    /// option will make the resulting collection be slightly faster than regular collections
    /// because ArangoDB does not enforce any synchronization to disk and does not calculate any
    /// CRC checksum for datafiles (as there are no datafiles). This option should therefore be
    /// used for cache-type collections only, and not for data that cannot be re-created otherwise.
    ///
    /// This option is meaningful for the MMFiles storage engine only.
    #[cfg(feature = "mmfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    is_volatile: Option<bool>,

    /// Whether or not the collection will be compacted.
    /// (default is true)
    ///
    /// This option is meaningful for the MMFiles storage engine only.
    #[cfg(feature = "mmfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    do_compact: Option<bool>,

    /// The number of buckets into which indexes using a hash table are split.
    /// (The default is 16)
    ///
    /// This number has to be a power of 2 and less than or equal to 1024.
    ///
    /// For very large collections one should increase this to avoid long pauses when the hash table
    /// has to be initially built or resized, since buckets are resized individually and can be
    /// initially built in parallel. For example, 64 might be a sensible value for a collection with
    /// 100 000 000 documents. Currently, only the edge index respects this value, but other index
    /// types might follow in future ArangoDB versions. Changes (see below) are applied when the
    /// collection is loaded the next time.
    ///
    /// This option is meaningful for the MMFiles storage engine only.
    #[cfg(feature = "mmfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    index_buckets: Option<u16>,

    /// The maximal size of a journal or datafile in bytes.
    /// (The default is a configuration parameter)
    ///
    /// The value must be at least 1048576 (1 MiB).
    ///
    /// This option is meaningful for the MMFiles storage engine only.
    #[cfg(feature = "mmfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    journal_size: Option<u64>,
}

impl NewCollection {
    fn _new(name: String, kind: Option<CollectionType>, is_system: Option<bool>) -> Self {
        NewCollection {
            name,
            kind,
            is_system,
            key_options: None,
            wait_for_sync: None,
            #[cfg(feature = "cluster")]
            number_of_shards: None,
            #[cfg(feature = "cluster")]
            shard_keys: None,
            #[cfg(feature = "cluster")]
            replication_factor: None,
            #[cfg(feature = "mmfiles")]
            is_volatile: None,
            #[cfg(feature = "mmfiles")]
            do_compact: None,
            #[cfg(feature = "mmfiles")]
            index_buckets: None,
            #[cfg(feature = "mmfiles")]
            journal_size: None,
        }
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The newly created
    /// collection will be a documents collection.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::_new(name.into(), None, None)
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The kind attribute
    /// is set to `Documents` explicitly.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn documents_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::_new(name.into(), Some(CollectionType::Documents), None)
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The newly created
    /// collection will be an edges collection.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn edges_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::_new(name.into(), Some(CollectionType::Edges), None)
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The created collection
    /// will be a system collection holding documents.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn system_documents_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::_new(name.into(), Some(CollectionType::Documents), Some(true))
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The created collection
    /// will be a system collection for edges.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn system_edges_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::_new(name.into(), Some(CollectionType::Edges), Some(true))
    }

    /// Returns the name of the collection to be created.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the collection to be created.
    pub fn kind(&self) -> Option<&CollectionType> {
        self.kind.as_ref()
    }

    /// Returns whether the collection is going to be a system or regular
    /// collection.
    pub fn is_system(&self) -> Option<bool> {
        self.is_system
    }

    /// Sets the key options of the collection to be created.
    pub fn set_key_options(&mut self, key_options: Option<NewKeyOptions>) {
        self.key_options = key_options;
    }

    /// Returns the key options of the collection to be created.
    pub fn key_options(&self) -> Option<&NewKeyOptions> {
        self.key_options.as_ref()
    }

    /// Sets whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn set_wait_for_sync(&mut self, wait_for_sync: Option<bool>) {
        self.wait_for_sync = wait_for_sync;
    }

    /// Returns whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn wait_for_sync(&self) -> Option<bool> {
        self.wait_for_sync
    }

    /// Sets the number of shards that shall be created for this collection.
    #[cfg(feature = "cluster")]
    pub fn set_number_of_shards(&mut self, number_of_shards: Option<u16>) {
        self.number_of_shards = number_of_shards;
    }

    /// Returns the number of shards that shall be created for this collection.
    #[cfg(feature = "cluster")]
    pub fn number_of_shards(&self) -> Option<u16> {
        self.number_of_shards
    }

    /// Sets the keys to determine the shard for a collection.
    #[cfg(feature = "cluster")]
    pub fn set_shard_keys(&mut self, shard_keys: Option<String>) {
        self.shard_keys = shard_keys;
    }

    /// Returns the keys to determine the shard for a collection.
    #[cfg(feature = "cluster")]
    pub fn shard_keys(&self) -> Option<&String> {
        self.shard_keys.as_ref()
    }

    /// Sets the number of copies that are kept of each shard.
    #[cfg(feature = "cluster")]
    pub fn set_replication_factor(&mut self, replication_factor: Option<u16>) {
        self.replication_factor = replication_factor;
    }

    /// Returns the number of copies that are kept of each shard.
    #[cfg(feature = "cluster")]
    pub fn replication_factor(&self) -> Option<u16> {
        self.replication_factor
    }

    /// Sets whether this collection is going to be a volatile collection.
    #[cfg(feature = "mmfiles")]
    pub fn set_volatile(&mut self, volatile: Option<bool>) {
        self.is_volatile = volatile;
    }

    /// Returns whether this collection is going to be a volatile collection.
    #[cfg(feature = "mmfiles")]
    pub fn is_volatile(&self) -> Option<bool> {
        self.is_volatile
    }

    /// Sets whether this collection is going to be compacted.
    #[cfg(feature = "mmfiles")]
    pub fn set_do_compact(&mut self, do_compact: Option<bool>) {
        self.do_compact = do_compact;
    }

    /// Returns whether this collection is going to be compacted.
    #[cfg(feature = "mmfiles")]
    pub fn is_do_compact(&self) -> Option<bool> {
        self.do_compact
    }

    /// Sets the number of buckets into which indexes using a hash table
    /// are split.
    #[cfg(feature = "mmfiles")]
    pub fn set_index_buckets(&mut self, index_buckets: Option<u16>) {
        self.index_buckets = index_buckets;
    }

    /// Returns the number of buckets into which indexes using a hash table
    /// are split.
    #[cfg(feature = "mmfiles")]
    pub fn index_buckets(&self) -> Option<u16> {
        self.index_buckets
    }

    /// Sets the maximal size of a journal or datafile in bytes.
    #[cfg(feature = "mmfiles")]
    pub fn set_journal_size(&mut self, journal_size: Option<u64>) {
        self.journal_size = journal_size;
    }

    /// Returns the maximal size of a journal or datafile in bytes.
    #[cfg(feature = "mmfiles")]
    pub fn journal_size(&self) -> Option<u64> {
        self.journal_size
    }
}

/// The `NewKeyOptions` struct holds the key options to be used for the
/// collection that is going to be created.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NewKeyOptions {
    /// If set to true, then it is allowed to supply own key values in the _key attribute of a
    /// document. If set to false, then the key generator will solely be responsible for generating
    /// keys and supplying own key values in the _key attribute of documents is considered an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_user_keys: Option<bool>,
    /// Specifies the type of the key generator.
    ///
    /// The currently available generators are 'Traditional' and 'AutoIncrement'.
    #[serde(rename = "type")]
    kind: KeyGeneratorType,
    /// Increment value for 'AutoIncrement key generator.
    ///
    /// Used for 'AutoIncrement' key generator only.
    #[serde(skip_serializing_if = "Option::is_none")]
    increment: Option<u64>,
    /// Initial offset value for autoincrement key generator.
    ///
    /// Used for 'AutoIncrement' key generator only.
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
}

/// The `CollectionProperties` struct holds the attributes of a collection.
#[derive(Clone, Debug, Deserialize)]
pub struct CollectionProperties {
    /// The id of the collection.
    id: String,
    /// The name of the collection.
    name: String,
    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,
    /// The status of the collection.
    status: CollectionStatus,
    /// Whether the collection is system collection or regular collection.
    is_system: bool,
    /// The key options of the collection.
    key_options: KeyOptions,
    /// Whether the server should wait until the collection is synchronized to
    /// the file system before returning the response.
    wait_for_sync: bool,
    /// The number of shards of the collection.
    #[cfg(feature = "cluster")]
    number_of_shards: u16,
    /// The keys used to identify the shards of a collection.
    #[cfg(feature = "cluster")]
    shard_keys: String,
    /// The number of copies that are kept of each shard.
    #[cfg(feature = "cluster")]
    replication_factor: u64,
    /// Whether this collection is volatile.
    #[cfg(feature = "mmfiles")]
    is_volatile: bool,
    /// Whether this collection is compacted.
    #[cfg(feature = "mmfiles")]
    do_compact: bool,
    /// The number of buckets into which indexes using a hash table are split.
    #[cfg(feature = "mmfiles")]
    index_buckets: u16,
    /// The maximal size of a journal or datafile in bytes.
    #[cfg(feature = "mmfiles")]
    journal_size: u64,
}

impl CollectionProperties {
    /// Returns the id of the collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the collection.
    pub fn kind(&self) -> &CollectionType {
        &self.kind
    }

    /// Returns whether the collection is a system or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }

    /// Returns the key options of the collection.
    pub fn key_options(&self) -> &KeyOptions {
        &self.key_options
    }

    /// Returns whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn is_wait_for_sync(&self) -> bool {
        self.wait_for_sync
    }

    /// Returns the number of shards for this collection.
    #[cfg(feature = "cluster")]
    pub fn number_of_shards(&self) -> u16 {
        self.number_of_shards
    }

    /// Returns the keys to determine the shard for a collection.
    #[cfg(feature = "cluster")]
    pub fn shard_keys(&self) -> &str {
        &self.shard_keys
    }

    /// Returns the number of copies that are kept of each shard.
    #[cfg(feature = "cluster")]
    pub fn replication_factor(&self) -> u16 {
        self.replication_factor
    }

    /// Returns whether this collection is a volatile collection.
    #[cfg(feature = "mmfiles")]
    pub fn is_volatile(&self) -> bool {
        self.is_volatile
    }

    /// Returns whether this collection is compacted.
    #[cfg(feature = "mmfiles")]
    pub fn is_do_compact(&self) -> bool {
        self.do_compact
    }

    /// Returns the number of buckets into which indexes using a hash table
    /// are split.
    #[cfg(feature = "mmfiles")]
    pub fn index_buckets(&self) -> u16 {
        self.index_buckets
    }

    /// Returns the maximal size of a journal or datafile in bytes.
    #[cfg(feature = "mmfiles")]
    pub fn journal_size(&self) -> u64 {
        self.journal_size
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CollectionPropertiesUpdate {
    /// Whether the server should wait until the collection is synchronized to
    /// the file system before returning the response.
    wait_for_sync: Option<bool>,
    /// The maximal size of a journal or datafile in bytes.
    #[cfg(feature = "mmfiles")]
    journal_size: Option<u64>,
}

impl CollectionPropertiesUpdate {
    /// Constructs a new instance of `CollectionPropertiesUpdate` with no
    /// options set.
    pub fn new() -> Self {
        CollectionPropertiesUpdate {
            wait_for_sync: None,
            #[cfg(feature = "mmfiles")]
            journal_size: None,
        }
    }

    /// Sets whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn set_wait_for_sync(&mut self, wait_for_sync: Option<bool>) {
        self.wait_for_sync = wait_for_sync;
    }

    /// Returns whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn wait_for_sync(&self) -> Option<bool> {
        self.wait_for_sync
    }

    /// Sets the maximal size of a journal or datafile in bytes.
    #[cfg(feature = "mmfiles")]
    pub fn set_journal_size(&mut self, journal_size: Option<u64>) {
        self.journal_size = journal_size;
    }

    /// Returns the maximal size of a journal or datafile in bytes.
    #[cfg(feature = "mmfiles")]
    pub fn journal_size(&self) -> Option<u64> {
        self.journal_size
    }
}

/// The `RenameTo` struct holds the new name for rename methods.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RenameTo {
    /// The new name to rename the object to.
    name: String,
}

impl RenameTo {
    /// Constructs a new instance of the `RenameTo` struct with the given
    /// new name.
    pub fn new<N>(new_name: N) -> Self
        where N: Into<String>
    {
        RenameTo {
            name: new_name.into(),
        }
    }

    /// Returns the new name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// The `KeyOptions` struct holds the key related attributes.
#[derive(Clone, Debug, Deserialize)]
pub struct KeyOptions {
    /// If set to true, then it is allowed to supply own key values in the _key attribute of a
    /// document. If set to false, then the key generator will solely be responsible for generating
    /// keys and supplying own key values in the _key attribute of documents is considered an error.
    allow_user_keys: bool,
    /// Specifies the type of the key generator.
    ///
    /// The currently available generators are 'Traditional' and 'AutoIncrement'.
    #[serde(rename = "type")]
    kind: KeyGeneratorType,
    /// Last used key value.
    last_value: u64,
}

impl KeyOptions {
    /// Returns whether the collection allows user supplied key values.
    pub fn is_allow_user_keys(&self) -> bool {
        self.allow_user_keys
    }

    /// Returns the type of the key generator.
    pub fn kind(&self) -> &KeyGeneratorType {
        &self.kind
    }

    /// Returns the last value assigned by the key generator.
    pub fn last_value(&self) -> u64 {
        self.last_value
    }
}

/// The `CollectionType` enum defines the different types of collections.
#[derive(Clone, Debug, PartialEq)]
pub enum CollectionType {
    /// Documents collection
    Documents,
    /// Edges collection
    Edges,
}

impl Serialize for CollectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::CollectionType::*;
        let type_id = match *self {
            Documents => 2,
            Edges => 3,
        };
        serializer.serialize_u8(type_id)
    }
}

impl<'de> Deserialize<'de> for CollectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_u64(CollectionTypeVisitor)
    }
}

struct CollectionTypeVisitor;

impl<'de> Visitor<'de> for CollectionTypeVisitor {
    type Value = CollectionType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 2 and 3")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where E: de::Error
    {
        use self::CollectionType::*;
        match value {
            2 => Ok(Documents),
            3 => Ok(Edges),
            _ => Err(E::custom(format!("u64 out of range: {}", value))),
        }
    }
}

/// The `CollectionStatus` enum defines the possible states a collection can
/// be in.
#[derive(Clone, Debug, PartialEq)]
pub enum CollectionStatus {
    /// New born
    NewBorn,
    /// Unloaded
    Unloaded,
    /// Loaded
    Loaded,
    /// In the process of being unloaded
    BeingUnloaded,
    /// Deleted
    Deleted,
    /// In the process of being loaded
    BeingLoaded,
    /// Indicates a corrupted collection
    Corrupted,
}

impl Serialize for CollectionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::CollectionStatus::*;
        let status_id = match *self {
            NewBorn => 1,
            Unloaded => 2,
            Loaded => 3,
            BeingUnloaded => 4,
            Deleted => 5,
            BeingLoaded => 6,
            Corrupted => u8::MAX,
        };
        serializer.serialize_u8(status_id)
    }
}

impl<'de> Deserialize<'de> for CollectionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_u64(CollectionStatusVisitor)
    }
}

struct CollectionStatusVisitor;

impl<'de> Visitor<'de> for CollectionStatusVisitor {
    type Value = CollectionStatus;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 1 and 6")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where E: de::Error
    {
        use self::CollectionStatus::*;
        match value {
            1 => Ok(NewBorn),
            2 => Ok(Unloaded),
            3 => Ok(Loaded),
            4 => Ok(BeingUnloaded),
            5 => Ok(Deleted),
            6 => Ok(BeingLoaded),
            _ => Ok(Corrupted),
        }
    }
}

/// The `KeyGeneratorType` enum defines the types of key generators that are
/// available.
#[derive(Clone, Debug, PartialEq)]
pub enum KeyGeneratorType {
    //TODO clarify what the `Traditional` key generator actually is.
    Traditional,
    //TODO clarify what the `AutoIncrement` key generator actually is.
    AutoIncrement,
}

impl Serialize for KeyGeneratorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::KeyGeneratorType::*;
        let type_str = match *self {
            Traditional => "traditional",
            AutoIncrement => "autoincrement",
        };
        serializer.serialize_str(type_str)
    }
}

impl<'de> Deserialize<'de> for KeyGeneratorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(KeyGeneratorTypeVisitor)
    }
}

struct KeyGeneratorTypeVisitor;

impl<'de> Visitor<'de> for KeyGeneratorTypeVisitor {
    type Value = KeyGeneratorType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string that is either 'traditional' or 'autoincrement'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        use self::KeyGeneratorType::*;
        match value {
            "traditional" => Ok(Traditional),
            "autoincrement" => Ok(AutoIncrement),
            _ => Err(E::custom(format!("Unknown KeyGeneratorType: {}", value))),
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where E: de::Error
    {
        self.visit_str(&value)
    }
}