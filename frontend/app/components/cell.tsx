export interface CellProps {
  children?: React.ReactNode;
}

export function Cell({ children }: CellProps) {
  return (
    <div className="flex items-center justify-center border-2 border-dashed rounded-lg w-24 h-24 border-white/30">
      {children}
    </div>
  );
}
