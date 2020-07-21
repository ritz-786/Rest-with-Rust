use crate::db;
use crate::error_handler::CustomError;
use crate::schema::teachers;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "teachers"]
pub struct Teacher {
    pub first_name: String,
    pub last_name: String,
    pub designation: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Teachers {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub designation: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

impl Teacher {
    fn from(teacher: Teacher) -> Teacher {
        Teacher {
            first_name: teacher.first_name,
            last_name: teacher.last_name,
            designation: teacher.designation,
            department: teacher.department,
            salary: teacher.salary,
            age: teacher.age,
        }
    }
}

impl Teachers {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let teachers = teachers::table.load::<Teachers>(&conn)?;
        Ok(teachers)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let teacher = teachers::table.filter(teachers::id.eq(id)).first(&conn)?;
        Ok(teacher)
    }

    pub fn find_by_department(department: String) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let teacher = teachers::table
            .filter(teachers::department.eq(department))
            .load::<Teachers>(&conn)?;
        Ok(teacher)
    }

    pub fn create(teacher: Teacher) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let teacher = Teacher::from(teacher);
        let teacher = diesel::insert_into(teachers::table)
            .values(teacher)
            .get_result(&conn)?;
        Ok(teacher)
    }

    pub fn update(id: i32, teacher: Teacher) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let teacher = diesel::update(teachers::table)
            .filter(teachers::id.eq(id))
            .set(teacher)
            .get_result(&conn)?;
        Ok(teacher)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(teachers::table)
            .filter(teachers::id.eq(id))
            .execute(&conn)?;
        Ok(res)
    }
}
