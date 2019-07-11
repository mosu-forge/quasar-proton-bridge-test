#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;
extern crate clap;
extern crate proton;

#[cfg(not(feature = "dev"))]
extern crate tiny_http;

#[cfg(feature = "dev")]
use clap::{Arg, App};

#[cfg(not(feature = "dev"))]
use std::thread;

mod cmd;
mod server;
//mod bridge; // todo

fn main() {
    let debug;
    let content;
    let _matches: clap::ArgMatches;

    #[cfg(not(feature="dev"))]
    {
        thread::spawn(|| {
            proton::command::spawn_relative_command("updater".to_string(), Vec::new(), std::process::Stdio::inherit()).unwrap();
        });
    }

    #[cfg(feature = "dev")]
    {
        let app = App::new("app")
            .version("1.0.0")
            .author("Author")
            .about("About")
            .arg(Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("Loads the specified URL into webview")
                .required(true)
                .takes_value(true)
            );

        _matches = app.get_matches();
        content = web_view::Content::Url(_matches.value_of("url").unwrap());
        debug = true;
    }
    #[cfg(not(feature="dev"))]
    {
        if let Some(available_port) = proton::tcp::get_available_port() {
            let server_url = format!("{}:{}", "127.0.0.1", available_port);
            content = web_view::Content::Url(format!("http://{}", server_url));
            debug = cfg!(debug_assertions);

            thread::spawn(move || {
                let server = tiny_http::Server::http(server_url).unwrap();
                for request in server.incoming_requests() {
                    let mut url = request.url().to_string();
                    if url == "/" {
                        url = "/index.html".to_string();
                    }
                    request.respond(server::asset_response(&url)).unwrap();
                }
            });
        }
        else
        {
            panic!("Could not find an open port");
        }
    }

    let mut webview = web_view::builder()
        .title("MyAppTitle")
        .content(content)
        .size(800, 600) // TODO:Resolution is fixed right now, change this later to be dynamic
        .resizable(true)
        .debug(debug)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            // leave this as is to use the proton API from your JS code
            if !proton::api::handler(_webview, arg)
            {
                use cmd::Cmd::*;
                match serde_json::from_str(arg) {
                    Err(_) => {},
                    Ok(command) => {
                        match command {
                            // definitions for your custom commands from Cmd here
                            Message { data } => {
                                //  your command code
                                println!("message received {}", data);

                                let response = serde_json::json!({
                                    "subtype": "TRANSMIT",
                                    "message": "pong"
                                });

                                _webview.eval(&format!("bridge.receive({})", response.to_string()));
                            },
                            MessagePromise { id, data } => {
                                //  your command code
                                println!("messagePromise received {}", data);

                                if data == "rejectme" {

                                    let response = serde_json::json!({
                                        "subtype": "PROMISE_RENDER",
                                        "id": id,
                                        "status": "REJECT",
                                        "message": "I am programmed to reject this message"
                                    });

                                    _webview.eval(&format!("bridge.receive({})", response.to_string()));
                                } else {

                                    let reversed: String = data
                                        .to_string()
                                        .chars()
                                        .rev()
                                        .collect();

                                    let response = serde_json::json!({
                                        "subtype": "PROMISE_RENDER",
                                        "id": id,
                                        "status": "RESOLVE",
                                        "message": reversed
                                    });

                                    _webview.eval(&format!("bridge.receive({})", response.to_string()));
                                }
                            }
                        }
                    }
                }
            }

            Ok(())
        })
        .build().unwrap();



    // let bridge = bridge::Bridge {
    //     webview: &mut webview
    // };

    // let bridge = bridge::Bridge::new(&mut webview);

    /*
     Original intention is to create a bridge class that acts as an event emitter.
     This would mimic the way that app-extension-electron-security works, but may
     not be the best way to handle this in rust.

     The reason why this was written like the below code in electron is to hide
     from the user the underlying message structure that deals with resolving
     or rejecting promises.

     Potentially something like this (pseudo-code):

     let bridge = bridge::Bridge::new(&webview);

     bridge.add_listener("message", |message| {
         println!("message received {}", message);
         bridge.send("PONG");
     })

     bridge.on("messagePromise", |resolve, reject, message| {
         println!("messagePromise received {}", message);
         if message == "rejectme" {
             reject("I am programmed to reject this message");
         } else {
             resolve(message.to_string().chars().rev().collect());
         }
     })

     bridge.sendPromise("getroute").then(|message| {
         println!("sendPromise was resolved {}", message);
     }).catch(error => {
         println!("sendPromise was rejected {}", error);
     })

     */


    webview.run().unwrap();
}
