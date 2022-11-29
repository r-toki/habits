type Key = 'access_token';

export const tokenStorage = {
  get: (key: Key): string | null => {
    return window.localStorage.getItem(key);
  },
  set: (key: Key, value: string) => {
    window.localStorage.setItem(key, value);
  },
  clear: (key: Key) => {
    window.localStorage.removeItem(key);
  },
};
