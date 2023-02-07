import { usePageState } from "./hooks/usePageState";

const Page = ({ children }: { children: any }) => {
  const Header = ({ txt }: { txt: string }) => <h4>{txt}</h4>;

  const pageState = usePageState();
  return (
    <div className="page-container">
      {Header({ txt: pageState })}
      {children}
    </div>
  );
};

export default Page;
