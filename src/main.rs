// Copyright 2014 Benjamin Elder from
// https://github.com/BenTheElder/slack-rs-demo
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// This is a simple example of using slack-rs.
// You can run it with `cargo run example -- <api_key>`
//

extern crate slack;
use slack::Event;

struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self,
                cli: &mut slack::RtmClient,
                event_result: Result<&Event, slack::Error>,
                raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event_result, raw_json);

        let event = match event_result {
            Ok(event) => event,
            Err(e) => {
                println!("on_event got ERROR: {}", e);
                return;
            }
        };

        match *event {
            Event::Message(ref message) => {
                match message.clone() {
                    slack::Message::Standard { ts, channel: _, user, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                        println!("SCORE: {:?} {:?}", user, text);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack::RtmClient) {
        println!("on_connect");

        let channels = cli.get_channels();

        match channels.iter().position(|channel| {
            println!("Checkng id {}", channel.name);
            channel.name == "general"
        }) {
            None => {},
            Some(i) => {
                let channel_id = &(channels[i].id);

                println!("Got channel ID: {}", channel_id);


                // Do a few things using the api:
                // send a message over the real time api websocket
                let _ = cli.send_message(channel_id, "Hello world! (rtm)");
                // post a message as a user to the web api
                let _ = cli.post_message(channel_id, "hello world! (postMessage)", None);
                // set a channel topic via the web api
                // let _ = cli.set_topic("#general", "bots rule!");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: cargo run slack_example -- <api-key>"),
        x => args[x - 1].clone(),
    };
    let mut handler = MyHandler;
    let mut cli = slack::RtmClient::new(&api_key);
    let r = cli.login_and_run::<MyHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}
