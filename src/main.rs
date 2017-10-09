extern crate rss;

use rss::Channel;

const KERNEL_RSS_URL: &str = "https://www.kernel.org/feeds/kdist.xml";

fn main() {
    let channel = Channel::from_url(KERNEL_RSS_URL).unwrap();

    for item in channel.items() {
        match item.title() {
            None => { println!("???"); }
            Some(s) => { println!("{}", s); }
        }
    }
}
