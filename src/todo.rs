use std::fmt;
use super::sql;

#[derive(Debug)]
pub struct Task {
    pub id:i64,
    pub title:String,
    pub done:bool
}

impl fmt::Display for Task {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let mut donestr = "";
        if self.done {
            donestr = "DONE: ";
        }

        write!(f, "{}. {}{}", self.id, donestr, self.title)
    }
}

pub struct Todo {
    pub connection:sqlite::Connection,
}

impl Todo {
    pub fn addtask(&self, title:String) {
        let mut newid_statement = self.connection.prepare(sql::SQL_CURRENTID).unwrap();
        let newid = match newid_statement.next() {
            Ok(_) => {
                newid_statement.read::<i64, _>("id").unwrap()
            }
            Err(_) => {
                0 // we want to start at "1" for user friendly id system
            }
        } + 1;

        let mut statement = self.connection.prepare(sql::SQL_INSERT).unwrap();
        statement.bind((1, newid)).unwrap();
        statement.bind((2, &*title)).unwrap();
        statement.bind((3, 0)).unwrap();
        statement.next().unwrap();
    }

    pub fn gettasks(&self, include_done:bool) -> Vec<Task> {
        let mut tasks:Vec<Task> = vec![];
    
        let mut statement = self.connection.prepare(sql::SQL_GET).unwrap();
        statement.bind((1, include_done as i64)).unwrap();
        while let Ok(sqlite::State::Row) = statement.next() {
            tasks.push(Task {
                id: statement.read::<i64, _>("id").unwrap(),
                title: statement.read::<String, _>("title").unwrap(),
                done: statement.read::<i64, _>("done").unwrap() != 0
            });
        };

        tasks
    }

    pub fn changestatus(&self, id:i64, done:bool) {
        let mut statement = self.connection.prepare(sql::SQL_CHANGESTATUS).unwrap();
        statement.bind((1, done as i64)).unwrap();
        statement.bind((2, id)).unwrap();

        statement.next().unwrap();
    }

    pub fn removetask(&self, id:i64) {
        let mut statement = self.connection.prepare(sql::SQL_DELETE).unwrap();
        statement.bind((1, id)).unwrap();

        statement.next().unwrap();
    }
}