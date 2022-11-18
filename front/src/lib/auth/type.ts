export type AuthUser = {
  id: string;
  name: string;
};

export type Tokens = {
  accessToken: string;
  refreshToken: string;
};

export type CreateAuthUserInput = {
  name: string;
  password: string;
};

export type CreateAuthUserSessionInput = {
  name: string;
  password: string;
};
