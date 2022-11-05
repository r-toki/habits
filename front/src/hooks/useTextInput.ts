import { ChangeEventHandler, useState } from 'react';

export const useTextInput = (initValue = '') => {
  const [value, set] = useState(initValue);
  const onChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    set(e.target.value);
  };
  const reset = (resetValue = '') => set(resetValue);
  return { value, set, onChange, reset, bind: { value, onChange } };
};
