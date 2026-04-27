export type ColumnId = "process" | "state" | "pid" | "last_changed" | "actions";

export type ProcessColumn = {
  id: ColumnId;
  label: string;
  visible: boolean;
  locked: boolean;
};
