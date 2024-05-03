import { useRoutes, Navigate } from "react-router-dom";

import { Layout } from "@/Layout";



export const Router = () => {
  const element = useRoutes([
    {
      path: "/",
      element: <Layout />,
      children: [
        { path: "/", element: <div>{'aaaa'}</div> },
        { path: "*", element: <Navigate to="." /> },
      ],
    },
  ]);

  return <>{element}</>;
};
