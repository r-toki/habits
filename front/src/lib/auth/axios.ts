import Axios from 'axios';

import { AxiosHeaders } from '../axios-headers';
import { tokenStorage } from '../token-storage';

export const axios = Axios.create({
  baseURL: import.meta.env.VITE_APP_AUTH_URL,
});

axios.interceptors.request.use((config) => {
  config.headers = AxiosHeaders(tokenStorage.get('access_token'));
  return config;
});

export const axiosForUpdateUserSession = Axios.create({
  baseURL: import.meta.env.VITE_APP_AUTH_URL,
});

axiosForUpdateUserSession.interceptors.request.use((config) => {
  config.headers = AxiosHeaders(tokenStorage.get('refresh_token'));
  return config;
});
