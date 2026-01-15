use curl::easy::{Easy, List};

fn main() {
    let mut put_response = Vec::new();

    let mut list = List::new();
    list.append("X-aws-ec2-metadata-token-ttl-seconds: 10")
        .unwrap();

    let mut easy = Easy::new();
    easy.url("http://169.254.169.254/latest/api/token").unwrap();
    easy.http_headers(list).unwrap();
    easy.put(true).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|buf| {
                put_response.extend_from_slice(buf);
                Ok(buf.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    let token = String::from_utf8(put_response).unwrap();
    println!("put_response: {:#?}", token);

    easy.url("http://169.254.169.254/latest/meta-data").unwrap();
    list = List::new();
    list.append(&format!("X-aws-ec2-metadata-token: {}", token))
        .unwrap();
    easy.http_headers(list).unwrap();
    easy.put(false).unwrap();

    let mut toplevel_metadata_response = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|buf| {
                toplevel_metadata_response.extend_from_slice(buf);
                Ok(buf.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    println!();
    println!("{}", String::from_utf8(toplevel_metadata_response).unwrap());
}
