import Axios from 'axios';

import { tokenStorage } from '../token-storage';

export const axios = Axios.create({
  baseURL: import.meta.env.VITE_APP_AUTH_URL,
  headers: {
    Accept: 'application/json',
    'Content-Type': 'application/json',
  },
});

axios.interceptors.request.use((config) => {
  if (!config.headers) return config;
  const accessToken = tokenStorage.get('access_token');
  if (accessToken) {
    config.headers['Authorization'] = `Bearer ${accessToken}`;
  }
  return config;
});

export const axiosForUpdateUserSession = Axios.create({
  baseURL: import.meta.env.VITE_APP_AUTH_URL,
  headers: {
    Accept: 'application/json',
    'Content-Type': 'application/json',
  },
});

axiosForUpdateUserSession.interceptors.request.use((config) => {
  if (!config.headers) return config;
  const refreshToken = tokenStorage.get('refresh_token');
  if (refreshToken) {
    config.headers['Authorization'] = `Bearer ${refreshToken}`;
  }
  return config;
});
