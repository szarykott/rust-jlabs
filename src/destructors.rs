pub fn entrypoint() {
    let _a = MyStruct::new("struct1");
    let _b = MyStruct::new("struct2");
}

pub struct MyStruct {
    val: String
}

impl MyStruct {
    pub fn new(val: &str) -> Self {
        MyStruct {
            val: val.to_string()
        }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("dropping {0}", self.val)
    }
}
