pub fn run() {
    let result = namehelpers::get_full_name("Alice", "Bob");
    println!("{result}");
    println!("{}", result);
}

pub mod namehelpers {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
        return full_name;
    }
    pub fn print() {
        println!("Print!");
    }
}

pub mod database {
    pub fn _find_all() {
        println!("FindAll!");
    }
}

mod private_fns {
    pub fn _private_fn() {
        println!("PrivateFn!");
    }
}

#[allow(dead_code)]
fn test_func() {}
