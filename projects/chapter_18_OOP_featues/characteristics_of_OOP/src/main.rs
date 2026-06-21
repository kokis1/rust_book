
// the fields in this struct are private
// the only way to interact with this struct is with its public API
pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {

    // initialising and interacting with the AverageColletion object
    let mut avg_collection = AverageCollection {list: vec![], average: 0.0};
    avg_collection.add(3);
    println!("Average: {}", avg_collection.average());
    avg_collection.add(4);
    println!("Average: {}", avg_collection.average());
    avg_collection.add(1);
    println!("Average: {}", avg_collection.average());
    avg_collection.add(2);
    println!("Average: {}", avg_collection.average());

    

}
