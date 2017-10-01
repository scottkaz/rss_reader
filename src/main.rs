extern crate rss;

use std::fs::File;
use std::io::BufReader;
use rss::Channel;

fn main() {
    let file = File::open("/home/skazimou/releases.xml").unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();

    //println!("Channel: {}", channel.title());
    //println!("Items: {}", channel.items().len());

    for item in channel.items() {
        match item.title() {
            None => { println!("???"); }
            Some(s) => { println!("{}", s); }
        }
        //println!("{}", item.content().unwrap_or_default());
    }
}
