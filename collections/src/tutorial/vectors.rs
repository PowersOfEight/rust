use std::vec;

#[derive(Debug)]
pub struct VectorStruct {
    example: String,
    num: u64
}

impl VectorStruct {
    pub fn create(example: String, num: u64) -> Self {
        VectorStruct {
            example,
            num
        }
    }

    pub fn get_example(self: &Self) -> &str {
        &self.example[..]
    }

    pub fn get_num(self: &Self) -> u64 {
        self.num
    }
}
pub fn vector_examples() {
    let vec_struct = VectorStruct::create(String::from("Hello, Vectors!"), 8);
    dbg!(&vec_struct);
    println!("Hello, Vectors! {:#?}", vec_struct);
    let n = vec_struct.get_num();
    let words = vec_struct.get_example();
    println!("The words are \"{}\" and the number is {}", words, n);

    let v = vec![1,2,3];
    let mut v = v;
    v.push(5);
    v.push(6);
    v.push(7);

    let v = vec![1,2,3,4,5];

    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(integer) => println!("The third element is {integer}"),
        None => println!("We don't have a third element")
    }

    // let does_not_exist = &v[100];
    // println!("If it doesn't exist: {does_not_exist}");'

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {first}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = v;
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    let mut v = v;
}