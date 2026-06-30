import { useDroppable } from "@dnd-kit/core";

export interface DroppableRowProps {
  id: string;
  children: React.ReactNode;
}

export const DroppableRow: React.FC<DroppableRowProps> = ({ id, children }) => {
  const { setNodeRef, isOver } = useDroppable({
    id,
    data: { type: "row" },
  });

  return (
    <div
      ref={setNodeRef}
      className={`w-full ${isOver ? "bg-blue-100/30" : ""}`}
    >
      {children}
    </div>
  );
};
