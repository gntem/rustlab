use std::{time::Duration, vec};

struct Temperature {
    value: f32,
    epoch: u64,
}

impl Temperature {
    fn new(value: f32, epoch: u64) -> Self {
        Temperature { value, epoch }
    }

    fn as_str(&self) -> String {
        format!("{}|{}", self.value, self.epoch)
    }

}  

impl Into<zmq::Message> for Temperature {
    fn into(self) -> zmq::Message {
        let v = self.as_str();
        let mut msg = zmq::Message::with_size(v.len());
        msg.copy_from_slice(v.as_bytes());
        msg
    }
}

// Service that receives temperature data
async fn service() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();

    let mut recordings: Vec<Temperature> = Vec::new();

    loop {
        responder.recv(&mut msg, 0).unwrap();
        
        let s = msg.as_str().unwrap();
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 2 {
            panic!("Invalid message format");
        }
        let value = parts[0].parse::<f32>().unwrap();
        let epoch = parts[1].parse::<u64>().unwrap();

        let temperature = Temperature::new(value, epoch);
        recordings.push(temperature);
        let len = recordings.len();
        if len % 10 == 0 {
            // average temperature
            let sum: f32 = recordings.iter().map(|t| t.value).sum();
            let avg = sum / len as f32;
            println!("average temperature: {}", avg);
        }
        responder.send("OK", 0).unwrap();
    }
}

// Dummy client that sends temperature data to the service every second
fn client() {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    println!("connecting to server...");
    assert!(requester.connect("tcp://localhost:5555").is_ok());
    print!("connected to server\n");
    loop {
        let rand_temp = rand::random::<f32>() * 100.0;
        let temp_str = Temperature::new(
            rand_temp,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        ).as_str();
        let t = temp_str.as_bytes();

        requester.send(t, 0).unwrap();
        let _reply = requester.recv_msg(0).unwrap();
        std::thread::sleep(Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(service());
    tokio::task::spawn_blocking(|| client());
}
