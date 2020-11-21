use std::clone::Clone;
use std::{sync::{Arc, Mutex},
         thread, time
         };

use rand::seq::SliceRandom;

const SIGNAL_DELAY_TIME: u64 = 10;       //in seconds
const WALK_CROSSING_TIME: u64 = 5;       //in seconds

#[derive(Debug, Clone)]
enum Signal {
    STOP,
    CAUTION,
    GO,
}

#[derive(Debug, Clone)]
struct TrafficSignal{
   signal: Signal,
   passage_requested: bool,
}

impl TrafficSignal{
   fn new()->TrafficSignal{
      TrafficSignal{
         signal: Signal::STOP,
         passage_requested: false,
      }
   }

   fn get_signal(&self)->&Signal{
      &self.signal
   }

   fn change_signal(& mut self){
      self.signal = match self.signal{
         Signal::STOP => {
                           if self.passage_requested{
                              self.walk_message();
                              thread::sleep(time::Duration::from_secs(SIGNAL_DELAY_TIME))
                           } 
                           self.passage_requested = false; 
                           Signal::GO
                         }
         Signal::GO => Signal::CAUTION,
         Signal::CAUTION => Signal::STOP
      };
   }

   fn request_passage(&mut self){
      if !self.passage_requested{
         println!(" Walk button pressed, wait for signal to cross");
         self.passage_requested = true;
      }
   }
   fn get_message(&self){
      println!(" traffic signal => {:?}", self.get_signal());
   }
   
   fn execute(&mut self){
         self.change_signal();
         self.get_message();   
   }

   fn walk_message(&mut self){
      println!("");
      println!(" WALK signal - ON, finish within time of {:?} ...",WALK_CROSSING_TIME);
      for i in (1..=WALK_CROSSING_TIME).rev(){
         println!("  \u{1F463} {:?} ", i);
         thread::sleep(time::Duration::from_secs(1));
      }
      println!(" WALK signal - OFF");
      println!("");
      self.passage_requested = false;
   }
}

pub fn run(){
   let tfs = TrafficSignal::new();   
   let shared_tfs = Arc::new(Mutex::new(tfs));
   let shared_tfs_clone = Arc::clone(&shared_tfs);

   let traffic_signal_worker_thrd = thread::spawn(move ||{
      loop{
         let mut tfs = shared_tfs.lock().unwrap();
         tfs.execute();
         match tfs.get_signal(){
            Signal::STOP => {
               if tfs.passage_requested{
                  thread::sleep(time::Duration::from_secs(1));   
               }else{
                  thread::sleep(time::Duration::from_secs(SIGNAL_DELAY_TIME));
               }
            }
            _ => thread::sleep(time::Duration::from_secs(SIGNAL_DELAY_TIME)),
         }
      };
   });
   
   let walk_press_worker_thrd = thread::spawn(move || {
      let walker_frequency = vec![2, 5, 10, 20];
      let random_walker_frequency = *(walker_frequency.choose(&mut rand::thread_rng()).unwrap()) as u64;
         loop{
               let mut tfs = shared_tfs_clone.lock().unwrap();
               if !tfs.passage_requested{
                  tfs.request_passage();
                  thread::sleep(time::Duration::from_secs(random_walker_frequency));
               }
         }
   });
   
   traffic_signal_worker_thrd.join().unwrap();
   walk_press_worker_thrd.join().unwrap();
}
