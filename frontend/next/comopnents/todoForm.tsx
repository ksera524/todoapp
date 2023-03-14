import { useState } from "react";
import { NewTodoPayload, Todo } from "../type/todo";

type Props = {
  onCreateTodo: (todo: NewTodoPayload) => void;
};

export const TodoForm = ({ onCreateTodo }: Props) => {
  const [text, setText] = useState("");

  const createTodo = async (payload: NewTodoPayload) => {
    try {
      const response = await fetch("http://localhost:3010/todos", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error("Failed to create todo");
      }

      const data: Todo = await response.json();

      onCreateTodo(data);
    } catch (error) {
      console.error(error);
    }
  };

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const newTodo: NewTodoPayload = {
      text,
    };

    createTodo(newTodo);

    setText("");
  };

  return (
    <form onSubmit={handleSubmit}>
      <input
        type="text"
        value={text}
        onChange={(event) => setText(event.target.value)}
      />
      <button type="submit">Add Todo</button>
    </form>
  );
};
