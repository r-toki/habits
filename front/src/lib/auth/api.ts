import { tokenStorage } from '../token-storage';
import { axios, axiosForUpdateUserSession } from './axios';
import { AuthUser, CreateUserInput, CreateUserSessionInput, Tokens } from './type';

export const getIndex = () => axios.get<string>('').then(({ data }) => data);

export const getAuthUser = async () => {
  const { data } = await axios.get<AuthUser>('user');
  return data;
};

export const createAuthUser = async ({ name, password }: CreateUserInput) => {
  const { data } = await axios.post<Tokens>('user', { name, password });
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};

export const destroyAuthUser = async () => {
  await axios.delete('user');
  tokenStorage.clear('access_token');
  tokenStorage.clear('refresh_token');
};

export const createAuthUserSession = async ({ name, password }: CreateUserSessionInput) => {
  const { data } = await axios.post<Tokens>('user/session', { name, password });
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};

export const destroyAuthUserSession = async () => {
  await axios.delete('user/session');
  tokenStorage.clear('access_token');
  tokenStorage.clear('refresh_token');
};

export const updateAuthUserSession = async () => {
  const { data } = await axiosForUpdateUserSession.patch<Tokens>('user/session');
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};
