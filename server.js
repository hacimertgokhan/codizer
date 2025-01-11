// Example server.js with promizer tags

// Endpoint for user authentication
//promizer(type='POST', format='raw-json', body=['id:number', 'title:string'])
function authenticateUser(req, res) {
    const { id, title } = req.body;
    // Authentication logic here...
    res.send({ message: `User ${title} authenticated` });
}

// Endpoint for fetching a to-do item by ID
//promizer(type='GET', format='json', body=['id:number'])
function getTodoItem(req, res) {
    const { id } = req.params;
    // Fetch todo item logic here...
    res.send({ id, task: "Buy groceries" });
}

// Endpoint for creating a new to-do item
//promizer(type='POST', format='raw-json', body=['task:string'])
function createTodoItem(req, res) {
    const { task } = req.body;
    // Create new todo item logic here...
    res.send({ message: `New task created: ${task}` });
}

// Endpoint for deleting a to-do item by ID
//promizer(type='DELETE', format='json', body=['id:number'])
function deleteTodoItem(req, res) {
    const { id } = req.params;
    // Delete todo item logic here...
    res.send({ message: `Task with ID ${id} deleted` });
}

module.exports = {
    authenticateUser,
    getTodoItem,
    createTodoItem,
    deleteTodoItem
};
