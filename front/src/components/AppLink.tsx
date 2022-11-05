import { Link as ChakraLink, LinkProps as ChakraLinkProps } from '@chakra-ui/react';
import { Link as RouterLink, LinkProps as RouterLinkProps } from 'react-router-dom';

export const AppLink = (props: RouterLinkProps & ChakraLinkProps) => {
  return <ChakraLink as={RouterLink} {...props} />;
};
