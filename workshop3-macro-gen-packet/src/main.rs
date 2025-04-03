use bincode;
use serde_json;

macro_rules! make_packet {
    ($( $field:ident : $type:ty ),* $(,)?) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        struct Packet {
            $( $field: $type ),*
        }

        impl Packet {
            fn as_bytes(&self) -> Vec<u8> {
                bincode::serialize(self).expect("Failed to serialize")
            }
        }
        impl Packet {
            fn from_bytes(bytes: Vec<u8>) -> Self {
                bincode::deserialize(&bytes).expect("Failed to deserialize")
            }
        }
        impl Packet {
            fn to_json(&self) -> String {
                serde_json::to_string(self).expect("Failed to serialize to JSON")
            }
        }
        impl Packet {
            fn from_json(json: &str) -> Self {
                serde_json::from_str(json).expect("Failed to deserialize from JSON")
            }
        }
    }
}

// Use the macro to generate a struct.
make_packet!(name: String, age: u32);

fn main() {
    let packet = Packet {
        name: "Alice".to_string(),
        age: 30,
    };

    let bytes = packet.as_bytes();
    let packet_from_bytes = Packet::from_bytes(bytes);
    let json = packet.to_json();
    let packet_from_json = Packet::from_json(&json);

    println!("as_bytes\n\t{:?} |", packet_from_bytes.as_bytes());
    println!("from_bytes\n\t{:?} |", packet_from_bytes);
    println!("to_json\n\t{:?} |", packet_from_json.to_json());
    println!("from_json\n\t{:?} |", packet_from_json);
}