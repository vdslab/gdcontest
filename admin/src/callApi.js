export async function callApi(path, args = {}) {
  const user = import.meta.env.VITE_API_USER;
  const password = import.meta.env.VITE_API_PASSWORD;
  const options = { ...args };
  if (!options.headers) {
    options.headers = {};
  }
  options.headers.Authorization = `Basic ${btoa(`${user}:${password}`)}`;
  const url = `${import.meta.env.VITE_API_ORIGIN}/admin${path}`;
  const request = await fetch(url, options);
  return request.json();
}
