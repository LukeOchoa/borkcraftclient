use std::vec;

use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use eframe::egui::Image;

//use eframe::egui::menu::SubMenu;

#[macro_use]
extern crate serde_derive;
// extern crate serde;
// extern crate serde_json;

pub trait FormatedStructString {
    fn to_formated_string(&self) -> String;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Breaker {
    #[serde(rename = "AllNetherPortals")]
    pub all_nether_portals: Vec<SecondBreaker>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SecondBreaker {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Nether")]
    pub nether: ThirdBreaker,
    #[serde(rename = "OverWorld")]
    pub over_world: ThirdBreaker,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ThirdBreaker {
    #[serde(rename = "Xcord")]
    pub x_cord: i32,
    #[serde(rename = "Ycord")]
    pub y_cord: i32,
    #[serde(rename = "Zcord")]
    pub z_cord: i32,
    #[serde(rename = "Locale")]
    pub locale: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "Notes")]
    pub notes: String,
}

impl FormatedStructString for MoreDeath {
    fn to_formated_string(&self) -> String {
        fn format_third_breaker(val: &Death) -> String {
            format!(
                "\t\tXcord: {}\n\t\tYcord: {}\n\t\tZcord: {}\n\t\tLocale: {}\n\t\tOwner: {}\n\t\tNotes: {}",
                val.x_cord, val.y_cord, val.z_cord, val.locale, val.owner, val.notes
            )
        }
        let mut breaker: String = String::new();
        for value in self.all_nether_portals.iter() {
            breaker = breaker
                + &format!(
                    "[->\n\tId: {} \n\tNether:\n{} 
                    \n\tOverWorld:\n{}",
                    value.id,
                    format_third_breaker(&value.nether),
                    format_third_breaker(&value.over_world),
                )
                + "\n<-]\n\n";
        }

        breaker
    }
}

impl FormatedStructString for Breaker {
    fn to_formated_string(&self) -> String {
        fn format_third_breaker(val: &ThirdBreaker) -> String {
            format!(
                "\t\tXcord: {}\n\t\tYcord: {}\n\t\tZcord: {}\n\t\tLocale: {}\n\t\tOwner: {}\n\t\tNotes: {}",
                val.x_cord, val.y_cord, val.z_cord, val.locale, val.owner, val.notes
            )
        }
        let mut breaker: String = String::new();
        for value in self.all_nether_portals.iter() {
            breaker = breaker
                + &format!(
                    "[->\n\tId: {} \n\tNether:\n{} 
                    \n\tOverWorld:\n{}",
                    value.id,
                    format_third_breaker(&value.nether),
                    format_third_breaker(&value.over_world),
                )
                + "\n<-]\n\n";
        }

        breaker
    }
}

pub enum ThirdBreakerVal {
    Text(String),
    Integer(i32),
}
pub enum ThirdBreakerValMutRef<'a> {
    Text(&'a mut String),
    Integer(&'a mut i32),
}

pub struct MoreDeath {
    pub all_nether_portals: Vec<SomeDeath>,
}
#[derive(Default, Debug)]
pub struct SomeDeath {
    pub id: String,
    pub nether: Death,
    pub over_world: Death,
}

#[derive(Default, Debug)]
pub struct Death {
    pub x_cord: String,
    pub y_cord: String,
    pub z_cord: String,
    pub locale: String,
    pub owner: String,
    pub notes: String,
}

impl SomeDeath {
    pub fn default() -> Self {
        let instance: Self = Default::default();
        instance
    }

    pub fn return_mut_ref(&mut self, field: String, which_struct: String) -> &mut String {
        let the_struct: &mut Death;
        match which_struct.as_str() {
            "nether" => the_struct = &mut self.nether,
            "over_world" => the_struct = &mut self.over_world,
            _ => panic!("something went wrong at SomeDeath::return_mut_ref...!"),
        }
        match field.as_str() {
            "x_cord" => return &mut the_struct.x_cord,
            "y_cord" => return &mut the_struct.y_cord,
            "z_cord" => return &mut the_struct.z_cord,
            "locale" => return &mut the_struct.locale,
            "owner" => return &mut the_struct.owner,
            "notes" => return &mut the_struct.notes,
            _ => panic!("something went wrong at SomeDeath::return_mut_ref...!"),
        }
    }
}

pub struct Hashy {
    pub key: String,
    pub value: ThirdBreakerVal,
}

impl SecondBreaker {
    pub fn default() -> Self {
        let instance: Self = Default::default();
        instance
    }
}
impl ThirdBreaker {
    pub fn default() -> Self {
        let instance: Self = Default::default();
        instance
    }

    pub fn struct_to_array(self) -> Vec<Hashy> {
        //let vector = vec![
        //    ThirdBreakerVal::Integer(self.x_cord),
        //    ThirdBreakerVal::Integer(self.y_cord),
        //    ThirdBreakerVal::Integer(self.z_cord),
        //    ThirdBreakerVal::Text(self.locale),
        //    ThirdBreakerVal::Text(self.owner),
        //    ThirdBreakerVal::Text(self.notes),
        //];
        let vector = vec![
            Hashy {
                key: "x_cord".to_string(),
                value: ThirdBreakerVal::Integer(self.x_cord),
            },
            Hashy {
                key: "y_cord".to_string(),
                value: ThirdBreakerVal::Integer(self.y_cord),
            },
            Hashy {
                key: "z_cord".to_string(),
                value: ThirdBreakerVal::Integer(self.z_cord),
            },
            Hashy {
                key: "locale".to_string(),
                value: ThirdBreakerVal::Text(self.locale),
            },
            Hashy {
                key: "owner".to_string(),
                value: ThirdBreakerVal::Text(self.owner),
            },
            Hashy {
                key: "notes".to_string(),
                value: ThirdBreakerVal::Text(self.notes),
            },
        ];

        vector
    }
}

impl SecondBreaker {
    pub fn struct_as_array(&self, field: String) -> ThirdBreaker {
        fn new_breaker(ob: &ThirdBreaker) -> ThirdBreaker {
            ThirdBreaker {
                x_cord: ob.x_cord,
                y_cord: ob.y_cord,
                z_cord: ob.z_cord,
                locale: ob.locale.clone(),
                owner: ob.owner.clone(),
                notes: ob.notes.clone(),
            }
        }
        match field.as_str() {
            "Nether" => new_breaker(&self.nether),
            "OverWorld" => new_breaker(&self.over_world),
            _ => panic!("You gave a bad string to be matched by struct_as_array()...!"),
        }
    }
}

pub fn retrieve_user() -> Breaker {
    ureq::get("http://localhost:8123/netherportals")
        .call()
        .unwrap()
        .into_json()
        .unwrap()
}

pub fn handle_portal_submission(submission_values: &SomeDeath) {
    println!("{:?}", submission_values)
}

// pub type Job = Box<dyn FnMut() + Send + 'static>;

// pub enum Message {
//     NewJob(Job),
//     Terminate,
// }

// pub struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Message>,
// }

// impl ThreadPool {
//     pub fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);

//         let (sender, receiver) = mpsc::channel();

//         let receiver = Arc::new(Mutex::new(receiver));

//         let mut workers = Vec::with_capacity(size);

//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }

//         ThreadPool { workers, sender }
//     }

//     pub fn execute<F>(&self, f: F)
//     where
//         F: FnMut() + Send + 'static,
//     {
//         let job = Box::new(f);

//         self.sender.send(Message::NewJob(job)).unwrap();
//     }
// }

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         println!("Sending terminate message to all workers.");

//         for _ in &self.workers {
//             self.sender.send(Message::Terminate).unwrap();
//         }

//         println!("Shutting down all workers.");

//         for worker in &mut self.workers {
//             println!("Shutting down worker {}", worker.id);

//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }

// struct Worker {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>,
// }

// impl Worker {
//     pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
//         let thread = thread::spawn(move || loop {
//             let message = receiver.lock().unwrap().recv().unwrap();

//             match message {
//                 Message::NewJob(mut job) => {
//                     println!("\nWorker {} got a job; executing.", id);
//                     job();
//                 }
//                 Message::Terminate => {
//                     println!("Worker {} was told to terminate.", id);
//                     break;
//                 }
//             }

//             println!("<---> Worker {} finished...!", id);
//         });

//         Worker {
//             id,
//             thread: Some(thread),
//         }
//     }
// }

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                    println!("job {id} finished executing?");
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
pub fn get_all_picture_names() -> HashMap<i32, String> {
    let base_urls: HashMap<i32, String> = ureq::get("http://localhost:1234/allpicturenames")
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    base_urls
}

//use egui_extras::RetainedImage;
//pub struct ImageVector {
//    pub id: i32,
//    pub image_vector: Arc<Mutex<Vec<Option<RetainedImage>>>>,
//}
