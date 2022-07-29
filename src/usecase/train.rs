


use crate::domain::train::{Train, TrainRepository};


pub struct TrainUsecase<T:TrainRepository>{
    pub train_repository:T,
}

impl <T:TrainRepository>TrainUsecase<T>{
    pub fn new(train_repository:T)->Self{
        Self{train_repository}
    }

    pub fn create_train(&self,trainname:String,volume:i32,set:i32,rep:i32)->Train{

        let train=Train::new(trainname,volume,set,rep);
        println!("{:?}",train);
        self.train_repository.create();
        train
    }

    fn train(_:Self,trains:Vec<Train>)->Train{
        todo!()
    }
}
