import { Container, Navbar, Text } from "@nextui-org/react";
import { useUser } from "@auth0/nextjs-auth0/client";
import { useRouter } from "next/router";

export default function Layout({ children }) {
  const router = useRouter();
  const { user } = useUser();
  return (
    <>
      <Navbar>
        <Navbar.Brand>
          <Text b>Graph Drawing Contest by VDSLab</Text>
        </Navbar.Brand>
        <Navbar.Content>
          <Navbar.Item>{user ? user.sub : ""}</Navbar.Item>
          {user ? (
            <Navbar.Link href={`/api/auth/logout?returnTo=${router.asPath}`}>
              Logout
            </Navbar.Link>
          ) : (
            <Navbar.Link href={`/api/auth/login?returnTo=${router.asPath}`}>
              Login
            </Navbar.Link>
          )}
        </Navbar.Content>
      </Navbar>
      <Container>{children}</Container>
    </>
  );
}
