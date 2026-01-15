use std::io::{Write, stdout};

use curl::easy::{Easy, List};

fn main() {
    let mut list = List::new();
    list.append("X-aws-ec2-metadata-token-ttl-seconds: 10")
        .unwrap();

    let mut easy = Easy::new();
    easy.url("http://169.254.169.254/latest/api/token").unwrap();
    easy.http_headers(list).unwrap();
    easy.put(true).unwrap();

    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
}
