use crate::{
    domain::{
        muscle::Muscle,
        train::{Train, TrainRepository},
    },
    utils::errors::MyError,
};
use async_trait::async_trait;
use sqlx::MySqlPool;

pub struct TrainRepositoryImpl<'a> {
    pub conn: &'a MySqlPool,
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct FetchTrainRdbQueryModel {
    pub id: String,
    pub train_name: String,
    pub volume: i32,
    pub rep: i32,
    pub muscle_name: String,
    pub size: String,
    pub body_part_name: String,
}

impl FetchTrainRdbQueryModel {
    pub fn new(
        id: String,
        train_name: String,
        volume: i32,
        rep: i32,
        muscle_name: String,
        size: String,
        body_part_name: String,
    ) -> FetchTrainRdbQueryModel {
        FetchTrainRdbQueryModel {
            id,
            train_name,
            volume,
            rep,
            muscle_name,
            size,
            body_part_name,
        }
    }

    fn to_domain(items: Vec<FetchTrainRdbQueryModel>) -> Result<Train, MyError> {
        let mut muscles = vec![];
        for item in items.iter() {
            muscles.push(Muscle::from(
                item.id.clone(),
                item.muscle_name.clone(),
                item.body_part_name.clone(),
                item.size.clone(),
            )?)
        }
        let initial_item = items[0].clone();
        Train::new(
            initial_item.train_name,
            initial_item.volume,
            initial_item.rep,
            muscles,
        )
    }
}

#[async_trait]
impl TrainRepository for TrainRepositoryImpl<'_> {
    async fn create(&self, train: &Train, account_id: &String) -> Result<(), MyError> {
        // store train model and connect train and muscles.
        // TODO transaction,bulk insert.
        for muscle in train.muscles.iter() {
            sqlx::query!(
                "insert into train_muscle(train_id,muscle_id) values(?,?)",
                train.id,
                muscle.id
            )
            .execute(self.conn)
            .await?;
        }
        sqlx::query!(
            "insert into trains(id,account_id,name,volume,rep) values(?,?,?,?,?)",
            train.id,
            account_id,
            train.name,
            train.volume,
            train.rep
        )
        .execute(self.conn)
        .await?;

        Ok(())
    }

    async fn fetch_one(&self, id: &String) -> Result<Train, MyError> {
        let query_trains = sqlx::query!(
            "select t.id,t.name,t.volume,t.rep,m.id as train_id,m.name as muscle_name,m.size,bp.name as body_part_name
            from trains as t 
            join train_muscle as tm on t.id=tm.train_id
            join muscles as m on tm.muscle_id=m.id 
            join body_parts as bp on m.body_part_id=bp.id 
            where t.id=?",
            id
        )
        .fetch_all(self.conn)
        .await?
        .iter()
        .map(|x|FetchTrainRdbQueryModel::new(x.id.clone(), x.name.clone(), x.volume, x.rep, x.muscle_name.clone(), x.size.clone(), x.body_part_name.clone())).collect::<Vec<FetchTrainRdbQueryModel>>();

        FetchTrainRdbQueryModel::to_domain(query_trains)
    }

    fn find_by_name(&self) {
        todo!()
    }
}
