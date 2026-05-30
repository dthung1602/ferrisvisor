const DASHBOARD_COLUMNS = "dashboard_columns";
const SIDEBAR_OPEN = "sidebar_open";
const THEME = "theme";

type StorageKeys = typeof DASHBOARD_COLUMNS | typeof SIDEBAR_OPEN | typeof THEME;

function get(key: StorageKeys, defaultValue: unknown = null) {
  const val = localStorage.getItem(key);
  return val ? JSON.parse(val) : defaultValue;
}

function set(key: StorageKeys, value: unknown) {
  localStorage.setItem(key, JSON.stringify(value));
}

function contain(key: StorageKeys) {
  return localStorage.getItem(key) !== null;
}

function remove(key: StorageKeys) {
  localStorage.removeItem(key);
}

export { get, set, contain, remove, DASHBOARD_COLUMNS, SIDEBAR_OPEN, THEME, type StorageKeys };
