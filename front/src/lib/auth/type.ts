export type AuthUser = {
  uid: string;
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
