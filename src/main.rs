#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
const LOGIN_FORM: &'static [&'static str] =
    &["username", "password", "firstname", "lastname", "role"];
const REAL_LOGIN_FORM: &'static [&'static str] = &["username", "password"];
//ehttp::Response;
//use egui_extras::Table;
//use hello_world::FormatedStructString;
//use hello_world::MoreDeath;
//use hello_world::SecondBreaker;
//use hello_world::ThirdBreaker;
//use std::io::Read;
//use std::string;
use borkcraftclient::{get_all_picture_names, Breaker, SomeDeath, ThreadPool};
use eframe::egui::{self}; //plot::Value, Direction, Response};
use egui_extras::image::RetainedImage;
use serde::Serialize;
use ureq::request;
//use hello_world::Breaker;
//use hello_world::SomeDeath;
//use hello_world::ThreadPool;
use core::panic;
use poll_promise::Promise;
//use std::sync::Arc;
//use std::sync::Mutex;
//use std::thread;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread, time, vec,
};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

enum ADirection {
    Up,
    Down,
    Both,
}

struct Image {
    image_name: String,
    image: Option<RetainedImage>,
}

#[derive(Default, Serialize, Debug)]
//struct KeyAndLoginForm {
//    key: String,
//    login_form:
//}
struct KeyAndLoginForm {
    key: String,
    profile: HashMap<String, String>,
}

#[derive(Default, Serialize)]
struct LoginForm {
    id: i32,
    username: String, // email
    password: String, // password
    firstname: String,
    lastname: String,
    role: String,
}

impl LoginForm {
    fn default() -> Self {
        let instance: Self = Default::default();
        instance
    }
}

type ImageVector = Vec<Option<Image>>;
struct MyApp {
    breaker: Arc<Mutex<Breaker>>, //String
    radio: String,
    modify: bool,
    breaker_mod_values: SomeDeath,
    submit_portals: bool,
    promise: Option<Promise<Option<RetainedImage>>>,
    all_images: Arc<Mutex<HashMap<i32, Option<ImageVector>>>>,
    some_image: Option<RetainedImage>,
    image_index: usize,
    finished: bool,
    the_key: i32,
    the_vec_key: usize,
    a_direction: ADirection,
    modal: String,
    picture_index: usize,
    login_form: LoginForm,
    login_form_hash: HashMap<String, String>,
    bin: LoginForm,
    key: Arc<Mutex<String>>,
    alreadyLoggedIn: Arc<Mutex<bool>>,
    timetime: TimeTime,
    threaded_time: Option<thread::JoinHandle<()>>,
    just_once: String,
    path: Option<String>,
    dropped_files: Vec<egui::DroppedFile>,
    all_paths: Vec<String>,
    path_image: Option<RetainedImage>,
}

impl Default for MyApp {
    fn default() -> Self {
        let my_breaker: Breaker = borkcraftclient::Breaker {
            all_nether_portals: vec![borkcraftclient::SecondBreaker {
                id: 0,
                nether: borkcraftclient::ThirdBreaker {
                    x_cord: 0,
                    y_cord: 0,
                    z_cord: 0,
                    locale: String::new(),
                    owner: String::new(),
                    notes: String::new(),
                },
                over_world: borkcraftclient::ThirdBreaker {
                    x_cord: 0,
                    y_cord: 0,
                    z_cord: 0,
                    locale: String::new(),
                    owner: String::new(),
                    notes: String::new(),
                },
            }],
        };

        //let stringy = Arc::new(Mutex::new(String::from("string")));
        let stringy = Arc::new(Mutex::new(my_breaker));

        let arc_clone = Arc::clone(&stringy);
        let radio = &*arc_clone.lock().unwrap().all_nether_portals[0]
            .id
            .to_string();
        thread::spawn(move || {
            //let res = reqwest::blocking::get("http://localhost:8123/netherportals"); //get("http://localhost:7070/testing");
            //let mut body = String::new();

            //res.unwrap().read_to_string(&mut body).unwrap();
            let body = borkcraftclient::retrieve_user();

            *arc_clone.lock().unwrap() = body;
            //println!("===: {}", *arc_clone.lock().unwrap());
            // sender.send(body).unwrap();
        });

        Self {
            radio: radio.to_string(),
            breaker: stringy, //Arc::clone(&stringy),A
            modify: false,
            breaker_mod_values: SomeDeath::default(),
            submit_portals: false,
            promise: Option::None,
            all_images: Arc::new(Mutex::new(HashMap::new())),
            image_index: 0,
            finished: false,
            some_image: None,
            the_key: 0,
            the_vec_key: 0,
            a_direction: ADirection::Both,
            modal: "Login".to_string(),
            picture_index: 0,
            login_form: LoginForm::default(),
            bin: LoginForm::default(),
            login_form_hash: default_login_form_hash(),
            key: Arc::new(Mutex::new("None".to_string())),
            alreadyLoggedIn: Arc::new(Mutex::new(false)),
            timetime: TimeTime::default(),
            threaded_time: None,
            just_once: "999".to_string(),
            path: None,
            dropped_files: Vec::new(),
            all_paths: Vec::new(),
            path_image: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Choose a Modal...!")
                .selected_text(self.modal.clone())
                .show_ui(ui, |ui| {
                    let options = ["Nether Portals", "Login"];
                    for option in options {
                        ui.selectable_value(&mut self.modal, option.to_string(), option);
                    }
                });

            //============================================
            //ui.label(&*self.breaker.lock().unwrap().to_formated_string()); // network request; threaded version

            use borkcraftclient::ThirdBreakerVal;
            use egui_extras::{Size, TableBuilder};

            match self.modal.as_str() {
                "Nether Portals" => {
                    ui.push_id(10, |ui| {
                        egui::Grid::new(10).show(ui, |ui| {
                            let mut ids = Vec::new();
                            for i in &*self.breaker.lock().unwrap().all_nether_portals {
                                ids.push(i.id.to_string());
                            }
                            egui::ComboBox::from_label("Choose an ID!")
                                .selected_text(self.radio.clone())
                                .show_ui(ui, |ui| {
                                    for i in ids {
                                        if ui
                                            .selectable_value(&mut self.radio, i.clone(), i.clone())
                                            .clicked()
                                        {
                                            self.finished = false;
                                            println!("We did it bois. Lets head back to base...!")
                                        };
                                    }
                                });

                            if ui.button("Modify!").clicked() {
                                println!("{}", self.modify);
                                self.modify = !self.modify;
                            }
                            if self.modify {
                                if ui.button("Submit Changes!").clicked() {
                                    self.breaker_mod_values.id = self.radio.clone();
                                    borkcraftclient::handle_portal_submission(
                                        &self.breaker_mod_values,
                                    );
                                    self.submit_portals = !self.submit_portals;
                                }
                            }
                            ui.end_row();
                        });
                    });

                    fn find_id_by_radio(
                        id: i32,
                        breaker: &Arc<Mutex<Breaker>>,
                    ) -> Result<usize, usize> {
                        //let lenn = breaker.lock().unwrap().all_nether_portals.len().to_string().parse::<i32>().unwrap();
                        let lenn = breaker.lock().unwrap().all_nether_portals.len();

                        for i in 0..lenn {
                            if breaker.lock().unwrap().all_nether_portals[i].id == id {
                                return Ok(i);
                            }
                        }
                        Err(0)
                    }

                    TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Max))
                        .column(Size::initial(90.0).at_least(30.0))
                        .column(Size::remainder().at_least(60.0))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("Row");
                            });
                            header.col(|ui| {
                                ui.heading("Content");
                            });
                        })
                        .body(|mut body| {
                            let headers = ["Nether", "OverWorld"];
                            let result =
                                find_id_by_radio(self.radio.parse::<i32>().unwrap(), &self.breaker);
                            let indexy: usize;
                            match result {
                                Ok(n) => indexy = n,
                                Err(e) => indexy = e,
                            };
                            let third_breaker_array1 =
                                &*self.breaker.lock().unwrap().all_nether_portals[indexy]
                                    .struct_as_array(headers[0].to_string())
                                    .struct_to_array();
                            let third_breaker_array2 =
                                &*self.breaker.lock().unwrap().all_nether_portals[indexy]
                                    .struct_as_array(headers[1].to_string())
                                    .struct_to_array();

                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.heading(headers[0]);
                                });
                            });
                            for j in 0..6 {
                                // Nether Vals
                                // Nether And OverWorld Values that are displayed!
                                let the_breaker = third_breaker_array1;
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&the_breaker[j].key);
                                    });
                                    row.col(|ui| {
                                        match &the_breaker[j].value {
                                            ThirdBreakerVal::Text(val) => ui.label(val),
                                            ThirdBreakerVal::Integer(val) => {
                                                ui.label(val.to_string())
                                            }
                                        };
                                        if self.modify {
                                            let the_ref = self.breaker_mod_values.return_mut_ref(
                                                the_breaker[j].key.clone(),
                                                "nether".to_string(),
                                            );
                                            let response =
                                                ui.add(egui::TextEdit::singleline(the_ref));
                                            if response.changed() {
                                                println!(
                                                    "{}",
                                                    self.breaker_mod_values.nether.notes
                                                );
                                            }
                                        }
                                    });
                                });
                            }
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.heading("||");
                                    ui.heading("||");
                                    ui.heading("||");
                                });
                            });
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.heading("||");
                                    ui.heading("||");
                                    ui.heading("||");
                                });
                            });

                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.heading("||");
                                    ui.heading("||");
                                    ui.heading("||");
                                });
                            });
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.heading(headers[1]);
                                });
                            });
                            for j in 0..6 {
                                // OverWorld Vals
                                let the_breaker = third_breaker_array2;
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&the_breaker[j].key);
                                    });
                                    row.col(|ui| {
                                        match &the_breaker[j].value {
                                            ThirdBreakerVal::Text(val) => ui.label(val),
                                            ThirdBreakerVal::Integer(val) => {
                                                ui.label(val.to_string())
                                            }
                                        };
                                        if self.modify {
                                            let the_ref = self.breaker_mod_values.return_mut_ref(
                                                the_breaker[j].key.clone(),
                                                "over_world".to_string(),
                                            );
                                            let response =
                                                ui.add(egui::TextEdit::singleline(the_ref));
                                            if response.changed() {
                                                println!(
                                                    "{}",
                                                    self.breaker_mod_values.nether.notes
                                                );
                                            }
                                        }
                                    });
                                });
                            }
                        });

                    if ui.button("rand img").clicked() {
                        println!("How many times did you execute?");
                        let ctx = ctx.clone();
                        let (sender, promise) = Promise::new();
                        //let request = ehttp::Request::get("https://picsum.photos/id/237/200/300");
                        let request = ehttp::Request::get(
                            "http://localhost:1234/specificpicture?name=SpiderCowboy.png",
                        );
                        ehttp::fetch(request, move |response| match response {
                            Ok(response) => {
                                ctx.request_repaint();
                                let resource = from_response(response); //response.map(|response| from_response(response));
                                sender.send(resource);
                            }
                            Err(error) => panic!("error at using match in a dumb way, {}", error),
                        });
                        self.promise = Some(promise);
                    }

                    if let Some(promise) = &self.promise {
                        if let Some(result) = promise.ready() {
                            match result {
                                Some(image) => {
                                    //let image = &self.image;
                                    //if let Some(image) = image {
                                    let mut size = image.size_vec2();
                                    size *= (ui.available_width() / size.x).min(1.0);
                                    image.show_size(ui, size);
                                    //}
                                }
                                None => {
                                    panic!("An error trying to show the image...!")
                                }
                            }
                        } else {
                            ui.spinner();
                        }
                    }
                    if ui.button("rand img array").clicked() {
                        let currently_selected_id: i32 = self.radio.clone().parse().unwrap();
                        let clone_of_all_images = Arc::clone(&self.all_images);
                        thread::spawn(move || {
                            let hash = get_all_picture_names();
                            let hash_len = hash.len();
                            let (sender, receiver): (
                                Sender<(Option<RetainedImage>, String)>,
                                Receiver<(Option<RetainedImage>, String)>,
                            ) = mpsc::channel();
                            let pool = ThreadPool::new(hash_len);

                            for (_, value) in hash {
                                let tx = sender.clone();
                                pool.execute(|| get_retained_image_threads(tx, value));
                            }

                            let mut final_image_vector: ImageVector = Vec::new();
                            for index in 0..hash_len {
                                if let Ok((image, key)) = receiver.recv() {
                                    final_image_vector.insert(
                                        index,
                                        Some(Image {
                                            image_name: key,
                                            image: image,
                                        }),
                                    )
                                } else {
                                    panic! {"something"}
                                }
                            }
                            clone_of_all_images
                                .lock()
                                .unwrap()
                                .insert(currently_selected_id, Some(final_image_vector));
                            drop(sender);
                            drop(receiver);

                            println!("END OF THREAD BOI");
                        });
                    }

                    if self.finished {
                        display_nether_portal_images(self, ui);
                        // if let Some(vec_op) =
                        //     self.all_images.lock().unwrap().get(&self.the_key).unwrap()
                        // {
                        //     if let Some(vecy) = vec_op.get(self.the_vec_key) {
                        //         if let Some(an_image) = vecy {
                        //             if let Some(another_image) = &an_image.image {
                        //                 let mut size = another_image.size_vec2();
                        //                 size *= (ui.available_width() / size.x).min(1.0);
                        //                 another_image.show_size(ui, size);
                        //             }
                        //         }
                        //     }
                        // }
                    }

                    if self.finished {
                        let len = self
                            .all_images
                            .lock()
                            .unwrap()
                            .get(&self.the_key)
                            .unwrap()
                            .as_ref()
                            .unwrap()
                            .len()
                            - 1;
                        if self.the_vec_key == len {
                            // if max
                            self.a_direction = ADirection::Down;
                        }
                        if self.the_vec_key == 0 {
                            // if min
                            self.a_direction = ADirection::Up;
                        }
                        if self.the_vec_key < len && self.the_vec_key > 0 {
                            // if between min and max
                            self.a_direction = ADirection::Both;
                        }

                        match self.a_direction {
                            ADirection::Up => {
                                if ui.button("Next Meme :)").clicked() {
                                    self.the_vec_key = self.the_vec_key + 1;
                                    self.a_direction = ADirection::Both;
                                }
                            }
                            ADirection::Down => {
                                if ui.button("Previous Meme (:").clicked() {
                                    self.the_vec_key = self.the_vec_key - 1;
                                    self.a_direction = ADirection::Both;
                                }
                            }
                            ADirection::Both => {
                                if ui.button("Next Meme :)").clicked() {
                                    self.the_vec_key = self.the_vec_key + 1;
                                }
                                if ui.button("Previous Meme (:").clicked() {
                                    self.the_vec_key = self.the_vec_key - 1;
                                }
                            }
                        }
                    }

                    // Current Death
                    fn display_nether_portal_images(some_self: &mut MyApp, ui: &mut egui::Ui) {
                        //let image = some_self
                        //    .all_images
                        //    .lock()
                        //    .unwrap()
                        //    .get(&some_self.radio.parse::<i32>().unwrap())
                        //    .unwrap()
                        //    .as_ref()
                        //    .unwrap()
                        //    .get(some_self.the_vec_key)
                        //    .unwrap()
                        //    .as_ref()
                        //    .unwrap()
                        //    .image
                        //    .as_ref()
                        //    .unwrap();
                        if let Some(image) = some_self
                            .all_images
                            .lock()
                            .unwrap()
                            .get(&some_self.radio.parse::<i32>().unwrap())
                        {
                            if let Some(image) = image {
                                let image = image
                                    .get(some_self.the_vec_key)
                                    .unwrap()
                                    .as_ref()
                                    .unwrap()
                                    .image
                                    .as_ref()
                                    .unwrap();
                                let mut size = image.size_vec2();
                                size *= (ui.available_width() / size.x).min(1.0);
                                image.show_size(ui, size);
                            }
                            //let mut size = image.size_vec2();
                            //size *= (ui.available_width() / size.x).min(1.0);
                            //image.show_size(ui, size);
                            //some_self.finished = !some_self.finished;
                            // if let Some(vector) = some_self
                            //     .all_images
                            //     .lock()
                            //     .unwrap()
                            //     .get(&some_self.radio.parse::<i32>().unwrap())
                            // {
                            //     let image = vector.as_ref().unwrap();
                            //     let image = &image
                            //         .get(some_self.picture_index)
                            //         .unwrap()
                            //         .as_ref()
                            //         .unwrap()
                            //         .image
                            //         .as_ref()
                            //         .unwrap();
                            // }
                        }
                    }

                    if ui.button("State of Mutex").clicked() {
                        if !self.finished {
                            self.finished = !self.finished;
                            self.the_key = self.radio.parse::<i32>().unwrap();
                            //display_nether_portal_images(self, ui);
                            // for key in &*self.all_images.lock().unwrap() {
                            //     println!("key me please...!");
                            //     println!("key: {}", key.0);
                            //     if let Some(value) = key.1 {
                            //         //for something in value {
                            //         for something in 0..value.len() {
                            //             if let Some(image) =
                            //                 &value[something].as_ref().unwrap().image
                            //             {
                            //                 println!("we found a value");
                            //                 let my_image = image;
                            //                 let mut size = my_image.size_vec2();
                            //                 size *= (ui.available_width() / size.x).min(1.0);
                            //                 my_image.show_size(ui, size);

                            //                 self.the_key = key.0.clone();
                            //                 self.the_vec_key = something;
                            //                 self.finished = !self.finished;
                            //                 break;
                            //             }
                            //         }
                            //     }
                            // }
                        }
                    }
                }
                "Signup" => {
                    signup(ui, self);
                }
                "Login" => {
                    if *self.alreadyLoggedIn.lock().unwrap() {
                        ui.label(format!("key: {}", self.key.lock().unwrap()));
                    } else {
                        login(ui, self);
                    }
                }
                _ => panic!("Something went wrong...! in modal...!"),
            }
            //                            egui::ComboBox::from_label("Choose an ID!")
            //                                .selected_text(self.radio.clone())
            //                                .show_ui(ui, |ui| {
            //                                    for i in ids {
            //                                        ui.selectable_value(&mut self.radio, i.clone(), i.clone());
            //                                    }
            //                                });

            //if ui.button("Change Modal").clicked() {}

            get_client_submited_images_button(self, ui);
            get_client_submited_images(&self.path_image, ui);
        });
    }
}

fn shove_image_into_all_images(some_self: &mut MyApp) {
    println!("SHOVE THE RADIO IMAGE: {}", &some_self.radio);
    if let Some(vecy) = some_self
        .all_images
        .lock()
        .unwrap()
        .get_mut(&some_self.radio.parse::<i32>().unwrap())
    {
        if let Some(value) = vecy {
            value.push(Some(Image {
                image_name: "breaker".to_string(),
                image: Some(get_retained_image_from_file(get_file_from_path(
                    some_self.path.clone().unwrap(),
                ))),
            }))
        }
    }
}

fn get_client_submited_images_button(some_self: &mut MyApp, ui: &mut egui::Ui) {
    if ui.button("Open file").clicked() {
        if let Some(a_path) = rfd::FileDialog::new().pick_file() {
            some_self.path = Some(a_path.display().to_string());
        }

        if let Some(path) = &some_self.path {
            ui.horizontal(|ui| {
                ui.label("Picked File:");
                ui.monospace(path);

                let file = get_file_from_path(path.clone());
                let retained_image = get_retained_image_from_file(file);
                some_self.path_image = Some(retained_image);
            });
        }
        shove_image_into_all_images(some_self);
    }
}

fn get_client_submited_images(image: &Option<RetainedImage>, ui: &mut egui::Ui) {
    if let Some(retained_image) = image {
        let mut size = retained_image.size_vec2();
        size *= (ui.available_width() / size.x).min(1.0);
        retained_image.show_size(ui, size);
    }
}

fn get_retained_image_threads(sender: Sender<(Option<RetainedImage>, String)>, base_url: String) {
    let base_url = format!(
        "{}{}",
        String::from("http://localhost:1234/specificpicture?name="),
        base_url
    );
    println!("baseurl: ----------------------> {}", base_url);

    let response = ureq::get(&base_url).call().ok().unwrap();
    let url = response.get_url().to_string();
    let mut bytes: Vec<u8> = Vec::new();
    response.into_reader().read_to_end(&mut bytes).ok().unwrap();

    let image = egui_extras::image::RetainedImage::from_image_bytes(url, &bytes).ok();

    sender.send((image, base_url)).unwrap();
}

fn get_retained_image(base_url: &String) -> Option<RetainedImage> {
    let response = ureq::get(base_url).call().ok().unwrap();
    let url = response.get_url().to_string();
    let mut bytes: Vec<u8> = Vec::new();
    response.into_reader().read_to_end(&mut bytes).ok().unwrap();
    let image = egui_extras::image::RetainedImage::from_image_bytes(url, &bytes).ok();

    image
}

fn from_response(response: ehttp::Response) -> Option<RetainedImage> {
    let image =
        egui_extras::image::RetainedImage::from_image_bytes(&response.url, &response.bytes).ok();
    image
}

fn signup(ui: &mut egui::Ui, some_self: &mut MyApp) {
    egui::Grid::new(9).show(ui, |ui| {
        for item in LOGIN_FORM.iter() {
            ui.label(item.clone());
            ui.add(egui::TextEdit::singleline(
                some_self.login_form_hash.get_mut(item.clone()).unwrap(),
            ));
            ui.end_row();
        }

        //if ui.button("Get Native Key").clicked() {
        //    let request = ehttp::Request::get("http://localhost:8123/nativesignup");
        //    ehttp::fetch(request, move |response| {
        //        //println!("{:?}", response.unwrap().text());
        //        println!("{:?}", response.unwrap().status_text)
        //    });
        //}

        if ui.button("Real Submit").clicked() {
            let json_login_form = json!(some_self.login_form_hash);

            let (sender, promise) = Promise::new();
            let request = ehttp::Request::post(
                "http://localhost:8123/nativesignup",
                serde_json::to_vec(&json_login_form).unwrap(),
            );
            ehttp::fetch(request, |response| {
                //some_self.key = response.unwrap().text().unwrap().to_string();
                let key = response.unwrap().text().unwrap().to_string();
                println!("{}", &key);
                sender.send(key)
            });

            if let Some(result) = promise.ready() {
                *some_self.key.lock().unwrap() = result.to_string()
            }
            some_self.login_form_hash = default_login_form_hash();
        }
    });
}

#[derive(Deserialize, Serialize, Debug)]
struct Key {
    key: String,
}

#[derive(Deserialize, Serialize, Debug)]
//struct SessionTime {
//    key: String,
//    time: HashMap<String, HashMap<String, String>>,
//}
struct SessionTime {
    key: String,
    time: TimeTime,
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct TimeTime {
    hour: String,
    minute: String,
    second: String,
}

impl TimeTime {
    pub fn default() -> Self {
        let instance: Self = Default::default();
        instance
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginFormHashWithKey {
    key: String,
    profile: HashMap<String, String>,
}
fn login(ui: &mut egui::Ui, some_self: &mut MyApp) {
    egui::Grid::new(9).show(ui, |ui| {
        for item in REAL_LOGIN_FORM.iter() {
            ui.label(item.clone());
            ui.add(egui::TextEdit::singleline(
                some_self.login_form_hash.get_mut(item.clone()).unwrap(),
            ));
            ui.end_row();
        }

        println!(
            "\n\n\nIN BIGG WORDS: {}\n\n\n",
            *some_self.alreadyLoggedIn.lock().unwrap()
        );
        if ui.button("Real Login").clicked() {
            //let json_login_form = json!(some_self.login_form_hash);
            println!(
                "the key we are looking for: {}",
                some_self.key.lock().unwrap().clone()
            );
            let login_form_hash = some_self.login_form_hash.clone();
            println!("{:?}", &login_form_hash);
            //let (sender, promise) = Promise::new();
            //let request = ehttp::Request::post(
            //    "http://localhost:8123/nativelogin",
            //    serde_json::to_vec(&key_and_login_form).unwrap(),
            //);
            // ehttp::fetch(request, |response| {
            //     let result = response.clone().unwrap().status_text;
            //     println!("{:?}", result);
            //     let key = response.unwrap().text().unwrap().to_string();
            //     println!("the key: {}", &key);
            //     sender.send(key)
            // });
            // if let Some(result) = promise.ready() {
            //     some_self.key = result.to_string()
            // }
            let gar = LoginFormHashWithKey {
                key: some_self.key.lock().unwrap().clone(),
                profile: some_self.login_form_hash.clone(),
            };
            let response = ureq::post("http://localhost:8123/nativelogin")
                .send_json(&gar)
                .unwrap();
            let status = response.status();
            if status.to_string() == String::from("202") {
                println!("\n\nim proud...!\n\n");
                let session_time: SessionTime = response.into_json().unwrap();
                println!("The remaining session time: \n{:?}\n\n", session_time);
                if session_time.message == "Your already logged in!".to_string() {
                    *some_self.alreadyLoggedIn.lock().unwrap() = true;
                } else {
                    *some_self.alreadyLoggedIn.lock().unwrap() = false;
                }
                if session_time.key != "" {
                    *some_self.key.lock().unwrap() = session_time.key;
                }
            } else {
                println!("|{}| |{}|", status.to_string(), String::from("202"));

                println!(
                    "\nHERE IS THE STATUS OF THE REQUEST:, status |{}|\n",
                    status
                );
                let key: SessionTime = response.into_json().unwrap();
                //println!("LJSD:LKJF:LSKDJF:LSDKJ: ----> {:?}", key);

                if key.message == "Your already logged in!".to_string() {
                    *some_self.alreadyLoggedIn.lock().unwrap() = true;
                } else {
                    *some_self.alreadyLoggedIn.lock().unwrap() = false;
                }

                *some_self.key.lock().unwrap() = key.key;
            }
            //let response = ureq::get(&base_url).call().ok().unwrap();
            //let url = response.get_url().to_string();
            //let mut bytes: Vec<u8> = Vec::new();
            //response.into_reader().read_to_end(&mut bytes).ok().unwrap();

            if some_self.just_once == "999".to_string() {
                some_self.just_once = "0".to_string();
                let am = Arc::clone(&some_self.key);
                let already_am = Arc::clone(&some_self.alreadyLoggedIn);
                some_self.threaded_time = Some(thread::spawn(move || loop {
                    println!("SOME AMAMAMAMAMAAMAMAMAMAM, |{}|", am.lock().unwrap());
                    let key = Key {
                        key: am.lock().unwrap().clone(),
                    };
                    let result_response =
                        ureq::post("http://localhost:8123/sessiontimeleft").send_json(&key);
                    match result_response {
                        Ok(response) => {
                            println!("match res, {}", response.status_text());
                            if response.status() == 202 {
                                let timetime: TimeTime = response.into_json().unwrap();
                                println!("\ntimtime: |{:?}|\n", timetime);
                                let one = "1".to_string();
                                if timetime.hour < one
                                    && timetime.minute < one
                                    && timetime.second < one
                                {
                                    *already_am.lock().unwrap() = false;
                                }
                            }
                        }
                        Err(_) => {
                            println!("funny, {} | ", am.lock().unwrap().clone());
                        }
                    }

                    thread::sleep(time::Duration::from_secs(3));
                    println!("And the cycle of life continues!!!!!!!!!!!!!!!!!!!!!!!!!!!!")
                }));
                //some_self.threaded_time(move || loop {
                //    let key = Key {
                //        key: am.lock().unwrap().clone(),
                //    };
                //    let response = ureq::post("http://localhost:8123/sessiontimeleft")
                //        .send_json(&key)
                //        .unwrap();
                //});
            }
        }

        //        if ui.button("Open file").clicked() {
        //            if let Some(a_path) = rfd::FileDialog::new().pick_file() {
        //                some_self.path = Some(a_path.display().to_string());
        //            }
        //        }
        //        if let Some(path) = &some_self.path {
        //            ui.horizontal(|ui| {
        //                ui.label("Picked File:");
        //                ui.monospace(path);
        //
        //                let file = get_file_from_path(path.clone());
        //                let retained_image = get_retained_image_from_file(file);
        //                some_self.path_image = Some(retained_image);
        //            });
        //        }
        //        if let Some(retained_image) = &some_self.path_image {
        //            let mut size = retained_image.size_vec2();
        //            size *= (ui.available_width() / size.x).min(1.0);
        //            retained_image.show_size(ui, size);
        //        }
    });
}

fn default_login_form_hash() -> HashMap<String, String> {
    let mut the_login_form_hash: HashMap<String, String> = HashMap::default();
    for item in LOGIN_FORM.iter() {
        the_login_form_hash.insert(item.to_string(), "".to_string());
    }

    the_login_form_hash
}

fn get_file_from_path(path: String) -> File {
    let file = match File::open(&path) {
        Err(why) => panic!("Could not open file...: {}: {}", path, why),
        Ok(file) => file,
    };

    file
}

fn get_retained_image_from_file(mut file: File) -> RetainedImage {
    let mut bytes: Vec<u8> = Vec::new();
    //file.read(&mut bytes).unwrap();
    file.read_to_end(&mut bytes).unwrap();
    println!("change");

    egui_extras::image::RetainedImage::from_image_bytes("name.png", &bytes).unwrap()
}
