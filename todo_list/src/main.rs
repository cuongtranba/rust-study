mod todolist;

use todolist::{TodoList, TodoListStatus};

fn main() {
    // Create todo list and add items
    let mut todo_list = TodoList::new();

    todo_list
        .add_todo("Learn Rust basics".to_string())
        .add_todo("Build a todo app".to_string())
        .add_todo("Master modules".to_string())
        .add_todo("Implement features".to_string())
        .add_todo("Write tests".to_string());

    println!("=== ALL TODOS ===");
    todo_list.display_all();

    // Complete some todos
    println!("\n=== COMPLETING SOME TODOS ===");
    let _ = todo_list.complete_todo(0); // Complete "Learn Rust basics"
    let _ = todo_list.complete_todo(2); // Complete "Master modules"

    // Archive one todo
    println!("=== ARCHIVING ONE TODO ===");
    let _ = todo_list.remove_todo(3); // Archive "Implement features"

    println!("\n=== ALL TODOS (UPDATED) ===");
    todo_list.display_all();

    // ============================================
    // DEMO: MATCH with filter status
    // ============================================
    println!("\n=== DEMO: MATCH with Filter Status ===");

    // Method 1: Use match to print each status type
    println!("\n1. Use match to classify:");
    for todo in todo_list.list_todos() {
        match todo.status() {
            TodoListStatus::Pending => {
                println!("  [PENDING] {} - {}", todo.id(), todo.title());
            }
            TodoListStatus::Completed => {
                println!("  [DONE] {} - {}", todo.id(), todo.title());
            }
            TodoListStatus::Archived => {
                println!("  [ARCHIVED] {} - {}", todo.id(), todo.title());
            }
            TodoListStatus::Error { err_code, err_msg } => {
                println!(
                    "  [ERROR {}] {} - {}: {}",
                    err_code,
                    todo.id(),
                    todo.title(),
                    err_msg
                );
            }
        }
    }

    // Method 2: Match with guard (additional condition)
    println!("\n2. Match with guard - only show completed with ID > 2:");
    for todo in todo_list.list_todos() {
        match (todo.status(), todo.id()) {
            (TodoListStatus::Completed, id) if id > 2 => {
                println!("  ✓ {} - {}", id, todo.title());
            }
            _ => {} // Skip other cases
        }
    }

    // ============================================
    // DEMO: CHAIN METHODS on Vector
    // ============================================
    println!("\n=== DEMO: CHAIN METHODS on Vector ===");

    // 1. iter() + filter() + collect()
    println!("\n1. Get pending todos (filter + collect):");
    let pending = todo_list.get_pending_todos();
    pending.iter().for_each(|todo| println!("  {}", todo));

    // 2. iter() + filter() + map() + collect()
    println!("\n2. Get titles of completed todos (filter + map + collect):");
    let completed_titles: Vec<String> = todo_list
        .list_todos()
        .iter()
        .filter(|todo| matches!(todo.status(), TodoListStatus::Completed))
        .map(|todo| format!("✓ {}", todo.title()))
        .collect();

    completed_titles
        .iter()
        .for_each(|title| println!("  {}", title));

    // 3. iter() + filter() + count()
    println!("\n3. Count number of todos by status:");
    let pending_count = todo_list
        .list_todos()
        .iter()
        .filter(|todo| matches!(todo.status(), TodoListStatus::Pending))
        .count();

    let completed_count = todo_list
        .list_todos()
        .iter()
        .filter(|todo| matches!(todo.status(), TodoListStatus::Completed))
        .count();

    println!("  Pending: {}", pending_count);
    println!("  Completed: {}", completed_count);

    // 4. iter() + enumerate() + filter() + for_each()
    println!("\n4. Show index of pending todos:");
    todo_list
        .list_todos()
        .iter()
        .enumerate()
        .filter(|(_, todo)| matches!(todo.status(), TodoListStatus::Pending))
        .for_each(|(index, todo)| {
            println!("  Index {}: {}", index, todo.title());
        });

    // 5. iter() + any() - check if any todo is pending
    println!("\n5. Check if any todo is pending:");
    let has_pending = todo_list
        .list_todos()
        .iter()
        .any(|todo| matches!(todo.status(), TodoListStatus::Pending));
    println!("  Has pending todos: {}", has_pending);

    // 6. iter() + all() - check if all todos are completed
    println!("\n6. Check if all todos are completed:");
    let all_completed = todo_list
        .list_todos()
        .iter()
        .all(|todo| matches!(todo.status(), TodoListStatus::Completed));
    println!("  All completed: {}", all_completed);

    // 7. iter() + find() - find first todo matching condition
    println!("\n7. Find first completed todo:");
    let first_completed = todo_list
        .list_todos()
        .iter()
        .find(|todo| matches!(todo.status(), TodoListStatus::Completed));

    match first_completed {
        Some(todo) => println!("  Found: {}", todo.title()),
        None => println!("  No completed todo found"),
    }

    // 8. iter() + filter() + take() - get first 2 pending
    println!("\n8. Get first 2 pending todos:");
    todo_list
        .list_todos()
        .iter()
        .filter(|todo| matches!(todo.status(), TodoListStatus::Pending))
        .take(2)
        .for_each(|todo| println!("  {}", todo));

    // 9. Match with closure in filter
    println!("\n9. Filter with match in closure:");
    let active_todos: Vec<&str> = todo_list
        .list_todos()
        .iter()
        .filter(|todo| match todo.status() {
            TodoListStatus::Pending | TodoListStatus::Completed => true,
            _ => false,
        })
        .map(|todo| todo.title())
        .collect();

    println!("  Active todos (Pending or Completed):");
    active_todos
        .iter()
        .for_each(|title| println!("    - {}", title));

    // 10. Use matches! macro - shorthand for match
    println!("\n10. Use matches! macro:");
    todo_list
        .list_todos()
        .iter()
        .filter(|todo| {
            matches!(
                todo.status(),
                TodoListStatus::Pending | TodoListStatus::Completed
            )
        })
        .for_each(|todo| println!("  {}", todo));

    // ============================================
    // DEMO: Advanced Match Patterns
    // ============================================
    println!("\n=== DEMO: Advanced Match Patterns ===");

    // 1. Match with tuple
    println!("\n1. Match with (status, id):");
    for todo in todo_list.list_todos() {
        match (todo.status(), todo.id()) {
            (TodoListStatus::Pending, 1) => println!("  First pending: {}", todo.title()),
            (TodoListStatus::Completed, id) if id % 2 == 0 => {
                println!("  Completed with even ID: {}", todo.title());
            }
            (TodoListStatus::Archived, _) => println!("  Archived: {}", todo.title()),
            _ => {} // Catch all
        }
    }

    // 2. Match with reference
    println!("\n2. Match with reference:");
    let first_todo = todo_list.list_todos().get(0);
    match first_todo {
        Some(todo) => {
            println!(
                "  First todo: {} (status: {:?})",
                todo.title(),
                todo.status()
            );
        }
        None => println!("  No todos"),
    }

    // ============================================
    // SUMMARY
    // ============================================
    println!("\n=== SUMMARY ===");
    println!("Total todos: {}", todo_list.list_todos().len());
    println!("Pending: {}", todo_list.get_pending_todos().len());
    println!("Completed: {}", todo_list.get_completed_todos().len());
    println!("Archived: {}", todo_list.get_archived_todos().len());
}
