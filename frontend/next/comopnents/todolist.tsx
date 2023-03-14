import { useState, useEffect } from "react";
import { Todo, NewTodoPayload } from "@/type/todo";
import { TodoForm } from "./todoForm";
import { UpdateButton } from "./UpdateButton";

const TodoPage = () => {
  const [todos, setTodos] = useState<Todo[]>([]);

  useEffect(() => {
    const fetchTodos = async () => {
      const res = await fetch("http://localhost:3010/todos");
      const data = await res.json();
      setTodos(data);
    };

    fetchTodos();
  }, []);

  const createTodo = (todo: Todo) => {
    setTodos([...todos, { id: todo.id, text: todo.text, completed: false }]);
  };

  const updateTodo = (updatedTodo: Todo) => {
    setTodos(
      todos.map((todo) => {
        if (todo.id === updatedTodo.id) {
          return updatedTodo;
        } else {
          return todo;
        }
      })
    );
  };

  const deleteTodo = async (id: number) => {
    await fetch(`http://localhost:3010/todos/${id}`, { method: "DELETE" });
    setTodos(todos.filter((todo) => todo.id !== id));
  };

  return (
    <div>
      <h1>Todos</h1>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Text</th>
            <th>Completed</th>
            <th>Update</th>
            <th>Delete</th>
          </tr>
        </thead>
        <tbody>
          {todos.map((todo) => (
            <tr key={todo.id}>
              <td>{todo.id}</td>
              <td>{todo.text}</td>
              <td>{todo.completed ? "✅" : "❌"}</td>
              <td>
                <UpdateButton todo={todo} onUpdateTodo={updateTodo} />
              </td>
              <td>
                <button onClick={() => deleteTodo(todo.id)}>Delete</button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <TodoForm onCreateTodo={createTodo} />
    </div>
  );
};

export default TodoPage;
