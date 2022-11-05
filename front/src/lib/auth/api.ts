import { tokenStorage } from '../token-storage';
import { axios } from './axios';
import { CreateUserInput, CreateUserSessionInput, Tokens } from './type';

export const createUser = async ({ name, password }: CreateUserInput) => {
  const { data } = await axios.post<Tokens>('user', { name, password });
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};

export const destroyUser = async () => {
  await axios.delete('user');
};

export const createUserSession = async ({ name, password }: CreateUserSessionInput) => {
  const { data } = await axios.post<Tokens>('user/session', { name, password });
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};

export const destroyUserSession = async () => {
  await axios.delete('user/session');
};

export const updateUserSession = async () => {
  const { data } = await axios.patch<Tokens>('user/sessions');
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};
