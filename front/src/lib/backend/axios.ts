import Axios from 'axios';

export const axios = Axios.create({
  baseURL: import.meta.env.VITE_APP_BACKEND_URL,
});

axios.interceptors.request.use((config) => {
  const token = window.localStorage.getItem('access_token');
  config.headers = {
    Accept: 'application/json',
    'Content-Type': 'application/json',
    Authorization: token ? `Bearer ${token}` : undefined,
  };
  return config;
});
