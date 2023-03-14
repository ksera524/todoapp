import { useState } from "react";
import { Todo } from "@/type/todo";

type Props = {
  todo: Todo;
  onUpdate: (text: string, completed: boolean) => void;
  onClose: () => void;
};

export const UpdateDialog = ({ todo, onUpdate, onClose }: Props) => {
  const [text, setText] = useState(todo.text);
  const [completed, setCompleted] = useState(todo.completed);

  const handleTextChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setText(e.target.value);
  };

  const handleCompletedChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setCompleted(e.target.checked);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    onUpdate(text, completed);
  };

  const handleCancel = () => {
    onClose();
  };

  return (
    <dialog open={true}>
      <h2>Update Todo</h2>
      <form onSubmit={handleSubmit}>
        <div>
          <label>
            Text:
            <input type="text" value={text} onChange={handleTextChange} />
          </label>
        </div>
        <div>
          <label>
            Completed:
            <input
              type="checkbox"
              checked={completed}
              onChange={handleCompletedChange}
            />
          </label>
        </div>
        <div>
          <button type="submit">Update</button>
          <button type="button" onClick={handleCancel}>
            Cancel
          </button>
        </div>
      </form>
    </dialog>
  );
};
