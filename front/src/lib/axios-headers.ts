export const AxiosHeaders = (token: string | null) => ({
  Accept: 'application/json',
  'Content-Type': 'application/json',
  Authorization: token ? `Bearer ${token}` : undefined,
});
