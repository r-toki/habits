const PREFIX = 'my_habits_';

type Key = 'access_token' | 'refresh_token';

export const tokenStorage = {
  get: (key: Key): string | null => {
    return window.localStorage.getItem(PREFIX + key);
  },
  set: (key: Key, value: string) => {
    window.localStorage.setItem(PREFIX + key, value);
  },
  clear: (key: Key) => {
    window.localStorage.removeItem(PREFIX + key);
  },
};
