use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value<'a>(&'a mut self, arg: u32) -> &'a u32 {
		let calculation = &self.calculation;
		self.value.entry(arg).or_insert_with(|| (calculation)(arg))
    }
}
