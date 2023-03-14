import { useState } from "react";
import { Todo } from "@/type/todo";
import { UpdateDialog } from "./updateDialog";

type Props = {
  todo: Todo;
  onUpdateTodo: (updatedTodo: Todo) => void;
};

export const UpdateButton = ({ todo, onUpdateTodo }: Props) => {
  const [isOpen, setIsOpen] = useState(false);

  const openDialog = () => setIsOpen(true);
  const closeDialog = () => setIsOpen(false);

  const handleUpdate = async (text: string, completed: boolean) => {
    const res = await fetch(`http://localhost:3010/todos/${todo.id}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ text, completed }),
    });
    const updatedTodo = await res.json();
    onUpdateTodo(updatedTodo);
    closeDialog();
  };

  return (
    <>
      <button onClick={openDialog}>Update</button>
      {isOpen && (
        <UpdateDialog
          todo={todo}
          onUpdate={handleUpdate}
          onClose={closeDialog}
        />
      )}
    </>
  );
};
