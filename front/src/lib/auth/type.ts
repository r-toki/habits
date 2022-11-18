export type AuthUser = {
  uid: string;
};

export type Tokens = {
  accessToken: string;
  refreshToken: string;
};

export type CreateUserInput = {
  name: string;
  password: string;
};

export type CreateUserSessionInput = {
  name: string;
  password: string;
};
