use super::muscle::Muscle;
use log::info;
use serde_json::json;
use uuid::Uuid;
use crate::utils::errors::MyError;

#[derive(Debug,Clone)]
pub struct Train{
    pub name:String,
    pub id:String,
    pub volume:i32,
    pub rep:i32,
    pub set:i32,
}


impl Train {
   pub fn new(name:String,volume:i32,rep:i32,set:i32)->Result<Self, MyError>{
        if volume>1000{return Err(MyError::BadRequest(json!({"error":"volume must be less than 1000.you are possibly not human."}))); };
        if rep>1000{return Err(MyError::BadRequest(json!({"error":"rep must be less than 1000.you are possibly not human."}))); };
        if set>1000{return Err(MyError::BadRequest(json!({"error":"set must be less than 1000.you are possibly not human."}))); };
        if name.chars().count()>30{return Err(MyError::BadRequest(json!({"error":"トレーニング名は30文字以内です"}))); }
       let id=Uuid::new_v4().to_string();
        Ok(Self{name,id,volume,rep,set})
   }
}


#[derive(Debug,Clone)]
pub struct TrainTemplate{
    name:String,
    id:String,
    muscle:Vec<Muscle>,

}

impl TrainTemplate  {
   fn new(name:String,muscle:Vec<Muscle>)-> Self{
       Self{
           name,
           id:Uuid::new_v4().to_string(),
           muscle, 
        }
       
   } 
}

pub trait TrainRepository{
    fn create(&self);
    fn find_by_name(&self)->Train;
}


// #[cfg(test)]
// mod tests {
//     use chrono::DateTime;
//     #[test]
//     fn test_train_new(){
//         let test_name=String::from("test_train");
//         let test_datetime=DateTime::now();

//     }
// }