extern crate rss;

use rss::Channel;


fn main() {
    let kernel_rss_url = "https://www.kernel.org/feeds/kdist.xml";
    let channel = Channel::from_url(kernel_rss_url).unwrap();

    for item in channel.items() {
        match item.title() {
            None => { println!("???"); }
            Some(s) => { println!("{}", s); }
        }
    }
}
