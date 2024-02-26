use mongodb::{IndexModel, sync::{Client, Database, Collection}};
use mongodb::bson::doc;

use crate::runner::RunResult;
use crate::help_parser::Ai;

pub struct Dao {
    _client: Client,
    database: Database,
}

impl Dao {
    fn init_database(db: &Database) {
        let players: Collection<Ai> = db.collection("players");
        players.create_index(
            IndexModel::builder().keys(doc!{"name": 1, "version": 1}).build(),
            None).unwrap();
        let run_results: Collection<RunResult> = db.collection("run_results");
        run_results.create_index(
            IndexModel::builder().keys(doc!{"player_results.name": 1, "player_results.version": 1}).build(),
            None).unwrap();
    }

    pub fn new(uri: &str, database: &str) -> Dao {
        let client = Client::with_uri_str(uri).unwrap();
        let database = client.database(database);
        Dao::init_database(&database);
        return Dao{
            _client: client,
            database
        }
    }

    pub fn insert_run_result(self, run_result: RunResult) {
        let collection: Collection<RunResult> = self.database.collection("run_results");
        let res = collection.insert_one(run_result, None).unwrap();
        println!("Inserted run result with _id: {}", res.inserted_id);
    }
}