/// <reference types="vite/client" />
interface ImportMetaEnv {
  readonly VITE_APP_BACKEND_URL: string;
  readonly VITE_APP_AUTH_URL: string;
}
interface ImportMeta {
  readonly env: ImportMetaEnv;
}
