import { tokenStorage } from '../token-storage';
import { axios, axiosForUpdateUserSession } from './axios';
import { CreateUserSessionInput, Tokens } from './type';

export const getIndex = () => axios.get<string>('').then(({ data }) => data);

export const destroyUser = async () => {
  await axios.delete('user');
  tokenStorage.clear('access_token');
  tokenStorage.clear('refresh_token');
};

export const createUserSession = async ({ name, password }: CreateUserSessionInput) => {
  const { data } = await axios.post<Tokens>('user/session', { name, password });
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};

export const destroyUserSession = async () => {
  await axios.delete('user/session');
  tokenStorage.clear('access_token');
  tokenStorage.clear('refresh_token');
};

export const updateUserSession = async () => {
  const { data } = await axiosForUpdateUserSession.patch<Tokens>('user/session');
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};
