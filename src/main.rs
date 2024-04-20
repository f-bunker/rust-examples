use String;

fn main() {
    test();
    array();
    flip_string();
}

fn test() {
    // let [mut] name [: Typ] = ...
    let test = "Hallo Welt".to_string();

    println!("{}",test);
}

fn array() {
    let array = vec![1,2,3];
    for e in array.clone() {
        println!("{}", e)
    }

    let filtered_array: Vec<i32> = array
        .into_iter()
        .filter(|e| e < &2)
        .collect();
    for e in filtered_array {
        println!("{}", e)
    }
}

struct FlipString {
    inner: String
}

impl FlipString {
    fn flip(&self) -> String {
        self.inner.chars().rev().collect()
    }
}

impl From<String> for FlipString {
    fn from(string: String) -> Self {
        FlipString {
            inner: string
        }
    }
}

fn flip_string() {
    let to_flip: String = "Hallo".into();
    let test: FlipString = to_flip.into();
    println!("{}", test.flip());
}

