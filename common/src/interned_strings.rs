pub enum BuiltinSymbol {
    LastSymbol,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Symbol {
    value: u32,
}

impl Symbol {
    pub fn new(value: u32) -> Symbol {
        Symbol { value }
    }

    pub fn from_string(value: String) -> Symbol {
        todo!()
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}
