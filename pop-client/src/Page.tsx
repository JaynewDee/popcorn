import { usePageState } from "./hooks/usePageState";

const Page = ({ children }: { children: any }) => {
  const Header = ({ txt }: { txt: string }) => <h4>{txt}</h4>;

  const [state] = usePageState();
  return (
    <div className="page-container">
      {Header({ txt: state })}
      {children}
    </div>
  );
};

export default Page;
