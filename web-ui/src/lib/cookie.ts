function setCookie(name: string, value: string, expire_at: string | Date | null = null, path: string = "/") {
  let expires = "";
  if (expire_at) {
    const date = new Date(expire_at);
    expires = "; expires=" + date.toUTCString();
  }
  document.cookie = name + "=" + (value || "") + expires + "; path=" + path;
}

const LOGIN_COOKIE_NAME = "session_token";

export function setSessionToken(session_token: string, expire_at: string) {
  setCookie(LOGIN_COOKIE_NAME, session_token, expire_at);
}

export function unsetSessionToken() {
  const expire_at = new Date(0);
  setCookie(LOGIN_COOKIE_NAME, "", expire_at);
}
