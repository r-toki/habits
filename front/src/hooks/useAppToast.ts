import { useToast } from '@chakra-ui/react';

export const useAppToast = () => {
  const toast = useToast({ position: 'top-right', isClosable: true });
  return toast;
};
