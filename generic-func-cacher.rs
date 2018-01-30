use std::hash::Hash;
use std::collections::HashMap;

struct Cacher<T, V>
    where T: Fn(V) -> V,
		  V: Eq + Hash + Copy
{
    calculation: T,
    value: HashMap<V, V>,
}

impl<T, V: Eq> Cacher<T, V>
    where T: Fn(V) -> V,
		  V: Eq + Hash + Copy
{
    fn new(calculation: T) -> Cacher<T, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value<'a>(&'a mut self, arg: V) -> &'a V {
		let calculation = &self.calculation;
		self.value.entry(arg).or_insert_with(|| (calculation)(arg))
    }
}
