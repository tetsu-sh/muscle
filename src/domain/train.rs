use super::muscle::Muscle;
use uuid::Uuid;


#[derive(Debug,Clone)]
pub struct Train{
    pub name:TrainName,
    pub id:TrainId,
    pub volume:Volume,
    pub rep:Rep,
    pub set:Set,
}


impl Train {
   pub fn new(name:String,volume:i32,rep:i32,set:i32)->Self{
       let id=TrainId(Uuid::new_v4().to_string());
        Self{name:TrainName(name),id,volume:Volume(volume),rep:Rep(rep),set:Set(set)}
   }
}


#[derive(Debug,Clone)]
pub struct TrainTemplate{
    name:TrainTemplateName,
    id:TrainTemplateId,
    muscle:Vec<Muscle>,

}

impl TrainTemplate  {
   fn new(name:TrainTemplateName,muscle:Vec<Muscle>)-> Self{
       Self{
           name,
           id:TrainTemplateId(Uuid::new_v4().to_string()),
           muscle, 
        }
       
   } 
}

pub trait TrainRepository{
    fn create(&self);
    fn find_by_name(&self)->Train;
}

#[derive(Debug,Clone)]
pub struct Volume(i32);

#[derive(Debug,Clone)]
pub struct Set(i32);

#[derive(Debug,Clone)]
pub struct Rep(i32);

#[derive(Debug,Clone)]
struct TrainTemplateId(String);

#[derive(Debug,Clone)]
pub struct TrainId(pub String);

#[derive(Debug,Clone)]
pub struct TrainName(pub String);

#[derive(Debug,Clone)]
struct TrainTemplateName(String);

// #[cfg(test)]
// mod tests {
//     use chrono::DateTime;
//     #[test]
//     fn test_train_new(){
//         let test_name=String::from("test_train");
//         let test_datetime=DateTime::now();

//     }
// }