#[derive(Debug)]
struct TodoItem {
    title: String,
    description: String,
    // status: TodoStatus 
}

#[derive(Debug)]
enum TodoStatus {
   Complete,
   Ongoing,
   Canceled
}


pub fn getList() -> [TodoItem, 5] {
    [
        { "Shop list", "Cheese, Ham, Bread, Milk, Pizza" },
        { "Project TODO", "Fix UI, Handle HTTP requests, Authenticate system" },
        { "Gardening tools", "Rake, Shovel, Gloves, Watering can"},
        { "My Passwords", "password123, isTh1sS3fe" },
        { "secret list", "he deals the cards to find the answer, the sacred geometry of change, the hidden law of probable outcome, the numbers lead the dance" }
    ]
}
