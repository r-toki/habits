import Axios, { AxiosError } from 'axios';

import { updateUserSession } from '../auth';
import { tokenStorage } from '../token-storage';

export const axios = Axios.create({
  baseURL: import.meta.env.VITE_APP_BACKEND_URL,
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

axios.interceptors.response.use(undefined, async (err: AxiosError) => {
  const refreshToken = tokenStorage.get('refresh_token');
  if (!err.config || err.response?.status !== 401 || !refreshToken) return Promise.reject(err);

  try {
    await updateUserSession();
    return axios(err.config);
  } catch (err) {
    return Promise.reject(err);
  }
});
