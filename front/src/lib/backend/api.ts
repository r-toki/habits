import { Tokens } from '../auth';
import { tokenStorage } from '../token-storage';
import { axios } from './axios';
import { CreateUserInput } from './type';

export const createUser = async ({ name, password }: CreateUserInput) => {
  const { data } = await axios.post<Tokens>('users', { name, password });
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};
