use lazy_static::lazy_static;
use std::sync::{Mutex};

lazy_static! {
   pub static ref CONFIG_SINGLETON: Mutex<Option<Config>> = Mutex::new(None);
}


#[derive(Debug)]
   pub struct Config{
      field:String,
      val:u8,
   }

   impl Config{
      pub fn set_field(&mut self, field: String){
         self.field = field;
      }
      pub fn get_field(&self)->&str{
         &self.field
      }

      pub fn set_val(&mut self, val: u8){
         self.val = val;
      }
      pub fn get_val(&self)->u8{
         self.val
      }
      
      pub fn initialize(field: String, val: u8) {
         let mut st = CONFIG_SINGLETON.lock().unwrap();
         if st.is_none() {
             let config = Config{field, val};
             *st = Some(config);
         } else {
             panic!("1st Config object is already created, 2nd can't be created");
         }
     }
 
     pub fn get()-> &'static Mutex<Option<Self>> {
         if CONFIG_SINGLETON.lock().unwrap().is_some() {
             &CONFIG_SINGLETON
         } else {
             panic!("Singleton must be initialized before use");
         }
     }
   }
