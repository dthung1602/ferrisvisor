const DASHBOARD_COLUMNS = "dashboard_columns";
const SIDEBAR_OPEN = "sidebar_open";
const THEME = "theme";
const TIMEZONE = "timezone";
const REFRESH_RATE = "refresh_rate";

type StorageKeys =
  | typeof DASHBOARD_COLUMNS
  | typeof SIDEBAR_OPEN
  | typeof THEME
  | typeof TIMEZONE
  | typeof REFRESH_RATE;

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

export { get, set, contain, remove, DASHBOARD_COLUMNS, SIDEBAR_OPEN, THEME, TIMEZONE, REFRESH_RATE, type StorageKeys };
