use crate::model::{item::Item, user::User};
use futures::TryStreamExt;
use mongodb::{bson::doc, error::Error, options::IndexOptions, Client, IndexModel};

const DB_NAME: &str = "hw3";
const USERS_COL: &str = "users";
const ITEMS_COL: &str = "items";

/// класс для работы с БД.
/// используется асинхронный клиент MongoDB
#[derive(Debug, Clone)]
pub struct MongoRepository {
    client: Client,
}

impl MongoRepository {
    pub async fn new(db_uri: &str) -> Result<Self, Error> {
        let client = Client::with_uri_str(db_uri).await?;
        Self::create_indexes(&client).await?;
        Ok(Self { client })
    }

    async fn create_indexes(client: &Client) -> Result<(), Error> {
        let options = IndexOptions::builder().unique(true).build();
        let users_model = IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(options.clone())
            .build();
        let items_model = IndexModel::builder()
            .keys(doc! { "name": 1 })
            .options(options)
            .build();

        client
            .database(DB_NAME)
            .collection::<User>(USERS_COL)
            .create_index(users_model, None)
            .await?;
        client
            .database(DB_NAME)
            .collection::<Item>(ITEMS_COL)
            .create_index(items_model, None)
            .await?;
        Ok(())
    }

    pub async fn insert_user(&self, user: User) -> Result<(), Error> {
        let collection = self.client.database(DB_NAME).collection(USERS_COL);
        collection.insert_one(user, None).await.map(|_| ())
    }

    pub async fn insert_item(&self, item: Item) -> Result<(), Error> {
        let collection = self.client.database(DB_NAME).collection(ITEMS_COL);
        collection.insert_one(item, None).await.map(|_| ())
    }

    pub async fn get_items(&self) -> Result<Vec<Item>, Error> {
        let collection = self.client.database(DB_NAME).collection(ITEMS_COL);
        let items = collection.find(doc! {}, None).await?;
        items.try_collect().await
    }

    pub async fn get_user(&self, username: String) -> Result<Option<User>, Error> {
        let collection = self.client.database(DB_NAME).collection::<User>(USERS_COL);
        collection.find_one(doc! {"username": username}, None).await
    }
}
