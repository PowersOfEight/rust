pub use std::collections::HashMap;
pub use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Debug)]
pub struct Company {
    pub department_employees: HashMap<String, Vec<String>>, 
}

impl Company {
    pub fn new() -> Self {
        let map: HashMap<String, Vec<String>> = HashMap::new();
        Company {
            department_employees: map,
        }
    }
    pub fn add(self: &mut Self, department: &str, employee: &str) -> () {
        match self.department_employees.entry(department.to_string()) {
            Occupied(mut occ) => {
                occ.get_mut().push(employee.to_string());
            },
            Vacant(vac) => {
                let employee_list = vec![employee.to_string()];
                vac.insert(employee_list);
            }
        }
    }
}