use std::collections::HashMap;

#[derive(Debug)]
pub enum MaybeMulti<M, S> {
	Multi(HashMap<String, M>),
	Single(S),
}
