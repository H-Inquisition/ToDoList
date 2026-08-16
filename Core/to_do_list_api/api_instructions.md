# Instructions

## Commands

### Create Task
Command is used to create a new task instance in the database

Arguments required to be provided:  
&nbsp; task_title: String - a name of the task, can be a task description  
&nbsp; priority: String - has to be convertable to the Priority enum, see enum values  

Returns:  
&nbsp; list_of_tasks: Vec<(i64, Task)> - a list of tuples consisting of the tasks along with their ids

Call:  
&nbsp; http://localhost:insert_port_number/create_task  
&nbsp; "Content-Type: application/json"  
&nbsp; '[task_title, priority]'  

Example:  
&nbsp; http://localhost:3000/create_task  
&nbsp; "Content-Type: application/json"  
&nbsp; '["TaskName", "High"]'  

### Update Task
Command is used to update a preexisting task in the database

Arguments required to be provided:  
&nbsp; task_id: i64 - an assigned id of the task in the database  
&nbsp; task_status: String - has to be convertable to the Status enum, see Status enum values  
&nbsp; task_title: String - a new name of the task  
&nbsp; priority: String - has to be convertable to the Priority enum, see Priority enum values  

Returns:  
&nbsp; list_of_tasks: Vec<(i64, Task)> - a list of tuples consisting of the tasks along with their ids

Call:  
&nbsp; http://localhost:insert_port_number/update_task  
&nbsp; "Content-Type: application/json"  
&nbsp; '[task_id, task_status, task_title, priority]'  

Example:  
&nbsp; http://localhost:3000/update_task  
&nbsp; "Content-Type: application/json"  
&nbsp; '[1, "Done", "NewTaskName", "High"]'  

### Delete Task
Command is used to delete a task from a database

Arguments required to be provided:  
&nbsp; id: i64 - an assigned id of the task in the database  

Returns:  
&nbsp; list_of_tasks: Vec<(i64, Task)> - a list of tuples consisting of the tasks along with their ids  

Call:  
&nbsp; http://localhost:insert_port_number/delete_task  
&nbsp; "Content-Type: application/json"  
&nbsp; 'id'  

Example:  
&nbsp; http://localhost:3000/delete_task  
&nbsp; "Content-Type: application/json"  
&nbsp; '2'  

## Queries

### List of Tasks
Query is used to get a list of tasks from the database

Returns:  
&nbsp; list_of_tasks: Vec<(i64, Task)> - a list of tuples consisting of the tasks along with their ids

Call:  
&nbsp; http://localhost:insert_port_number/list_of_tasks  

Example:  
&nbsp; http://localhost:3000/list_of_tasks
