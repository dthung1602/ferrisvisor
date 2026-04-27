export function setCookie(
  name: string,
  value: string,
  expire_at: string | Date | null = null,
  path: string = "/"
) {
  let expires = "";
  if (expire_at) {
    const date = new Date(expire_at);
    expires = "; expires=" + date.toUTCString();
  }
  document.cookie = name + "=" + (value || "") + expires + "; path=" + path;
}

export function getCookie(name: string): string | null {
  const nameEQ = name + "=";
  const ca = document.cookie.split(";");
  for (let i = 0; i < ca.length; i++) {
    let c = ca[i];
    while (c.charAt(0) == " ") c = c.substring(1, c.length);
    if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length, c.length);
  }
  return null;
}

export function setPersistentCookie(name: string, value: string) {
  // Set to expire in 100 years
  const date = new Date();
  date.setFullYear(date.getFullYear() + 100);
  setCookie(name, value, date);
}

const LOGIN_COOKIE_NAME = "session_token";

export function setSessionToken(session_token: string, expire_at: string) {
  setCookie(LOGIN_COOKIE_NAME, session_token, expire_at);
}

export function unsetSessionToken() {
  const expire_at = new Date(0);
  setCookie(LOGIN_COOKIE_NAME, "", expire_at);
}
