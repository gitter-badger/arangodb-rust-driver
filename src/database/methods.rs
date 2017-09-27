
use serde::ser::Serialize;

use api::{Method, Operation, Parameters, Prepare, RpcReturnType};
use user::UserExtra;
use super::types::*;

/// Retrieves information about the current database.
#[derive(Clone, Debug, PartialEq)]
pub struct GetCurrentDatabase {}

impl GetCurrentDatabase {
    /// Constructs a new `GetCurrentDatabase` method.
    pub fn new() -> Self {
        GetCurrentDatabase {}
    }
}

impl Method for GetCurrentDatabase {
    type Result = Database;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for GetCurrentDatabase {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/database/current")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Retrieves a list of all existing databases.
///
/// **Note**: retrieving the list of databases is only possible from within the
/// `_system` database.
/// **Note**: You should use the `user::ListDatabasesForUser` to fetch the
/// list of the available databases now.
#[derive(Clone, Debug, PartialEq)]
pub struct ListOfDatabases {}

impl ListOfDatabases {
    /// Constructs a new `ListOfDatabases` method.
    pub fn new() -> Self {
        ListOfDatabases {}
    }
}

impl Method for ListOfDatabases {
    type Result = Vec<String>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for ListOfDatabases {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/database")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Retrieves the list of all databases the current user can access without
/// specifying a different username or password.
#[derive(Clone, Debug, PartialEq)]
pub struct ListAccessibleDatabases {}

impl ListAccessibleDatabases {
    /// Constructs a new `ListAccessibleDatabases` method.
    pub fn new() -> Self {
        ListAccessibleDatabases {}
    }
}

impl Method for ListAccessibleDatabases {
    type Result = Vec<String>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for ListAccessibleDatabases {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/database/user")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Creates a new database.
///
/// **Note**: creating a new database is only possible from within the `_system`
/// database.
#[derive(Clone, Debug, PartialEq)]
pub struct CreateDatabase<E>
    where E: UserExtra
{
    database: NewDatabase<E>,
}

impl<E> CreateDatabase<E>
    where E: UserExtra
{
    /// Constructs a new `CreateDatabase` method with the parameters specified
    /// in the given `NewDatabase` struct.
    pub fn new(database: NewDatabase<E>) -> Self {
        CreateDatabase {
            database,
        }
    }

    /// Constructs a new `CreateDatabase` method with the given name used as
    /// the name of the database that is going to be created.
    ///
    /// All other parameters are left to their defaults.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        CreateDatabase {
            database: NewDatabase::with_name(name),
        }
    }

    /// Returns the `NewDatabase` parameters of this `CreateDatabase`
    /// method.
    pub fn database(&self) -> &NewDatabase<E> {
        &self.database
    }
}

impl<E> Method for CreateDatabase<E>
    where E: UserExtra
{
    type Result = bool;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl<E> Prepare for CreateDatabase<E>
    where E: UserExtra + Serialize
{
    type Content = NewDatabase<E>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from("/_api/database")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.database)
    }
}

/// Drops the database along with all data stored in it.
///
/// **Note**: dropping a database is only possible from within the `_system`
/// database. The `_system` database itself cannot be dropped.
#[derive(Clone, Debug, PartialEq)]
pub struct DropDatabase {
    database_name: String,
}

impl DropDatabase {
    /// Constructs a new `DropDatabase` method with the given name of the
    /// database to be dropped.
    pub fn new(database_name: String) -> Self
    {
        DropDatabase {
            database_name,
        }
    }

    /// Constructs a new `DropDatabase` method with the given name of the
    /// database to be dropped.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        DropDatabase {
            database_name: name.into(),
        }
    }

    /// Returns the name of the database to be dropped.
    pub fn database_name(&self) -> &str {
        &self.database_name
    }
}

impl Method for DropDatabase {
    type Result = bool;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for DropDatabase {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from("/_api/database/") + &self.database_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}