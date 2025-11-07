use std::fmt;

pub struct TodoList {
    todos: Vec<TodoListItem>,
    next_id: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TodoListStatus {
    Pending,
    Completed,
    Archived,
    Error { err_code: i8, err_msg: String },
}

pub struct TodoListItem {
    id: u32,
    title: String,
    status: TodoListStatus,
}

impl TodoListItem {
    // Getter methods
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> &TodoListStatus {
        &self.status
    }

    // Setter method for status
    pub fn set_status(&mut self, status: TodoListStatus) {
        self.status = status;
    }
}

impl fmt::Display for TodoListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_text = match self.status {
            TodoListStatus::Pending => "[ ]",
            TodoListStatus::Completed => "[✓]",
            TodoListStatus::Archived => "[X]",
            TodoListStatus::Error { .. } => "[!]",
        };
        write!(f, "{} {} - {}", status_text, self.id, self.title)
    }
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_todo(&mut self, title: String) -> &mut Self {
        self.todos.push(TodoListItem {
            id: self.next_id,
            title,
            status: TodoListStatus::Pending,
        });
        self.next_id += 1;
        self
    }

    pub fn list_todos(&self) -> &Vec<TodoListItem> {
        &self.todos
    }

    pub fn complete_todo(&mut self, index: usize) -> Result<(), String> {
        if index < self.todos.len() {
            self.todos[index].status = TodoListStatus::Completed;
            Ok(())
        } else {
            Err(format!(
                "Index {} out of bounds (max: {})",
                index,
                self.todos.len() - 1
            ))
        }
    }

    pub fn remove_todo(&mut self, index: usize) -> Result<(), String> {
        if index < self.todos.len() {
            self.todos[index].status = TodoListStatus::Archived;
            Ok(())
        } else {
            Err(format!(
                "Index {} out of bounds (max: {})",
                index,
                self.todos.len() - 1
            ))
        }
    }

    pub fn clear(&mut self) {
        self.todos.clear();
    }

    pub fn display_todos(&self) {
        for (index, todo) in self.todos.iter().enumerate() {
            println!("{}. {}", index + 1, todo.title);
        }
    }

    pub fn display_all(&self) {
        for todo in self.todos.iter() {
            println!("{}", todo);
        }
    }

    // Filter methods với chain và match
    pub fn filter_by_status(&self, status: TodoListStatus) -> Vec<&TodoListItem> {
        self.todos
            .iter()
            .filter(|todo| match (&todo.status, &status) {
                (TodoListStatus::Pending, TodoListStatus::Pending) => true,
                (TodoListStatus::Completed, TodoListStatus::Completed) => true,
                (TodoListStatus::Archived, TodoListStatus::Archived) => true,
                (TodoListStatus::Error { .. }, TodoListStatus::Error { .. }) => true,
                _ => false,
            })
            .collect()
    }

    pub fn get_pending_todos(&self) -> Vec<&TodoListItem> {
        self.todos
            .iter()
            .filter(|todo| matches!(todo.status, TodoListStatus::Pending))
            .collect()
    }

    pub fn get_completed_todos(&self) -> Vec<&TodoListItem> {
        self.todos
            .iter()
            .filter(|todo| matches!(todo.status, TodoListStatus::Completed))
            .collect()
    }

    pub fn get_archived_todos(&self) -> Vec<&TodoListItem> {
        self.todos
            .iter()
            .filter(|todo| matches!(todo.status, TodoListStatus::Archived))
            .collect()
    }
}
