#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
struct Event {
  name: String,
  cat: String,
  ph: String,
  pid: i64,
  ts: i64,
  #[serde(default)]
  dur: i64,
  tid: i64,
  args: HashMap<String,String>
}


fn main() {
    let mut f = File::open("trace.log.child-1").expect("file not found");
    let reader = BufReader::new(f );


    let deserialized: Vec<Event> = serde_json::from_reader(reader).unwrap();

    let mut duration = 0;
    let mut start = 0;
    let mut budget = 0;
    let mut out: Vec<Event> = Vec::new();
    let mut temp: Vec<Event> = Vec::new();

    for e in deserialized.iter() {
        if e.name.contains("Stable") {
            continue;
        }
        if e.name.contains("budget") {
            temp.clear();
            budget = e.dur;
        }
        if e.name.contains("DataCallback") {
            if e.ph == "B" {
                start = e.ts;
            } else if e.ph == "E" {
                if e.ts - start > budget {
                    out.append(&mut temp);
                    out.push(e.clone());
                }
            }
        }
        temp.push(e.clone());
    }
    let serialized = serde_json::to_string(&out).unwrap();
    print!("{}", serialized);
}
