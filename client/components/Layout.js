import {
  Button,
  Container,
  Input,
  Navbar,
  Spacer,
  Table,
  Text,
} from "@nextui-org/react";
import { useUser } from "@auth0/nextjs-auth0/client";

export default function Layout({ children }) {
  const { user } = useUser();
  return (
    <>
      <Navbar>
        <Navbar.Brand>
          <Text b>Graph Drawing Contest by VDSLab</Text>
        </Navbar.Brand>
        <Navbar.Content>
          {user ? (
            <>
              <Navbar.Item>{user.sub}</Navbar.Item>
              <Navbar.Link href="/api/auth/logout">Logout</Navbar.Link>
            </>
          ) : (
            <Navbar.Link href="/api/auth/login">Login</Navbar.Link>
          )}
        </Navbar.Content>
      </Navbar>
      <Container>{children}</Container>
    </>
  );
}
