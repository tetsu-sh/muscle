use chrono::DateTime;
use super::muscle::Muscle;


#[derive(Debug,Clone)]
pub struct Train{
    name:TrainName,
    id:TrainId,
    date:DateTime,
    volume:Volume,
    rep:Rep,
    set:Set,
}

impl Train {
   fn new(name:TrainName,volume:Volume,rep:Rep,set:Set){
        Self(name,"000",DateTime::date(&self),volume,rep,set)
   } 
}

pub struct TrainTemplate{
    name:TrainTemplateName,
    id:TrainTemplateId,
    muscle:Vec<Muscle>,

}

pub trait TrainRepository{
    fn create();
}

struct Volume(i32);

struct Set(i32);

struct Rep(i32);

struct TrainTemplateId(String);

struct TrainId(String);

struct TrainName(String);

struct TrainTemplateName(String);
